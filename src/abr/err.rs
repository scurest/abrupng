use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum OpenError {
        UnsupportedVersion {
            version: u16,
            subversion: u16,
        } {
            description("unknown/unsupported version")
            display("unknown/unsupported version: {}.{}", version, subversion)
        }
        Found8bim {
            // What IS this?
            description("found 8bim")
        }
        IoError(err: io::Error) {
            description("read error")
            display("read error: {}", err)
            cause(err)
            from()
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum BrushError {
        UnsupportedBitDepth { depth: u16 } {
            description("unsupported bit-depth")
            display("unsupported bit-depth, {}-bit", depth)
        }
        UnsupportedBrushType { ty: u16 } {
            description("unsupported brush type")
            display("unsupported brush type: {}", ty)
        }
        IoError(err: io::Error) {
            description("read error")
            display("read error: {}", err)
            cause(err)
            from()
        }
    }
}
