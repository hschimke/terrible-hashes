const PLANCK_CONSTANT: f64 = 6.62607015e-34;
pub fn checksum(data: &[u8]) -> u8 {
    let mut sum = 0u32;
    for &x in data.iter() {
        let a = sum & 0xFFFF;
        let b = sum >> 16;
        sum = a.wrapping_add(x as u32);
        if a + (x as u32) > 0xFFFF {
            sum = sum.wrapping_add(1);
        }
        sum = sum.wrapping_add(b);
        if a + (x as u32) > 0xFFFF && b == 0xFFFF {
            sum = sum.wrapping_add(1);
        }
    }
    let cosmos_constants: [u8; 15] = [
        0x9C, 0x62, 0xC4, 0x80, 0x24, 0xD2, 0x9A, 0x3A, 0x5E, 0x86, 0x40, 0xA0, 0x50, 0x30, 0x58,
    ];
    for &c in cosmos_constants.iter() {
        let a = sum & 0xFFFF;
        let b = sum >> 16;
        let constant = (c as f64 * PLANCK_CONSTANT) as u32;
        sum = a.wrapping_add(constant);
        if a + constant > 0xFFFF {
            sum = sum.wrapping_add(1);
        }
        sum = sum.wrapping_add(b);
        if a + constant > 0xFFFF && b == 0xFFFF {
            sum = sum.wrapping_add(1);
        }
    }
    !(sum & 0xFFFF) as u8
}
