use image::{GenericImageView, GenericImage};
use anyhow::{Result, bail};

pub fn encode(input_path: &std::path::Path, message: &str, output_path: &std::path::Path) -> Result<()> {
    let mut img = image::open(input_path)?;
    let (width, height) = img.dimensions();
    let message_bytes = message.as_bytes();
    let message_len = message_bytes.len() as u32;
    let message_len_bytes = message_len.to_be_bytes();

    let total_capacity = width * height * 3; // 3 bits per pixel (in R, G, B channels)
    let required_bits = (message_len_bytes.len() + message_bytes.len()) * 8;

    if total_capacity < required_bits as u32 {
        bail!("Image is not large enough to hold the message.");
    }

    let mut payload_bits = Vec::with_capacity(required_bits);
    for byte in message_len_bytes.iter().chain(message_bytes.iter()) {
        for i in 0..8 {
            payload_bits.push((byte >> (7 - i)) & 1);
        }
    }
    let mut bit_iter = payload_bits.into_iter();

    'outer: for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel(x, y);

            if let Some(bit) = bit_iter.next() {
                pixel[0] = (pixel[0] & 0xFE) | bit;
            } else {
                break 'outer;
            }

            if let Some(bit) = bit_iter.next() {
                pixel[1] = (pixel[1] & 0xFE) | bit;
            } else {
                img.put_pixel(x, y, pixel);
                break 'outer;
            }

            if let Some(bit) = bit_iter.next() {
                pixel[2] = (pixel[2] & 0xFE) | bit;
            } else {
                img.put_pixel(x, y, pixel);
                break 'outer;
            }
            
            img.put_pixel(x, y, pixel);
        }
    }

    img.save(output_path)?;
    Ok(())
}

pub fn decode(input_path: &std::path::Path) -> Result<String> {
    let img = image::open(input_path)?;
    let mut bits = Vec::new();
    for pixel in img.pixels() {
        let p = pixel.2; // This is an Rgba<u8>
        bits.push(p[0] & 1);
        bits.push(p[1] & 1);
        bits.push(p[2] & 1);
    }
    let mut bit_iter = bits.into_iter();

    // Read the first 32 bits for the message length
    let len_bits: Vec<u8> = bit_iter.by_ref().take(32).collect();
    if len_bits.len() < 32 {
        bail!("Image ended before reading message length");
    }

    let mut len_bytes = [0u8; 4];
    for (i, chunk) in len_bits.chunks(8).enumerate() {
        for (j, bit) in chunk.iter().enumerate() {
            len_bytes[i] |= bit << (7-j);
        }
    }
    let message_len = u32::from_be_bytes(len_bytes);

    // Read the message bits
    let message_bits: Vec<u8> = bit_iter.take((message_len * 8) as usize).collect();
    if message_bits.len() < (message_len * 8) as usize {
        bail!("Image ended before reading full message");
    }

    let mut message_bytes = Vec::new();
    for chunk in message_bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        message_bytes.push(byte);
    }

    let message = String::from_utf8(message_bytes)?;
    Ok(message)
} 