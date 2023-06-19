#![feature(is_terminal)]

use clap::Parser;
use indicatif::ProgressIterator;
use rand::random;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, IsTerminal};
use std::path::PathBuf;

mod color;
mod geometry;

use color::Color;
use geometry::{
    vec3::{dot, Vec3},
    world::load_world,
    Hittable, Plane, Sphere,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, num_args = 2, value_names = ["WIDTH", "HEIGHT"], default_values_t = [640, 360])]
    /// the size of the output image
    geometry: Vec<u32>,

    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..), default_value_t = 30)]
    /// the number of samples per pixel
    samples: u32,

    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..), default_value_t = 2)]
    /// the max number of bounces per ray
    bounces: u32,

    #[arg(short, long, value_name = "ANGLE")]
    /// vertical field of view in degrees
    fov: Option<f32>,

    #[arg(short = 'l', long, value_name = "LENGTH")]
    /// focal length
    focal_length: Option<f32>,

    #[arg(short, long, value_name = "RADIUS", default_value_t = 0.0)]
    /// aperture for depth of field
    aperture: f32,

    file: Option<PathBuf>,
}

enum RsTraceError {
    OutError,
    EncodingError(png::EncodingError),
    IoError(std::io::Error),
    WorldError(geometry::world::WorldError),
}

impl std::error::Error for RsTraceError {}

impl fmt::Debug for RsTraceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for RsTraceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OutError => {
                write!(f, "Please redirect stdout to a file or pipe")
            }
            Self::EncodingError(e) => e.fmt(f),
            Self::IoError(e) => e.fmt(f),
            Self::WorldError(e) => e.fmt(f),
        }
    }
}

impl From<png::EncodingError> for RsTraceError {
    fn from(e: png::EncodingError) -> RsTraceError {
        RsTraceError::EncodingError(e)
    }
}

impl From<std::io::Error> for RsTraceError {
    fn from(e: std::io::Error) -> RsTraceError {
        RsTraceError::IoError(e)
    }
}

impl From<geometry::world::WorldError> for RsTraceError {
    fn from(e: geometry::world::WorldError) -> RsTraceError {
        RsTraceError::WorldError(e)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl From<Color> for Pixel {
    fn from(c: color::Color) -> Pixel {
        Pixel {
            r: (255.0 * c.r.sqrt()) as u8,
            g: (255.0 * c.g.sqrt()) as u8,
            b: (255.0 * c.b.sqrt()) as u8,
        }
    }
}

const EPSILON: f32 = 0.00001;

fn ray_color(r: &geometry::Ray, h: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::BLACK;
    }
    if let Some(hi) = h.hit(r, EPSILON, f32::INFINITY) {
        let mut direction = Vec3::<f32>::random_unit_ball();
        if dot(hi.normal, direction) < 0.0 {
            direction = -direction;
        }
        let new_r = geometry::Ray {
            origin: hi.at,
            direction,
        };
        return dot(hi.normal, direction) * ray_color(&new_r, h, depth - 1);
    }
    let unit_dir = r.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    color::blend(Color::LIGHT_BLUE, Color::WHITE, t)
}

fn main() -> Result<(), RsTraceError> {
    let args = Args::parse();
    let (width, height) = (args.geometry[0], args.geometry[1]);

    let stdout = io::stdout();

    if stdout.is_terminal() {
        return Err(RsTraceError::OutError);
    }

    let aspect_ratio = width as f32 / height as f32;
    let mut reader: Box<dyn BufRead> = match args.file {
        Some(path) => {
            let file = File::open(path)?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let world =
        load_world(&mut reader, aspect_ratio, args.focal_length, args.fov)?;

    let handle = BufWriter::new(stdout);
    let mut encoder = png::Encoder::new(handle, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    let mut buf = vec![Pixel { r: 0, g: 0, b: 0 }; (width * height) as usize];

    let mut r: geometry::Ray;
    for y in (0..height).progress() {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let mut color = Color::BLACK;
            for _ in 0..args.samples {
                r = world.camera.ray(
                    (x as f32 + random::<f32>()) / width as f32,
                    (y as f32 + random::<f32>()) / height as f32,
                );
                color = color + ray_color(&r, &world, args.bounces + 1).into();
            }
            buf[idx] = (color / args.samples as f32).into();
        }
    }
    let bytes = unsafe { as_u8_slice(&buf) };
    writer.write_image_data(bytes)?;
    Ok(())
}

// this is safe as long as T is packed and has alignment 1, which is the case
// for us since we're using it for pixels
unsafe fn as_u8_slice<T: Sized>(p: &[T]) -> &[u8] {
    std::slice::from_raw_parts(
        p.as_ptr() as *const u8,
        std::mem::size_of::<T>() * p.len(),
    )
}
