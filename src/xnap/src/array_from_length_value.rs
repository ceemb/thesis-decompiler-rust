use crate::bindings::{LengthAndValue, wrapper_free};

pub fn array_from_length_value_3(length_and_value: *mut LengthAndValue) -> [u8; 3] {
    unsafe {
        let value_slice = std::slice::from_raw_parts((*length_and_value).value, (*length_and_value).length as usize);
        wrapper_free(length_and_value);
        let mut octet_array: [u8; 3] = [0; 3];

        for (i, byte) in value_slice.iter().enumerate().take(3) {
            let unsigned_byte = *byte as u8;
            octet_array[i] = unsigned_byte;
        }
        return octet_array;
    } // compiler's warning regarding this specific unsafe block: "note: consult the function's documentation for information on how to avoid undefined behavior".
}

pub fn array_from_length_value_8(length_and_value: *mut LengthAndValue) -> [u8; 8] {
    unsafe {
        let value_slice = std::slice::from_raw_parts((*length_and_value).value, (*length_and_value).length as usize);
        let mut octet_array: [u8; 8] = [0; 8];

        for (i, byte) in value_slice.iter().enumerate().take(8) {
            let unsigned_byte = *byte as u8;
            octet_array[i] = unsigned_byte;
        }
        return octet_array;
    }
}