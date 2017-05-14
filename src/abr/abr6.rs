use std::io::{self, Read, Seek, SeekFrom};
use super::byteorder::{BigEndian, ReadBytesExt};
use super::{ImageBrush, OpenError, BrushError};
use super::util;

/// Decoder state for ABR6-like formats (versions 6 and 10).
pub struct Decoder<R> {
    rdr: R,
    #[allow(dead_code)]
    version: u16,
    subversion: u16,
    sample_section_end: u64,
    next_brush_pos: u64,
}

pub fn open<R: Read + Seek>(mut rdr: R, version: u16, subversion: u16)
                            -> Result<Decoder<R>, OpenError> {
    // Find the sample section
    loop {
        let mut buf = [0; 4];

        rdr.read_exact(&mut buf)?;
        if buf == &b"8bim"[..] {
            return Err(OpenError::Found8bim);
        }

        rdr.read_exact(&mut buf)?;
        if buf == &b"samp"[..] {
            break;
        }

        let len = rdr.read_u32::<BigEndian>()?;
        rdr.seek(SeekFrom::Current(len as i64))?;
    }

    let len = rdr.read_u32::<BigEndian>()? as u64;
    let cur = util::tell(&mut rdr)?;
    let sample_section_end = cur + len;

    Ok(Decoder {
        rdr,
        version,
        subversion,
        sample_section_end,
        next_brush_pos: cur,
    })
}


pub fn next_brush<R: Read + Seek>(dec: &mut Decoder<R>)
                                  -> Option<Result<ImageBrush, BrushError>> {
    // Is iteration over?
    if dec.next_brush_pos >= dec.sample_section_end {
        return None;
    }

    Some(match do_brush_head(dec) {
        Ok(res) => {
            dec.next_brush_pos = res.next_brush_pos;
            do_brush_body(dec)
        }
        Err(e) => {
            // We didn't get the next brush's position, so we can't resume on
            // the next brush. Flag the iteration as over before we error out.
            dec.next_brush_pos = dec.sample_section_end;
            Err(e.into())
        }
    })
}

struct BrushHeadResult {
    next_brush_pos: u64,
}

/// Moves `dec` into position to read out the next brush with `do_brush_body`.
/// Returns where the brush after this one is located.
fn do_brush_head<R: Read + Seek>(dec: &mut Decoder<R>)
                                   -> Result<BrushHeadResult, io::Error> {
    let brush_pos = dec.next_brush_pos;
    dec.rdr.seek(SeekFrom::Start(brush_pos))?;

    let len = dec.rdr.read_u32::<BigEndian>()? as u64;
    // We are now at brush_pos + 4
    let end_pos = (brush_pos + 4) + len;
    // Brushes are aligned to 4-byte boundaries; round up to get to one.
    let next_brush_pos = (end_pos + 3) & !3;

    Ok(BrushHeadResult { next_brush_pos })
}

/// With `dec` positioned by `do_brush_head`, reads out a brush.
fn do_brush_body<R: Read + Seek>(dec: &mut Decoder<R>) -> Result<ImageBrush, BrushError> {
    // Skip over... something.
    let skip_amt = if dec.subversion == 1 { 47 } else { 301 };
    dec.rdr.seek(SeekFrom::Current(skip_amt))?;

    let top = dec.rdr.read_u32::<BigEndian>()?;
    let left = dec.rdr.read_u32::<BigEndian>()?;
    let bottom = dec.rdr.read_u32::<BigEndian>()?;
    let right = dec.rdr.read_u32::<BigEndian>()?;

    let depth = dec.rdr.read_u16::<BigEndian>()?;
    if depth != 8 {
        return Err(BrushError::UnsupportedBitDepth { depth });
    }

    let compressed = dec.rdr.read_u8()? != 0;

    let width = right - left;
    let height = bottom - top;
    let size = (width as usize) * (height as usize) * (depth as usize >> 3);

    let data = if compressed {
        util::read_rle_data(&mut dec.rdr, height, size)?
    } else {
        let mut v = vec![0; size];
        dec.rdr.read_exact(&mut v)?;
        v
    };

    Ok(ImageBrush { width, height, depth, data })
}
