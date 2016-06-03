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
    let first_arg = env::args().next();
    let prgm_name = first_arg.as_ref().map(String::as_str).unwrap_or("abr2png");
    let brief = format!("Usage: {} INPUT [-o OUTPUT]", prgm_name);
    print!("{}", opts.usage(&brief));
}

pub fn parse_cli_options(opts: &Options) -> Result<Command, Error> {
    let args: Vec<String> = env::args().collect();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => return Err(Error::BadCommandlineOptions),
    };

    if matches.opt_present("h") {
        return Ok(Command::Help);
    }

    let input_path = PathBuf::from(if matches.free.len() == 1 {
        &matches.free[0]
    } else {
        return Err(Error::WrongNumberOfInputFiles(matches.free.len()));
    });

    // Get the output directory's path. If one isn't given, try to guess one
    // from the stem of the input file (ex. mybrushes.abr => ./mybrushes).
    let output_path = match matches.opt_str("o") {
        Some(s) => PathBuf::from(s),
        None => {
            match input_path.file_stem() {
                Some(name) => PathBuf::from(name),
                None => return Err(Error::CouldntGuessOutputName),
            }
        }
    };

    Ok(Command::Process {
        input_path: input_path,
        output_path: output_path,
    })
}
