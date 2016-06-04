use std::io::{Read, Seek, SeekFrom};
use super::byteorder::{self, BigEndian, ReadBytesExt};
use super::util;
use super::{ImageBrush, OpenError, BrushError};

pub struct Abr12Decoder<R> {
    rdr: R,
    version: u16,
    count: u16,
    next_brush_pos: u64,
}


pub fn open<R: Read + Seek>(mut rdr: R,
                            version: u16,
                            count: u16)
                            -> Result<Abr12Decoder<R>, OpenError> {
    let cur_pos = try!(util::tell(&mut rdr));
    Ok(Abr12Decoder {
        rdr: rdr,
        version: version,
        count: count,
        next_brush_pos: cur_pos,
    })
}


pub fn next_brush<R: Read + Seek>(dec: &mut Abr12Decoder<R>)
                                  -> Option<Result<ImageBrush, BrushError>> {
    if dec.count == 0 {
        return None;
    }

    dec.count -= 1;

    // Process the length; if we can't get it, we can't resume on the next brush, so
    // flag the iteration as over by setting count to 0.
    match process_brush_length(dec) {
        Ok(next_brush_pos) => {
            dec.next_brush_pos = next_brush_pos;
        }
        Err(e) => {
            dec.count = 0;
            return Some(Err(BrushError::IoError(e)));
        }
    }

    Some(process_brush_body(dec))
}

/// Get the reader prepped to begin reading out a brush. Returns the position where
/// the next brush starts.
fn process_brush_length<R: Read + Seek>(dec: &mut Abr12Decoder<R>) -> Result<u64, byteorder::Error> {
    let brush_pos = dec.next_brush_pos;

    try!(dec.rdr.seek(SeekFrom::Start(brush_pos)));
    let len = try!(dec.rdr.read_u16::<BigEndian>()) as u64;
    // We are now at brush_pos + 2.
    let next_brush_pos = (brush_pos + 2) + len;

    Ok(next_brush_pos)
}

fn process_brush_body<R: Read + Seek>(dec: &mut Abr12Decoder<R>) -> Result<ImageBrush, BrushError> {
    let ty = try!(dec.rdr.read_u16::<BigEndian>());
    if ty != 2 {
        return Err(BrushError::UnsupportedBrushType { ty: ty });
    }

    let _misc = try!(dec.rdr.read_u32::<BigEndian>());
    let _spacing = try!(dec.rdr.read_u16::<BigEndian>());

    if dec.version == 2 {
        // Skip over a length-prefixed UCS2 String
        let len = try!(dec.rdr.read_u32::<BigEndian>()) as i64;
        let len_in_bytes = 2 * len;
        try!(dec.rdr.seek(SeekFrom::Current(len_in_bytes)));
    }

    let _antialiasing = try!(dec.rdr.read_u8());

    let top = try!(dec.rdr.read_u16::<BigEndian>());
    let left = try!(dec.rdr.read_u16::<BigEndian>());
    let bottom = try!(dec.rdr.read_u16::<BigEndian>());
    let right = try!(dec.rdr.read_u16::<BigEndian>());

    let _topl = try!(dec.rdr.read_u32::<BigEndian>());
    let _leftl = try!(dec.rdr.read_u32::<BigEndian>());
    let _bottoml = try!(dec.rdr.read_u32::<BigEndian>());
    let _rightl = try!(dec.rdr.read_u32::<BigEndian>());

    let depth = try!(dec.rdr.read_u16::<BigEndian>());
    if depth != 8 {
        return Err(BrushError::UnsupportedBitDepth { depth: depth });
    }

    let compressed = try!(dec.rdr.read_u8()) != 0;

    let width = (right - left) as u32;
    let height = (bottom - top) as u32;
    let size = (width as usize) * (height as usize) * (depth as usize >> 3);

    let data = if compressed {
        try!(util::read_rle_data(&mut dec.rdr, height, size))
    } else {
        let mut v = vec![0; size];
        try!(dec.rdr.read_exact(&mut v));
        v
    };

    Ok(ImageBrush {
        width: width,
        height: height,
        depth: depth,
        data: data,
    })
}
