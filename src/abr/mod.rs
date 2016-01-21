use std::io::{Read, Seek};
use self::byteorder::{BigEndian, ReadBytesExt};

extern crate byteorder;

pub mod err;
mod abr6;
mod abr12;
mod helper;

pub use abr::err::{OpenError, BrushError};
pub use abr::abr6::Abr6Decoder;
pub use abr::abr12::Abr12Decoder;

#[derive(Debug)]
pub struct SampleBrush {
    pub width: u32,
    pub height: u32,
    pub depth: u16,
    pub data: Vec<u8>,
}

pub enum Decoder<R> {
    Abr6(Abr6Decoder<R>),
    Abr12(Abr12Decoder<R>),
}

impl<R: Read + Seek> Decoder<R> {
    pub fn open(mut rdr: R) -> Result<Decoder<R>, OpenError> {
        let version = try!(rdr.read_u16::<BigEndian>());
        let subversion = try!(rdr.read_u16::<BigEndian>());
        match version {
            1 | 2 => Ok(Decoder::Abr12(try!(abr12::open(rdr, version, subversion)))),
            6 if subversion == 1 || subversion == 2 => {
                Ok(Decoder::Abr6(try!(abr6::open(rdr, subversion))))
            }
            _ => {
                Err(OpenError::UnsupportedVersion {
                    version: version,
                    subversion: subversion,
                })
            }
        }
    }
}

impl<R: Read + Seek> Iterator for Decoder<R> {
    type Item = Result<SampleBrush, BrushError>;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Decoder::Abr6(ref mut dec) => abr6::next_brush(dec),
            Decoder::Abr12(ref mut dec) => abr12::next_brush(dec),
        }
    }
}
