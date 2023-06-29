use crate::bindings::{LengthAndValue, wrapper_free};

pub fn decompile_tac(tac_data: *mut LengthAndValue) -> u32 {
    let value = unsafe { std::slice::from_raw_parts((*tac_data).value as *const u8, (*tac_data).length as usize) }; //FIXME: från kompilatorn: consult the function's documentation for information on how to avoid undefined behavior
    unsafe {
        wrapper_free(tac_data);
    }
    let mut tac = u32::from(value[0]) << 16;
    tac |= u32::from(value[1]) << 8;
    tac |= u32::from(value[2]);
    return tac;
}