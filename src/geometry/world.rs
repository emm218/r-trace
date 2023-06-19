use super::vec3::{cross, dot, vec3, Vec3};
use super::{HitInfo, Hittable, Ray};
use std::io::BufRead;
use std::num::ParseFloatError;
use std::str::FromStr;

pub struct Camera {
    horizontal: Vec3<f32>,
    vertical: Vec3<f32>,
    upper_left: Vec3<f32>,
    eye: Vec3<f32>,
}

impl Camera {
    pub fn new(
        eye: Vec3<f32>,
        look_at: Vec3<f32>,
        up: Vec3<f32>,
        fov: f32,
        aspect_ratio: f32,
        focal_length: Option<f32>,
    ) -> Camera {
        let v_look = match focal_length {
            None => look_at - eye,
            Some(fl) => (look_at - eye).normalize() * fl,
        };
        let fov_rads = fov * std::f32::consts::PI / 180.0;
        let vh = (fov_rads / 2.0).tan().abs() * v_look.magnitude() * 2.0;
        let vw = vh * aspect_ratio;

        let vertical = (up - dot(up, v_look) / dot(v_look, v_look) * v_look)
            .normalize()
            * vh;
        let horizontal = -cross(vertical, v_look).normalize() * vw;

        let upper_left = v_look - 0.5 * vertical - 0.5 * horizontal;

        Camera {
            eye,
            vertical,
            horizontal,
            upper_left,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> super::Ray {
        super::Ray {
            origin: self.eye,
            direction: self.upper_left
                + u * self.horizontal
                + (1.0 - v) * self.vertical,
        }
    }
}

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
    pub camera: Camera,
}

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {
        let mut best_hi: Option<HitInfo> = None;
        for b in &self.objects {
            if let Some(new_hi) = match best_hi {
                None => (*b).hit(r, t_min, t_max),
                Some(ref hi) => (*b).hit(r, t_min, hi.t),
            } {
                best_hi = Some(new_hi);
            }
        }
        best_hi
    }
}

#[derive(Debug)]
pub enum WorldError {
    IOError(std::io::Error),
    ParseError(&'static str),
}

impl std::error::Error for WorldError {}

impl std::fmt::Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IOError(e) => e.fmt(f),
            Self::ParseError(s) => write!(f, "{}", s),
        }
    }
}

impl From<std::io::Error> for WorldError {
    fn from(e: std::io::Error) -> WorldError {
        Self::IOError(e)
    }
}

impl From<ParseFloatError> for WorldError {
    fn from(_: ParseFloatError) -> WorldError {
        Self::ParseError("expected f32")
    }
}

pub fn load_world(
    input: &mut dyn BufRead,
    aspect_ratio: f32,
    focal_length: Option<f32>,
    fov: Option<f32>,
) -> Result<World, WorldError> {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    let mut lines = input.lines();
    let mut camera: Option<Camera> = None;
    while let Some(line) = lines.next() {
        let line = line?;
        let line = match line.chars().position(|c| c == '#') {
            Some(n) => &line[0..n],
            None => &line,
        };
        let mut words = line.split_whitespace();
        match words.next() {
            None => continue,
            Some("s") => {
                let sphere = parse_sphere(&mut words)?;
                objects.push(Box::new(sphere));
            }
            Some("p") => {
                let plane = parse_plane(&mut words)?;
                objects.push(Box::new(plane));
            }
            Some("c") => match camera {
                None => {
                    camera = Some(parse_camera(
                        &mut words,
                        aspect_ratio,
                        fov,
                        focal_length,
                    )?)
                }
                _ => {
                    return Err(WorldError::ParseError(
                        "Multiple camera definitions",
                    ))
                }
            },
            _ => return Err(WorldError::ParseError("Unexpected symbol")),
        }
    }
    let fov = fov.unwrap_or(30.0);
    let mut camera = camera.unwrap_or(Camera::new(
        Vec3::<f32>::ZERO,
        Vec3::<f32>::K * 3.0,
        Vec3::<f32>::J,
        fov,
        aspect_ratio,
        focal_length,
    ));

    Ok(World { objects, camera })
}

fn consume<T: FromStr>(
    iter: &mut dyn Iterator<Item = &'_ str>,
) -> Result<T, <T as FromStr>::Err> {
    match iter.next() {
        Some(s) => s.parse::<T>(),
        None => "".parse::<T>(),
    }
}

fn parse_sphere(
    words: &mut dyn Iterator<Item = &'_ str>,
) -> Result<super::Sphere, WorldError> {
    let center = vec3!(
        consume::<f32>(words)?,
        consume::<f32>(words)?,
        consume::<f32>(words)?
    );
    let radius = consume::<f32>(words)?;
    Ok(super::Sphere { center, radius })
}

fn parse_plane(
    words: &mut dyn Iterator<Item = &'_ str>,
) -> Result<super::Plane, WorldError> {
    let point = vec3!(
        consume::<f32>(words)?,
        consume::<f32>(words)?,
        consume::<f32>(words)?
    );
    let normal = vec3!(
        consume::<f32>(words)?,
        consume::<f32>(words)?,
        consume::<f32>(words)?
    );
    match words.next() {
        None => Ok(super::Plane { point, normal }),
        _ => Err(WorldError::ParseError("unexpected symbol")),
    }
}

fn parse_camera(
    words: &mut dyn Iterator<Item = &'_ str>,
    aspect_ratio: f32,
) -> Result<Camera, WorldError> {
    let eye = vec3!(
        consume::<f32>(words)?,
        consume::<f32>(words)?,
        consume::<f32>(words)?
    );
    let look_at = vec3!(
        consume::<f32>(words)?,
        consume::<f32>(words)?,
        consume::<f32>(words)?
    );
    let fov = consume::<f32>(words)?;
    let focal_length = consume::<f32>(words)?;
    Ok(Camera::new(eye, look_at, Vec3::J, aspect_ratio, fov, focal_length);
}
