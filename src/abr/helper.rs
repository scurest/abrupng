use std;
use std::io::{Read, Seek, SeekFrom};
use super::byteorder::{self, BigEndian, ReadBytesExt};

/// Get the current location in a seekable stream.
pub fn tell<R: Seek>(rdr: &mut R) -> std::io::Result<u64> {
    rdr.seek(SeekFrom::Current(0))
}

/// Read `height` rows of run-length compressed data into a vector.
/// `size_hint` is a guess at the size of the uncompressed data.
pub fn read_rle_data<R: Read>(mut rdr: R,
                              height: u32,
                              size_hint: usize)
                              -> Result<Vec<u8>, byteorder::Error> {
    // Read the lengths of the RLE-coded scanlines
    let lens: Vec<usize> = try!((0..height)
        .map(|_| rdr.read_u16::<BigEndian>().map(|x| x as usize))
        .collect());

    // Decode.
    let mut data = Vec::with_capacity(size_hint);
    for len in lens {
        let mut bytes_read = 0;
        while bytes_read < len {
            let n = try!(rdr.read_i8());
            bytes_read += 1;
            if n < 0 {
                // RLE encoded. Repeat the next byte -n+1 times.
                if n == -128 {
                    // (...except this is a NOP)
                    continue;
                }
                let count = (-n) as usize + 1;
                let b = try!(rdr.read_u8());
                bytes_read += 1;
                data.extend(std::iter::repeat(b).take(count));
            } else {
                // Uncoded. Read the next n+1 bytes, raw, from the input.
                let count = n as usize + 1;
                let start_idx = data.len();
                data.extend(std::iter::repeat(0).take(count));
                try!(rdr.read_exact(&mut data[start_idx..]));
                bytes_read += count;
            }
        }
    }
    Ok(data)
}
