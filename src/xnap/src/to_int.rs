use crate::bindings::{LengthAndValue, wrapper_free};
pub fn bcd_to_uint(bcd: u32) -> u32 {
    let mut result = 0;
    let mut multiplier = 1;
    let mut value = bcd;

    while value != 0 {
        let digit = value % 16;
        result += digit * multiplier;
        multiplier *= 10;
        value /= 16;
    }

    result
}

pub fn deserialize_64bit(source: &[u8; 8], length_and_value: *mut LengthAndValue) -> u64 {
    unsafe{
        let bit_length: u32 = (*length_and_value).length;
        let nr_of_bytes = (bit_length + 7) / 8;
        let mut tmp = [0u8; 8];
        tmp[..nr_of_bytes as usize].clone_from_slice(&source[..nr_of_bytes as usize]);
        wrapper_free(length_and_value);

        let value = ((tmp[0] as u64) << 56) |
                    ((tmp[1] as u64) << 48) |
                    ((tmp[2] as u64) << 40) |
                    ((tmp[3] as u64) << 32) |
                    ((tmp[4] as u64) << 24) |
                    ((tmp[5] as u64) << 16) |
                    ((tmp[6] as u64) << 8) |
                    (tmp[7] as u64);

        value >> (64 - bit_length)
    }
}

pub fn unload_u32(int_value: &mut u32, value: &[u8]) {
    *int_value = (value[0] as u32) << 24
        | (value[1] as u32) << 16
        | (value[2] as u32) << 8
        | (value[3] as u32);
}

pub fn get_value_as_int32(value: &[u8], byte_length: u16) -> u32 {
    let mut result = 0;
    let mut tmp: [u8; 4] = [0, 0, 0, 0];
    let bit_length = byte_length * 8;
    if bit_length <= 32 {
        let nr_of_bytes = (bit_length + 7) / 8;
        for i in 0..nr_of_bytes.min(4) {
            tmp[i as usize] = value[i as usize];
        }
        unload_u32(&mut result, &tmp);
        return result >> (32 - bit_length);
    } else {
        return result;
    }
}