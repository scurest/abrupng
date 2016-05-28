use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;
use abr;

#[derive(Debug)]
pub enum SaveBrushError {
    AbrBrushError(abr::BrushError),
    SavePngError(io::Error),
}

#[derive(Debug)]
pub enum Error {
    BadCommandlineOptions,
    WrongNumberOfInputFiles(usize),
    CouldntOpenFile {
        file_path: PathBuf,
        err: io::Error,
    },
    CouldntOpenAbr(abr::OpenError),
    CouldntGuessOutputName,
    CouldntCreateOutputDir {
        output_path: PathBuf,
        err: io::Error,
    },
}

impl From<abr::BrushError> for SaveBrushError {
    fn from(e: abr::BrushError) -> SaveBrushError {
        SaveBrushError::AbrBrushError(e)
    }
}

impl From<io::Error> for SaveBrushError {
    fn from(e: io::Error) -> SaveBrushError {
        SaveBrushError::SavePngError(e)
    }
}

impl fmt::Display for SaveBrushError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SaveBrushError::AbrBrushError(ref e) => write!(f, "Error reading brush: {}", *e),
            SaveBrushError::SavePngError(ref e) => write!(f, "Failed to save PNG: {}", *e),
        }
    }
}

impl error::Error for SaveBrushError {
    fn description(&self) -> &str {
        match *self {
            SaveBrushError::AbrBrushError(_) => "Error reading brush",
            SaveBrushError::SavePngError(_) => "Failed to save PNG",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SaveBrushError::AbrBrushError(ref e) => Some(e),
            SaveBrushError::SavePngError(ref e) => Some(e),
        }
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BadCommandlineOptions => {
                write!(f, "Unexpected command-line option. Use -h for help.")
            }
            Error::WrongNumberOfInputFiles(num) => {
                write!(f, "Expected exactly one input file, but got {}.", num)
            }
            Error::CouldntOpenFile {ref file_path, ref err} => {
                write!(f, "Couldn't open file {}. {}", file_path.display(), *err)
            }
            Error::CouldntOpenAbr(ref e) => {
                write!(f, "Couldn't open file as ABR (is it an ABR?): {}", *e)
            }
            Error::CouldntGuessOutputName => {
                // TODO: Can this actually happen?
                write!(f,
                       "Couldn't guess an output directory from the input, please supply one \
                        explicitly with -o.")
            }
            Error::CouldntCreateOutputDir {ref output_path, ref err} => {
                write!(f,
                       "Couldn't create output directory {}. {}",
                       output_path.display(),
                       *err)
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadCommandlineOptions => "unexpected command-line option",
            Error::WrongNumberOfInputFiles(_) => "expected exactly one input file",
            Error::CouldntOpenFile {..} => "couldn't open file",
            Error::CouldntOpenAbr(_) => "couldn't open file as ABR",
            Error::CouldntGuessOutputName => "couldn't guess an output directory from the input",
            Error::CouldntCreateOutputDir {..} => "couldn't create output directory",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::CouldntOpenFile {ref err, ..} => Some(err),
            Error::CouldntOpenAbr(ref e) => Some(e),
            Error::CouldntCreateOutputDir {ref err, ..} => Some(err),
            _ => None,
        }
    }
}
