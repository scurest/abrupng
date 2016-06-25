// Extracts brushes from Adobe ABR files as PNGs. Based on gimpbrush-load.c
// from GIMP.

extern crate getopts;
extern crate image;
#[macro_use] extern crate quick_error;

mod abr;
mod cli;
mod err;

use err::{Error, SaveBrushError};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    let ret_code = main2();
    std::process::exit(ret_code);
}

fn main2() -> i32 {
    let opts = cli::make_options();
    let result = cli::parse_cli_options(&opts).and_then(|command| {
        match command {
            cli::Command::Help => {
                cli::print_usage(&opts);
                Ok(())
            }
            cli::Command::Process { ref input_path, ref output_path } => {
                process(input_path, output_path)
            }
        }
    });

    match result {
        Ok(()) => 0,
        Err(e) => { report_error(e); 1 }
    }
}

fn process(input_path: &Path, output_path: &Path) -> Result<(), Error> {
    let rdr = std::io::BufReader::new(match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(Error::CouldntOpenFile {
                file_path: input_path.into(),
                err: e,
            })
        }
    });

    let brushes = match abr::open(rdr) {
        Ok(dec) => dec,
        Err(e) => return Err(Error::CouldntOpenAbr(e)),
    };

    if let Err(e) = std::fs::create_dir(output_path) {
        return Err(Error::CouldntCreateOutputDir {
            output_path: output_path.into(),
            err: e,
        });
    }

    for (idx, brush_result) in brushes.enumerate() {
        let save_path = output_path.join(Path::new(&format!("{}.png", idx)));
        match save_brush(brush_result, &save_path) {
            Ok(()) => {
                println!("Wrote {}.", save_path.display());
            }
            Err(e) => {
                writeln!(io::stderr(), "Error saving brush #{}: {}", idx, e).unwrap();
            }
        }
    }

    Ok(())
}

fn save_brush(brush_result: Result<abr::ImageBrush, abr::BrushError>,
              save_path: &Path)
              -> Result<(), SaveBrushError> {
    let brush = try!(brush_result);
    Ok(try!(image::save_buffer(save_path,
                               &brush.data[..],
                               brush.width,
                               brush.height,
                               image::Gray(brush.depth as u8))))
}

fn report_error(err: Error) {
    let stderr = io::stderr();
    let mut out = stderr.lock();

    writeln!(out, "error: {}", err).unwrap();

    // Try to suggest how to fix it.
    match err {
        Error::BadCommandlineOptions | Error::WrongNumberOfInputFiles(_) =>
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
