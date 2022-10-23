extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn gridDiskDistancesSafe(
        origin: H3Index,
        k: libc::c_int,
        out: *mut H3Index,
        distances: *mut libc::c_int,
    ) -> H3Error;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn gridDiskUnsafe(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    fn _h3LeadingNonZeroDigit(h: H3Index) -> Direction;
    fn error(msg: *const libc::c_char);
    fn h3Println(h: H3Index);
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
pub type H3Index = uint64_t;
pub type H3Error = uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const E_OPTION_INVALID: C2RustUnnamed = 15;
pub const E_MEMORY_BOUNDS: C2RustUnnamed = 14;
pub const E_MEMORY_ALLOC: C2RustUnnamed = 13;
pub const E_RES_MISMATCH: C2RustUnnamed = 12;
pub const E_NOT_NEIGHBORS: C2RustUnnamed = 11;
pub const E_DUPLICATE_INPUT: C2RustUnnamed = 10;
pub const E_PENTAGON: C2RustUnnamed = 9;
pub const E_VERTEX_INVALID: C2RustUnnamed = 8;
pub const E_UNDIR_EDGE_INVALID: C2RustUnnamed = 7;
pub const E_DIR_EDGE_INVALID: C2RustUnnamed = 6;
pub const E_CELL_INVALID: C2RustUnnamed = 5;
pub const E_RES_DOMAIN: C2RustUnnamed = 4;
pub const E_LATLNG_DOMAIN: C2RustUnnamed = 3;
pub const E_DOMAIN: C2RustUnnamed = 2;
pub const E_FAILED: C2RustUnnamed = 1;
pub const E_SUCCESS: C2RustUnnamed = 0;
pub type Direction = libc::c_uint;
pub const PENTAGON_SKIPPED_DIGIT: Direction = 1;
pub const NUM_DIGITS: Direction = 7;
pub const INVALID_DIGIT: Direction = 7;
pub const IJ_AXES_DIGIT: Direction = 6;
pub const IK_AXES_DIGIT: Direction = 5;
pub const I_AXES_DIGIT: Direction = 4;
pub const JK_AXES_DIGIT: Direction = 3;
pub const J_AXES_DIGIT: Direction = 2;
pub const K_AXES_DIGIT: Direction = 1;
pub const CENTER_DIGIT: Direction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestOutput {
    pub ret0: libc::c_int,
    pub ret0ValidationFailures: libc::c_int,
    pub ret1: libc::c_int,
    pub ret1ValidationFailures: libc::c_int,
    pub ret2: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn doCell(
    mut h: H3Index,
    mut maxK: libc::c_int,
    mut testOutput: *mut TestOutput,
) {
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < maxK {
        let mut maxSz: int64_t = 0;
        maxGridDiskSize(k, &mut maxSz);
        let mut gridDiskInternalOutput: *mut H3Index = calloc(
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
            maxSz as libc::c_ulong,
        ) as *mut H3Index;
        let mut gridDiskUnsafeOutput: *mut H3Index = calloc(
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
            maxSz as libc::c_ulong,
        ) as *mut H3Index;
        let mut gridDiskInternalDistances: *mut libc::c_int = calloc(
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            maxSz as libc::c_ulong,
        ) as *mut libc::c_int;
        gridDiskDistancesSafe(h, k, gridDiskInternalOutput, gridDiskInternalDistances);
        let mut gridDiskUnsafeFailed: H3Error = gridDiskUnsafe(h, k, gridDiskUnsafeOutput);
        if gridDiskUnsafeFailed == E_FAILED as libc::c_int as libc::c_uint {
            (*testOutput).ret2 += 1;
        } else {
            if gridDiskUnsafeFailed == E_SUCCESS as libc::c_int as libc::c_uint {
                (*testOutput).ret0 += 1;
                let mut startIdx: libc::c_int = 0 as libc::c_int;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i <= k {
                    let mut n: libc::c_int = i * 6 as libc::c_int;
                    if i == 0 as libc::c_int {
                        n = 1 as libc::c_int;
                    }
                    let mut ii: libc::c_int = 0 as libc::c_int;
                    while ii < n {
                        let mut h2: H3Index =
                            *gridDiskUnsafeOutput.offset((ii + startIdx) as isize);
                        let mut found: libc::c_int = 0 as libc::c_int;
                        let mut iii: int64_t = 0 as libc::c_int as int64_t;
                        while iii < maxSz {
                            if *gridDiskInternalOutput.offset(iii as isize) == h2
                                && *gridDiskInternalDistances.offset(iii as isize) == i
                            {
                                found = 1 as libc::c_int;
                                break;
                            } else {
                                iii += 1;
                            }
                        }
                        if found == 0 {
                            (*testOutput).ret0ValidationFailures += 1;
                            h3Println(h);
                            return;
                        }
                        ii += 1;
                    }
                    startIdx += n;
                    i += 1;
                }
            } else if gridDiskUnsafeFailed == E_PENTAGON as libc::c_int as libc::c_uint {
                (*testOutput).ret1 += 1;
                let mut foundPent: libc::c_int = 0 as libc::c_int;
                let mut i_0: int64_t = 0 as libc::c_int as int64_t;
                while i_0 < maxSz {
                    if isPentagon(*gridDiskInternalOutput.offset(i_0 as isize)) != 0 {
                        foundPent = 1 as libc::c_int;
                        break;
                    } else {
                        i_0 += 1;
                    }
                }
                if foundPent == 0 {
                    printf(
                        b"NO C k=%d h=%llx\n\0" as *const u8 as *const libc::c_char,
                        k,
                        h,
                    );
                    (*testOutput).ret1ValidationFailures += 1;
                    return;
                }
            }
            free(gridDiskInternalOutput as *mut libc::c_void);
            free(gridDiskUnsafeOutput as *mut libc::c_void);
            free(gridDiskInternalDistances as *mut libc::c_void);
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn recursiveH3IndexToGeo(
    mut h: H3Index,
    mut res: libc::c_int,
    mut maxK: libc::c_int,
    mut testOutput: *mut TestOutput,
) {
    let mut d: libc::c_int = 0 as libc::c_int;
    while d < 7 as libc::c_int {
        h = h & !((7 as libc::c_int as uint64_t) << (15 as libc::c_int - res) * 3 as libc::c_int)
            | (d as uint64_t) << (15 as libc::c_int - res) * 3 as libc::c_int;
        if !(_isBaseCellPentagon(
            ((h & (127 as libc::c_int as uint64_t) << 45 as libc::c_int) >> 45 as libc::c_int)
                as libc::c_int,
        ) != 0
            && _h3LeadingNonZeroDigit(h) as libc::c_uint == 1 as libc::c_int as libc::c_uint)
        {
            if res
                == ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
                    as libc::c_int
            {
                doCell(h, maxK, testOutput);
            } else {
                recursiveH3IndexToGeo(h, res + 1 as libc::c_int, maxK, testOutput);
            }
        }
        d += 1;
    }
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    if argc != 2 as libc::c_int && argc != 3 as libc::c_int {
        fprintf(
            __stderrp,
            b"usage: %s resolution [maxK]\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    let mut res: libc::c_int = 0 as libc::c_int;
    if sscanf(
        *argv.offset(1 as libc::c_int as isize),
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut res as *mut libc::c_int,
    ) == 0
    {
        error(b"resolution must be an integer\0" as *const u8 as *const libc::c_char);
    }
    let mut maxK: libc::c_int = 5 as libc::c_int;
    if argc > 2 as libc::c_int {
        if sscanf(
            *argv.offset(2 as libc::c_int as isize),
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut maxK as *mut libc::c_int,
        ) == 0
        {
            error(b"maxK must be an integer\0" as *const u8 as *const libc::c_char);
        }
    }
    let mut testOutput: TestOutput = {
        let mut init = TestOutput {
            ret0: 0 as libc::c_int,
            ret0ValidationFailures: 0 as libc::c_int,
            ret1: 0 as libc::c_int,
            ret1ValidationFailures: 0 as libc::c_int,
            ret2: 0 as libc::c_int,
        };
        init
    };
    let mut bc: libc::c_int = 0 as libc::c_int;
    while bc < 122 as libc::c_int {
        let mut rootCell: H3Index = 35184372088831 as libc::c_ulonglong;
        rootCell = rootCell & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
            | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
        rootCell = rootCell & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
            | (bc as uint64_t) << 45 as libc::c_int;
        if res == 0 as libc::c_int {
            doCell(rootCell, maxK, &mut testOutput);
        } else {
            let mut rootRes: libc::c_int = ((rootCell
                & (15 as libc::c_ulonglong) << 52 as libc::c_int)
                >> 52 as libc::c_int) as libc::c_int;
            rootCell = rootCell & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
                | (res as uint64_t) << 52 as libc::c_int;
            recursiveH3IndexToGeo(rootCell, rootRes + 1 as libc::c_int, maxK, &mut testOutput);
        }
        bc += 1;
    }
    printf(
        b"ret0: %d\nret1: %d\nret2: %d\n\0" as *const u8 as *const libc::c_char,
        testOutput.ret0,
        testOutput.ret1,
        testOutput.ret2,
    );
    if testOutput.ret2 > 0 as libc::c_int
        || testOutput.ret0ValidationFailures != 0
        || testOutput.ret1ValidationFailures != 0
    {
        printf(
            b"FAILED\nfailed0: %d\nfailed1: %d\n\0" as *const u8 as *const libc::c_char,
            testOutput.ret0ValidationFailures,
            testOutput.ret1ValidationFailures,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
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
