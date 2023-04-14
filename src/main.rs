#![feature(is_terminal)]

use clap::Parser;
use std::fmt;
use std::io::{self, BufWriter, IsTerminal};
use std::path::PathBuf;

mod color;
mod geometry;

use geometry::vec3::Vec3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, num_args = 2,
        value_names = ["WIDTH", "HEIGHT"],
        default_values_t = [640, 360])]
    /// the size of the output image
    geometry: Vec<u32>,

    #[arg(short, long, 
        value_parser = clap::value_parser!(u32).range(1..),
        default_value_t = 30)]
    /// the number of samples per pixel
    samples: u32,

    #[arg(short, long,
        value_parser = clap::value_parser!(u32).range(1..),
        default_value_t = 2)]
    /// the max number of bounces per ray
    bounces: u32,

    #[arg(short, long, value_name = "ANGLE", default_value_t = 30.0)]
    /// vertical field of view in degrees
    fov: f32,

    #[arg(short = 'l', long, value_name = "LENGTH", default_value_t = 3.0)]
    /// focal length
    focal_length: f32,

    #[arg(short, long, value_name = "RADIUS", default_value_t = 0.0)]
    /// aperture for depth of field
    aperture: f32,

    file: Option<PathBuf>,
}

enum RsTraceError {
    OutError,
    EncodingError(png::EncodingError),
    IoError(std::io::Error),
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

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl From<color::Color> for Pixel {
    fn from(c: color::Color) -> Pixel {
        Pixel {
            r: (255.0 * c.r) as u8,
            g: (255.0 * c.g) as u8,
            b: (255.0 * c.b) as u8,
        }
    }
}

const WHITE: color::Color = color::Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};
const LIGHT_BLUE: color::Color = color::Color {
    r: 0.5,
    g: 0.7,
    b: 1.0,
};

fn ray_color(r: geometry::Ray) -> color::Color {
    let t = 0.5 * (r.direction.y + 1.0);
    color::blend(WHITE, LIGHT_BLUE, t)
}

fn main() -> Result<(), RsTraceError> {
    let args = Args::parse();
    let (width, height) = (args.geometry[0], args.geometry[1]);

    let stdout = io::stdout();

    if stdout.is_terminal() {
        return Err(RsTraceError::OutError);
    }

    let handle = BufWriter::new(stdout);
    let mut encoder = png::Encoder::new(handle, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    let mut buf = vec![Pixel { r: 0, g: 0, b: 0 }; (width * height) as usize];

    let aspect_ratio = width as f32 / height as f32;
    let vh = 2.0;
    let vw = 2.0 * aspect_ratio;
    let lower_left = vec3!(-vw * 0.5, -vh * 0.5, args.focal_length);
    let origin = vec3!(0.0, 0.0, 0.0);
    let mut r: geometry::Ray;
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let target = lower_left
                + vec3!(x as f32 / width as f32, y as f32 / height as f32, 0.0);
            r = geometry::Ray {
                origin,
                direction: target - origin,
            };
            buf[idx] = ray_color(r).into();
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

