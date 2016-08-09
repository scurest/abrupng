use abr;
use getopts;
use std::io;
use std::path::PathBuf;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        BadCommandlineOptions(reason: getopts::Fail) {
            description("bad command-line option")
            display("bad command-line option: {}", reason)
            cause(reason)
            from()
        }
        WrongNumberOfInputFiles(num: usize) {
            description("expected exactly one input file")
            display("expected exactly one input file but got {}", num)
        }
        CouldntOpenFile { file_path: PathBuf, err: io::Error } {
            description("couldn't open file")
            display("couldn't open file {}: {}", file_path.display(), err)
            cause(err)
        }
        CouldntOpenAbr(err: abr::OpenError) {
            description("couldn't open as ABR")
            display("couldn't open as ABR: {}", err)
            cause(err)
            from()
        }
        CouldntGuessOutputName {
            description("couldn't guess output name from input")
        }
        CouldntCreateOutputDir { output_path: PathBuf, err: io::Error } {
            description("couldn't create output directory")
            display("couldn't create output directory {}: {}",
                    output_path.display(), err)
            cause(err)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum ProcessBrushError {
        AbrBrushError(err: abr::BrushError) {
            description("couldn't read brush")
            display("couldn't read brush: {}", err)
            cause(err)
            from()
        }
        SavePngError(err: io::Error) {
            description("couldn't save PNG")
            display("couldn't save PNG: {}", err)
            cause(err)
            from()
        }
    }
}
