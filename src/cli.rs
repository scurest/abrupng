use err::Error;
use getopts::Options;
use std::env;
use std::path::PathBuf;

pub enum Command {
    Help,
    Process {
        input_path: PathBuf,
        output_path: PathBuf,
    },
}

pub fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optopt("o", "", "set output directory (will be created)", "DIR");
    opts.optflag("h", "help", "print this help menu");
    opts
}

pub fn print_usage(opts: &Options) {
    let brief = "Extracts image brushes from Adobe ABR files as PNGs.\n\nUsage:\n    abrupng \
                 INPUT [-o OUTPUT]";
    print!("{}", opts.usage(&brief));
}

pub fn parse_cli_options(opts: &Options) -> Result<Command, Error> {
    let args: Vec<String> = env::args().collect();
    let matches = try!(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        Ok(Command::Help)
    } else {
        let input_path = if matches.free.len() == 1 {
            PathBuf::from(&matches.free[0])
        } else {
            return Err(Error::WrongNumberOfInputFiles(matches.free.len()));
        };

        // Get the output directory's path. If one isn't given, try to guess one
        // from the stem of the input file (ex. mybrushes.abr => ./mybrushes).
        let output_path = match matches.opt_str("o") {
            Some(s) => PathBuf::from(s),
            None => {
                match input_path.file_stem() {
                    Some(stem) => PathBuf::from(stem),
                    None => return Err(Error::CouldntGuessOutputName),
                }
            }
        };

        Ok(Command::Process {
            input_path: input_path,
            output_path: output_path,
        })
    }
}
