use super::byteorder;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum OpenError {
    UnsupportedVersion {
        version: u16,
        subversion: u16,
    },
    Found8bim,
    IoError(byteorder::Error),
}

#[derive(Debug)]
pub enum BrushError {
    UnsupportedBitDepth {
        depth: u16,
    },
    UnsupportedBrushType {
        ty: u16,
    },
    IoError(byteorder::Error),
}

impl<T: Into<byteorder::Error>> From<T> for OpenError {
    fn from(e: T) -> OpenError {
        OpenError::IoError(e.into())
    }
}

impl<T: Into<byteorder::Error>> From<T> for BrushError {
    fn from(e: T) -> BrushError {
        BrushError::IoError(e.into())
    }
}


impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpenError::UnsupportedVersion { version, subversion } => {
                write!(f, "Unsupported version {}/{}", version, subversion)
            }
            OpenError::Found8bim => write!(f, "Found 8bim"),
            OpenError::IoError(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for OpenError {
    fn description(&self) -> &str {
        match *self {
            OpenError::UnsupportedVersion { .. } => "Unsupported version",
            OpenError::Found8bim => "Found 8bim",
            OpenError::IoError(ref e) => error::Error::description(e),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            OpenError::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for BrushError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BrushError::UnsupportedBitDepth { depth } => {
                write!(f, "Unsupported bit-depth, {}", depth)
            }
            BrushError::UnsupportedBrushType { ty } => write!(f, "Unsupported brush type, {}", ty),
            BrushError::IoError(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for BrushError {
    fn description(&self) -> &str {
        match *self {
            BrushError::UnsupportedBitDepth { .. } => "Unsupported bit-depth",
            BrushError::UnsupportedBrushType { .. } => "Unsupported brush type",
            BrushError::IoError(ref e) => error::Error::description(e),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            BrushError::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}
