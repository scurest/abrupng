use std::io;
use std::path::PathBuf;
use abr;

pub enum SaveBrushError {
    AbrBrushError(abr::BrushError),
    SavePngError(io::Error),
}

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

macro_rules! println_err {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
                panic!("Failed to write to stderr.\
                    \nOriginal error output: {}\
                    \nSecondary error writing to stderr: {}", format!($($arg)*), e);
            }
        }
    )
}

pub fn print_error_msg(err: Error) {
    match err {
        Error::BadCommandlineOptions => {
            println_err!("Err: Unexpected command-line option. Use -h for help.");
        }
        Error::WrongNumberOfInputFiles(num) => {
            println_err!("Err: Expected exactly one input file, but got {}.", num);
        }
        Error::CouldntOpenFile {file_path, err} => {
            println_err!("Err: Couldn't open file {}. {}", file_path.display(), err);
        }
        Error::CouldntOpenAbr(e) => {
            println_err!("Err: Couldn't open file as ABR (is it an ABR?): {}", e);
        }
        Error::CouldntGuessOutputName => {
            println_err!("Err: Couldn't guess an output directory from the input, please supply \
                          one explicitly with -o.");
        }
        Error::CouldntCreateOutputDir {output_path, err} => {
            println_err!("Err: Couldn't create output directory {}. {}",
                         output_path.display(),
                         err);
        }
    }
}

pub fn print_save_brush_error_msg(idx: usize, err: SaveBrushError) {
    match err {
        SaveBrushError::AbrBrushError(e) => {
            println_err!("Failed to read brush #{}. {}", idx, e);
        }
        SaveBrushError::SavePngError(e) => {
            println_err!("Failed to write brush #{}. {}", idx, e);
        }
    }
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
