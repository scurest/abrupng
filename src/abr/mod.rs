extern crate byteorder;
mod abr1;
mod abr6;
mod err;
mod util;

pub use self::err::{OpenError, BrushError};
use self::byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Seek};

enum Decoder<R> {
    Abr1(abr1::Decoder<R>),
    Abr6(abr6::Decoder<R>),
}

/// An image brush.
#[derive(Debug)]
pub struct ImageBrush {
    /// Image width.
    pub width: u32,
    /// Image height.
    pub height: u32,
    /// Bit-depth (always 8, currently).
    pub depth: u16,
    /// Row-major vector of width×height image samples.
    pub data: Vec<u8>,
}

/// An iterator over an ABR's image brushes.
pub struct Brushes<R>(Decoder<R>);

/// Gets an iterator over the image brushes in an ABR file in `rdr`.
pub fn open<R: Read + Seek>(mut rdr: R) -> Result<Brushes<R>, OpenError> {
    let version = rdr.read_u16::<BigEndian>()?;
    let subversion = rdr.read_u16::<BigEndian>()?;

    let abr1_like =
        version == 1 || version == 2;
    let abr6_like =
        (version == 6 || version == 10) && (subversion == 1 || subversion == 2);

    Ok(Brushes(
        if abr1_like {
            Decoder::Abr1(abr1::open(rdr, version, subversion)?)
        } else if abr6_like {
            Decoder::Abr6(abr6::open(rdr, version, subversion)?)
        } else {
            return Err(OpenError::UnsupportedVersion { version, subversion });
        }
    ))
}

impl<R: Read + Seek> Iterator for Brushes<R> {
    type Item = Result<ImageBrush, BrushError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Decoder::Abr6(ref mut dec) => abr6::next_brush(dec),
            Decoder::Abr1(ref mut dec) => abr1::next_brush(dec),
        }
    }
}
