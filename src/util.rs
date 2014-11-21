pub fn bits_to_uint_16(number_bytes: &[u8]) -> uint {
    (number_bytes[1] as u16 + (number_bytes[0] as u16 << 8)) as uint
}

pub fn bits_to_uint_24(number_bytes: &[u8]) -> uint {
    (number_bytes[2] as u32 + 
     (number_bytes[1] as u32 << 8) + 
     (number_bytes[0] as u32 << 16)) as uint
}
