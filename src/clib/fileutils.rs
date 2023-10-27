use ::libc;
extern "C" {
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type FILE = libc::FILE;
pub type Bool = libc::c_uint;
pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub type uint8 = libc::c_uchar;
pub type int8 = libc::c_schar;
pub type int16 = libc::c_short;
pub type int32 = libc::c_int;
unsafe extern "C" fn parse_int32_be(mut buff: *const uint8) -> int32 {
    return (*buff.offset(0 as libc::c_int as isize) as libc::c_int) << 24 as libc::c_int
        | (*buff.offset(1 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
        | (*buff.offset(2 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *buff.offset(3 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn file_read_int32(mut file: *mut FILE, mut o_val: *mut int32) -> Bool {
    let mut buff: [uint8; 4] = [0; 4];
    if fread(
        buff.as_mut_ptr() as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 1 as libc::c_int as libc::c_ulong
    {
        *o_val = parse_int32_be(buff.as_mut_ptr());
        return DA_TRUE;
    }
    return DA_FALSE;
}
unsafe extern "C" fn serialize_int32_be(mut buff: *mut uint8, mut val: int32) {
    *buff.offset(0 as libc::c_int as isize) =
        (val >> 24 as libc::c_int & 0xff as libc::c_int) as uint8;
    *buff.offset(1 as libc::c_int as isize) =
        (val >> 16 as libc::c_int & 0xff as libc::c_int) as uint8;
    *buff.offset(2 as libc::c_int as isize) =
        (val >> 8 as libc::c_int & 0xff as libc::c_int) as uint8;
    *buff.offset(3 as libc::c_int as isize) = (val & 0xff as libc::c_int) as uint8;
}
#[no_mangle]
pub unsafe extern "C" fn serialize_int32_be_incr(mut buff: *mut *mut uint8, mut val: int32) {
    serialize_int32_be(*buff, val);
    *buff = (*buff).offset(4 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn file_write_int32(mut file: *mut FILE, mut val: int32) -> Bool {
    let mut buff: [uint8; 4] = [0; 4];
    serialize_int32_be(buff.as_mut_ptr(), val);
    return (fwrite(
        buff.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 1 as libc::c_int as libc::c_ulong) as libc::c_int as Bool;
}
unsafe extern "C" fn parse_int16_be(mut buff: *const uint8) -> int16 {
    return ((*buff.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *buff.offset(1 as libc::c_int as isize) as libc::c_int) as int16;
}
#[no_mangle]
pub unsafe extern "C" fn file_read_int16(mut file: *mut FILE, mut o_val: *mut int16) -> Bool {
    let mut buff: [uint8; 2] = [0; 2];
    if fread(
        buff.as_mut_ptr() as *mut libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 1 as libc::c_int as libc::c_ulong
    {
        *o_val = parse_int16_be(buff.as_mut_ptr());
        return DA_TRUE;
    }
    return DA_FALSE;
}
unsafe extern "C" fn serialize_int16_be(mut buff: *mut uint8, mut val: int16) {
    *buff.offset(0 as libc::c_int as isize) = (val as libc::c_int >> 8 as libc::c_int) as uint8;
    *buff.offset(1 as libc::c_int as isize) = (val as libc::c_int & 0xff as libc::c_int) as uint8;
}
#[no_mangle]
pub unsafe extern "C" fn serialize_int16_be_incr(mut buff: *mut *mut uint8, mut val: int16) {
    serialize_int16_be(*buff, val);
    *buff = (*buff).offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn file_write_int16(mut file: *mut FILE, mut val: int16) -> Bool {
    let mut buff: [uint8; 2] = [0; 2];
    serialize_int16_be(buff.as_mut_ptr(), val);
    return (fwrite(
        buff.as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 1 as libc::c_int as libc::c_ulong) as libc::c_int as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn file_read_int8(mut file: *mut FILE, mut o_val: *mut int8) -> Bool {
    return (fread(
        o_val as *mut libc::c_void,
        ::core::mem::size_of::<int8>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 1 as libc::c_int as libc::c_ulong) as libc::c_int as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn file_write_int8(mut file: *mut FILE, mut val: int8) -> Bool {
    return (fwrite(
        &mut val as *mut int8 as *const libc::c_void,
        ::core::mem::size_of::<int8>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        file,
    ) == 1 as libc::c_int as libc::c_ulong) as libc::c_int as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn file_read_chars(
    mut file: *mut FILE,
    mut buff: *mut libc::c_char,
    mut len: libc::c_int,
) -> Bool {
    return (fread(
        buff as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        len as libc::c_ulong,
        file,
    ) == len as libc::c_ulong) as libc::c_int as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn file_write_chars(
    mut file: *mut FILE,
    mut buff: *const libc::c_char,
    mut len: libc::c_int,
) -> Bool {
    return (fwrite(
        buff as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        len as libc::c_ulong,
        file,
    ) == len as libc::c_ulong) as libc::c_int as Bool;
}
