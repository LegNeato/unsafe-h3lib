use ::libc;
extern "C" {
    
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn fclose(_: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
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
    fn getHexagonAreaAvgKm2(res: libc::c_int, out: *mut libc::c_double) -> H3Error;
    fn getHexagonAreaAvgM2(res: libc::c_int, out: *mut libc::c_double) -> H3Error;
    fn getHexagonEdgeLengthAvgKm(res: libc::c_int, out: *mut libc::c_double) -> H3Error;
    fn getHexagonEdgeLengthAvgM(res: libc::c_int, out: *mut libc::c_double) -> H3Error;
    fn getNumCells(res: libc::c_int, out: *mut int64_t) -> H3Error;
    fn getPentagons(res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn error(msg: *const libc::c_char);
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type int64_t = libc::c_longlong;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub _seek: Option<unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t>,
    pub _write: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut libc::c_void,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type uint8_t = libc::c_uchar;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type H3Index = uint64_t;
pub type H3Error = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inputArgs {
    pub res: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn generateTestCase(
    mut filename: *const libc::c_char,
    mut expectedSize: size_t,
) -> libc::c_int {
    let mut fp: *mut FILE = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
    let mut zero: uint8_t = 0 as libc::c_int as uint8_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < expectedSize {
        if fwrite(
            &mut zero as *mut uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            error(b"Error writing\n\0" as *const u8 as *const libc::c_char);
        }
        i = (i as libc::c_ulong).wrapping_add(::core::mem::size_of::<uint8_t>() as libc::c_ulong)
            as size_t as size_t;
    }
    fclose(fp);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LLVMFuzzerTestOneInput(
    mut data: *const uint8_t,
    mut size: size_t,
) -> libc::c_int {
    if size < ::core::mem::size_of::<inputArgs>() as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let mut args: *const inputArgs = data as *const inputArgs;
    let mut out: libc::c_double = 0.;
    getHexagonAreaAvgKm2((*args).res, &mut out);
    getHexagonAreaAvgM2((*args).res, &mut out);
    getHexagonEdgeLengthAvgKm((*args).res, &mut out);
    getHexagonEdgeLengthAvgM((*args).res, &mut out);
    let mut outInt: int64_t = 0;
    getNumCells((*args).res, &mut outInt);
    let mut pentagons: [H3Index; 12] = [0; 12];
    getPentagons((*args).res, pentagons.as_mut_ptr());
    return 0 as libc::c_int;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    if argc == 3 as libc::c_int {
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--generate\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            error(
                b"Invalid option (should be --generate, otherwise look at aflHarness.h to see options)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return generateTestCase(
            *argv.offset(2 as libc::c_int as isize),
            ::core::mem::size_of::<inputArgs>() as libc::c_ulong,
        );
    }
    if argc != 2 as libc::c_int {
        error(
            b"Should have one argument, test case file, or --generate test_case_file\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut filename: *const libc::c_char = *argv.offset(1 as libc::c_int as isize);
    let mut fp: *mut FILE = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        error(b"Error opening test case file\n\0" as *const u8 as *const libc::c_char);
    }
    let mut data: [uint8_t; 4] = [0; 4];
    if fread(
        &mut data as *mut [uint8_t; 4] as *mut libc::c_void,
        ::core::mem::size_of::<inputArgs>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        fp,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        error(b"Error reading test case file\n\0" as *const u8 as *const libc::c_char);
    }
    fclose(fp);
    return LLVMFuzzerTestOneInput(
        data.as_mut_ptr(),
        ::core::mem::size_of::<inputArgs>() as libc::c_ulong,
    );
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
