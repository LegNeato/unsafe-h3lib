extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn gridDiskDistances(
        origin: H3Index,
        k: libc::c_int,
        out: *mut H3Index,
        distances: *mut libc::c_int,
    ) -> H3Error;
    fn gridDistance(origin: H3Index, h3: H3Index, distance: *mut int64_t) -> H3Error;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn iterateAllIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    );
    fn iterateAllIndexesAtResPartial(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
        maxBaseCell: libc::c_int,
    );
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
static mut MAX_DISTANCES: [libc::c_int; 6] = [
    1 as libc::c_int,
    2 as libc::c_int,
    5 as libc::c_int,
    12 as libc::c_int,
    19 as libc::c_int,
    26 as libc::c_int,
];
unsafe extern "C" fn gridDistance_identity_assertions(mut h3: H3Index) {
    let mut distance: int64_t = 0;
    if gridDistance(h3, h3, &mut distance) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistanceExhaustive.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
            b"!(gridDistance(h3, h3, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(distance == 0 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistanceExhaustive.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            b"distance == 0\0" as *const u8 as *const libc::c_char,
            b"distance to self is 0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn gridDistance_gridDisk_assertions(mut h3: H3Index) {
    let mut r: libc::c_int =
        ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if !(r <= 5 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistanceExhaustive.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            b"r <= 5\0" as *const u8 as *const libc::c_char,
            b"resolution supported by test function (gridDisk)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut maxK: libc::c_int = MAX_DISTANCES[r as usize];
    let mut sz: int64_t = 0;
    if maxGridDiskSize(maxK, &mut sz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistanceExhaustive.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            b"!(maxGridDiskSize(maxK, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut neighbors: *mut H3Index = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let mut distances: *mut libc::c_int = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if gridDiskDistances(h3, maxK, neighbors, distances) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistanceExhaustive.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
            b"!(gridDiskDistances(h3, maxK, neighbors, distances))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < sz {
        if !(*neighbors.offset(i as isize) == 0 as libc::c_int as libc::c_ulonglong) {
            let mut calculatedDistance: int64_t = 0;
            let mut calculatedError: H3Error =
                gridDistance(h3, *neighbors.offset(i as isize), &mut calculatedDistance);
            if calculatedError == E_SUCCESS as libc::c_int as libc::c_uint {
                if !(calculatedDistance == *distances.offset(i as isize) as libc::c_longlong) {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testGridDistanceExhaustive.c\0" as *const u8
                            as *const libc::c_char,
                        69 as libc::c_int,
                        b"calculatedDistance == distances[i]\0" as *const u8 as *const libc::c_char,
                        b"gridDiskDistances matches gridDistance\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
            }
        }
        i += 1;
    }
    free(distances as *mut libc::c_void);
    free(neighbors as *mut libc::c_void);
}
unsafe extern "C" fn runTests() {
    currentTestName = b"gridDistance_identity\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(gridDistance_identity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(gridDistance_identity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(gridDistance_identity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"gridDistance_gridDisk\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(gridDistance_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(gridDistance_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(gridDistance_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtResPartial(
        3 as libc::c_int,
        Some(gridDistance_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
        27 as libc::c_int,
    );
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"gridDistance\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"gridDistance\0" as *const u8 as *const libc::c_char,
    );
    runTests();
    printf(
        b"\nDONE: %d assertions\n\0" as *const u8 as *const libc::c_char,
        globalTestCount,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
