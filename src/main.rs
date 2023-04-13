#![feature(is_terminal)]

use clap::Parser;
use std::fmt;
use std::io::{self, BufWriter, IsTerminal, Write};
use std::path::PathBuf;

mod geometry;

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

    #[arg(short, long,
        value_name = "ANGLE",
        default_value_t = 30.0)]
    /// vertical field of view in degrees
    fov: f64,

    #[arg(short = 'l', long, 
        value_name = "LENGTH", 
        default_value_t = 3.0)]
    /// focal length
    focal_length: f64,

    #[arg(short, long,
        value_name = "RADIUS",
        default_value_t = 0.0)]
    /// aperture for depth of field
    aperture: f64,

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

fn main() -> Result<(), RsTraceError> {
    let args = Args::parse();
    let (width, height) = (args.geometry[0], args.geometry[1]);

    let stdout = io::stdout();

    if stdout.is_terminal() {
        return Err(RsTraceError::OutError);
    }

    let mut handle = BufWriter::new(stdout);
    let mut encoder = png::Encoder::new(handle, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    let mut buf = vec![Pixel { r: 0, g: 0, b: 0 }; (width * height) as usize];
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            buf[idx] = Pixel {
                r: ((255 * y) / (height - 1)) as u8,
                g: ((255 * x) / (width - 1)) as u8,
                b: 127,
            };
        }
    }
    let bytes = unsafe { as_u8_slice(&buf) };
    writer.write_image_data(bytes)?;
    Ok(())
}

unsafe fn as_u8_slice<T: Sized>(p: &[T]) -> &[u8] {
    std::slice::from_raw_parts(
        p.as_ptr() as *const u8,
        std::mem::size_of::<T>() * p.len(),
    )
}
