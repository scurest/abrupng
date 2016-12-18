//! Command-line utility for converting an Adobe ABR file to the
//! brushes it contains (as PNGs).

extern crate getopts;
extern crate image;
#[macro_use]
extern crate quick_error;

mod abr;
mod cli;
mod err;

use err::{Error, ProcessBrushError};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let ret_code = main2();
    std::process::exit(ret_code);
}

/// C-style main function.
fn main2() -> i32 {
    let opts = cli::make_options();
    let result = cli::parse_cli_options(&opts).and_then(|command| {
        match command {
            cli::Command::Help => {
                cli::print_usage(&opts);
                Ok(())
            }
            cli::Command::Process { input_path, output_path } => {
                process(input_path, output_path)
            }
        }
    });

    match result {
        Ok(()) => 0,
        Err(e) => { report_error(e); 1 }
    }
}

/// Reads an ABR file at `input_path` and extracts the image brushes
/// as PNGs, writing them to the directory `output_path`.
fn process(input_path: PathBuf, output_path: PathBuf) -> Result<(), Error> {
    let file = File::open(&input_path)
        .map_err(|e| Error::CouldntOpenFile {
            file_path: input_path,
            err: e,
        })?;
    let rdr = std::io::BufReader::new(file);

    let brushes = abr::open(rdr)
        .map_err(|e| Error::CouldntOpenAbr(e))?;

    std::fs::create_dir(&output_path)
        .map_err(|e| Error::CouldntCreateOutputDir {
            output_path: output_path.clone(),
            err: e,
        })?;

    for (idx, brush_result) in brushes.enumerate() {
        let save_path = output_path.join(Path::new(&format!("{}.png", idx)));
        match process_brush(brush_result, &save_path) {
            Ok(()) => println!("Wrote {}.", save_path.display()),
            Err(e) => writeln!(io::stderr(), "error on brush {}: {}", idx, e).unwrap(),
        }
    }

    Ok(())
}

/// Saves the result of reading out a brush to `save_path`. Returns an
/// error if either the reading failed or the writing fails.
fn process_brush(brush_result: Result<abr::ImageBrush, abr::BrushError>,
              save_path: &Path)
              -> Result<(), ProcessBrushError> {
    let brush = brush_result?;
    image::save_buffer(save_path,
                       &brush.data[..],
                       brush.width,
                       brush.height,
                       image::Gray(brush.depth as u8))?;
    Ok(())
}

/// Prints an error, plus some information for humans about what they
/// might do about it.
fn report_error(err: Error) {
    let stderr = io::stderr();
    let mut out = stderr.lock();

    writeln!(out, "error: {}", err).unwrap();

    // Try to suggest how to fix it.
    match err {
        Error::BadCommandlineOptions(_) | Error::WrongNumberOfInputFiles(_) =>
            writeln!(out, "Use -h for help.").unwrap(),
        Error::CouldntOpenAbr(_) =>
            writeln!(out, "Ensure the provided file was an ABR. If it was, \
                           it's unsuporrted, sorry :(").unwrap(),
        Error::CouldntCreateOutputDir { err: ref io_err, .. }
            if io_err.kind() == io::ErrorKind::AlreadyExists =>
            writeln!(out, "The output directory will be created. Make sure \
                           it doesn't already exist.").unwrap(),
        _ => (),
    }
}
