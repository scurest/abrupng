use err::SavePngError;
use pnglib;
use std::fs::File;
use std::path::Path;

pub fn save_greyscale(path: &Path,
                      data: &[u8],
                      width: u32,
                      height: u32,
                      depth: u16)
                      -> Result<(), SavePngError> {
    let fout = File::create(path)?;
    let mut enc = pnglib::Encoder::new(fout, width, height);
    let bit_depth = match depth {
        1 => pnglib::BitDepth::One,
        2 => pnglib::BitDepth::Two,
        4 => pnglib::BitDepth::Four,
        8 => pnglib::BitDepth::Eight,
        16 => pnglib::BitDepth::Sixteen,
        _ => return Err(SavePngError::BadBitDepth(depth)),
    };
    enc.set_color(pnglib::ColorType::Grayscale);
    enc.set_depth(bit_depth);
    let mut writer = enc.write_header()?;
    writer.write_image_data(data)?;
    Ok(())
}
