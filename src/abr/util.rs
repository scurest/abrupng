use std;
use std::io::{self, Read, Seek, SeekFrom};
use super::byteorder::{BigEndian, ReadBytesExt};

/// Get the current location in a seekable stream.
pub fn tell<R: Seek>(rdr: &mut R) -> std::io::Result<u64> {
    rdr.seek(SeekFrom::Current(0))
}

/// Read `height` rows of run-length compressed data into a vector.
/// `size_hint` is a guess at the size of the uncompressed data.
pub fn read_rle_data<R: Read>(mut rdr: R,
                              height: u32,
                              size_hint: usize)
                              -> Result<Vec<u8>, io::Error> {
    // There are `height` u16s containing the RLE'd length of each of
    // the `height` scanlines.
    // We just need the total length.
    let mut len = 0u64;
    for _ in 0..height {
        len += rdr.read_u16::<BigEndian>()? as u64;
    }

    // Decode RLE'd data.
    let mut data = Vec::with_capacity(size_hint);
    let mut bytes_read = 0;
    while bytes_read < len {
        let n = rdr.read_i8()?;
        bytes_read += 1;
        if n == -128 {
            // NOP
        } else if n < 0 {
            // RLE encoded. Repeat the next byte -n+1 times.
            let count = -n as usize + 1;
            let b = rdr.read_u8()?;
            bytes_read += 1;
            data.extend(std::iter::repeat(b).take(count));
        } else {
            // Uncoded. Read the next n+1 bytes, raw, from the input.
            let count = n as usize + 1;
            let off = data.len();
            data.extend(std::iter::repeat(0).take(count));
            rdr.read_exact(&mut data[off..])?;
            bytes_read += count as u64;
        }
    }
    Ok(data)
}
