// Extracts brushes from Adobe ABR files as PNGs. Based on gimpbrush-load.c
// from GIMP.

use std::io::stderr;
use std::io::Write;
use std::fs::File;
use std::path::Path;

extern crate getopts;
extern crate image;

mod abr;
mod err;
mod cli;

use err::{Error, SaveBrushError};

// `main` is a wrapper for the C-style main function, `main2`. This let's us
// unwind the stack before we die from calling `process::exit`.

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
            cli::Command::Process {ref input_path, ref output_path} => {
                process(input_path, output_path)
            }
        }
    });

    match result {
        Ok(()) => 0,
        Err(e) => {
            writeln!(stderr(), "error: {}", e).unwrap();
            1
        }
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

    let decoder = match abr::Decoder::open(rdr) {
        Ok(dec) => dec,
        Err(e) => return Err(Error::CouldntOpenAbr(e)),
    };

    if let Err(e) = std::fs::create_dir(output_path) {
        return Err(Error::CouldntCreateOutputDir {
            output_path: output_path.into(),
            err: e,
        });
    }

    for (idx, brush_result) in decoder.enumerate() {
        let save_path = output_path.join(Path::new(&format!("{}.png", idx)));
        match save_brush(brush_result, &save_path) {
            Ok(()) => {
                println!("Wrote {}.", save_path.display());
            }
            Err(e) => {
                writeln!(stderr(), "error saving brush #{}: {}", idx, e).unwrap();
            }
        }
    }

    Ok(())
}

fn save_brush(brush_result: Result<abr::SampleBrush, abr::BrushError>,
              save_path: &Path)
              -> Result<(), SaveBrushError> {
    let brush = try!(brush_result);
    Ok(try!(image::save_buffer(save_path,
                               &brush.data[..],
                               brush.width,
                               brush.height,
                               image::Gray(brush.depth as u8))))
}
