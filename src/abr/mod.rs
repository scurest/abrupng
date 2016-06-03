extern crate byteorder;

mod abr12;
mod abr6;
mod err;
mod helper;

pub use abr::err::{OpenError, BrushError};
use self::byteorder::{BigEndian, ReadBytesExt};
use self::abr12::Abr12Decoder;
use self::abr6::Abr6Decoder;
use std::io::{Read, Seek};

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
    let version = try!(rdr.read_u16::<BigEndian>());
    let subversion = try!(rdr.read_u16::<BigEndian>());

    Ok(Brushes(match version {
        1 | 2 => {
            Decoder::Abr12(try!(abr12::open(rdr, version, subversion)))
        }
        6 if subversion == 1 || subversion == 2 => {
            Decoder::Abr6(try!(abr6::open(rdr, subversion)))
        }
        _ => return Err(OpenError::UnsupportedVersion {
            version: version,
            subversion: subversion,
        })
    }))
}

enum Decoder<R> {
    Abr6(Abr6Decoder<R>),
    Abr12(Abr12Decoder<R>),
}

impl<R: Read + Seek> Iterator for Brushes<R> {
    type Item = Result<ImageBrush, BrushError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Decoder::Abr6(ref mut dec) => abr6::next_brush(dec),
            Decoder::Abr12(ref mut dec) => abr12::next_brush(dec),
        }
    }
}
