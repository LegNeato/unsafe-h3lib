use ::libc;
pub type int64_t = libc::c_longlong;
#[no_mangle]
pub unsafe extern "C" fn _ipow(mut base: int64_t, mut exp: int64_t) -> int64_t {
    let mut result: int64_t = 1 as libc::c_int as int64_t;
    while exp != 0 {
        if exp & 1 as libc::c_int as libc::c_longlong != 0 {
            result *= base;
        }
        exp >>= 1 as libc::c_int;
        base *= base;
    }
    return result;
}
