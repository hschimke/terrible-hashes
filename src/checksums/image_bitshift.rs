use image::{ImageBuffer, Rgba};
use imageproc::filter;

pub fn calculate_checksum_checksum(data: &[u8]) -> u64 {
    let mut checksum: u64 = 0;
    for (i, byte) in data.iter().enumerate() {
        if i % 2 == 0 {
            checksum = checksum ^ ((*byte as u64) << 1);
        } else {
            checksum = checksum ^ ((*byte as u64) >> 1);
        }
    }
    let side = ((data.len() / 4) as f64).sqrt().ceil() as u32;
    let img = ImageBuffer::from_fn(side, side, |x, y| {
        if y * side + x < (data.len() / 4) as u32 {
            let i = (y * side + x) as usize * 4;
            Rgba([data[i], data[i + 1], data[i + 2], data[i + 3]])
        } else {
            Rgba([0, 0, 0, 0])
        }
    });
    let filtered_img = filter::gaussian_blur_f32(&img, 3.0);
    let mut pixel_weights: u64 = 0;
    for pixel in filtered_img.pixels() {
        pixel_weights += pixel[0] as u64;
    }
    checksum ^ pixel_weights
}
