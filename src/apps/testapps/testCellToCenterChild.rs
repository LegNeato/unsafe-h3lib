extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn getResolution(h: H3Index) -> libc::c_int;
    fn cellToParent(h: H3Index, parentRes: libc::c_int, parent: *mut H3Index) -> H3Error;
    fn cellToCenterChild(h: H3Index, childRes: libc::c_int, child: *mut H3Index) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
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
unsafe extern "C" fn runTests() {
    let mut baseHex: H3Index = 0;
    let mut baseCentroid: LatLng = LatLng { lat: 0., lng: 0. };
    setH3Index(
        &mut baseHex,
        8 as libc::c_int,
        4 as libc::c_int,
        J_AXES_DIGIT,
    );
    cellToLatLng(baseHex, &mut baseCentroid);
    currentTestName = b"propertyTests\0" as *const u8 as *const libc::c_char;
    let mut res: libc::c_int = 0 as libc::c_int;
    while res <= 15 as libc::c_int - 1 as libc::c_int {
        let mut childRes: libc::c_int = res + 1 as libc::c_int;
        while childRes <= 15 as libc::c_int {
            let mut centroid: LatLng = LatLng { lat: 0., lng: 0. };
            let mut h3Index: H3Index = 0;
            if latLngToCell(&mut baseCentroid, res, &mut h3Index) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8
                        as *const libc::c_char,
                    35 as libc::c_int,
                    b"!(latLngToCell(&baseCentroid, res, &h3Index))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            cellToLatLng(h3Index, &mut centroid);
            let mut geoChild: H3Index = 0;
            if latLngToCell(&mut centroid, childRes, &mut geoChild) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8
                        as *const libc::c_char,
                    40 as libc::c_int,
                    b"!(latLngToCell(&centroid, childRes, &geoChild))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut centerChild: H3Index = 0;
            if cellToCenterChild(h3Index, childRes, &mut centerChild) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8
                        as *const libc::c_char,
                    43 as libc::c_int,
                    b"!(cellToCenterChild(h3Index, childRes, &centerChild))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if !(centerChild == geoChild) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8
                        as *const libc::c_char,
                    48 as libc::c_int,
                    b"centerChild == geoChild\0" as *const u8 as *const libc::c_char,
                    b"center child should be same as indexed centroid at child resolution\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if !(getResolution(centerChild) == childRes) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8
                        as *const libc::c_char,
                    50 as libc::c_int,
                    b"H3_EXPORT(getResolution)(centerChild) == childRes\0" as *const u8
                        as *const libc::c_char,
                    b"center child should have correct resolution\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut parent: H3Index = 0;
            if cellToParent(centerChild, res, &mut parent) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8
                        as *const libc::c_char,
                    53 as libc::c_int,
                    b"!(cellToParent(centerChild, res, &parent))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if !(parent == h3Index) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8
                        as *const libc::c_char,
                    56 as libc::c_int,
                    b"parent == h3Index\0" as *const u8 as *const libc::c_char,
                    b"parent at original resolution should be initial index\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            childRes += 1;
        }
        res += 1;
    }
    currentTestName = b"sameRes\0" as *const u8 as *const libc::c_char;
    let mut res_0: libc::c_int = getResolution(baseHex);
    let mut child: H3Index = 0;
    if cellToCenterChild(baseHex, res_0, &mut child) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            b"!(cellToCenterChild(baseHex, res, &child))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(child == baseHex) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            b"child == baseHex\0" as *const u8 as *const libc::c_char,
            b"center child at same resolution should return self\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"invalidInputs\0" as *const u8 as *const libc::c_char;
    let mut res_1: libc::c_int = getResolution(baseHex);
    let mut child_0: H3Index = 0;
    if !(cellToCenterChild(baseHex, res_1 - 1 as libc::c_int, &mut child_0)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            b"H3_EXPORT(cellToCenterChild)(baseHex, res - 1, &child) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"should fail at coarser resolution\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellToCenterChild(baseHex, -(1 as libc::c_int), &mut child_0)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            b"H3_EXPORT(cellToCenterChild)(baseHex, -1, &child) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"should fail for negative resolution\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellToCenterChild(baseHex, 15 as libc::c_int + 1 as libc::c_int, &mut child_0)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToCenterChild.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            b"H3_EXPORT(cellToCenterChild)(baseHex, MAX_H3_RES + 1, &child) == E_RES_DOMAIN\0"
                as *const u8 as *const libc::c_char,
            b"should fail beyond finest resolution\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"cellToCenterChild\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"cellToCenterChild\0" as *const u8 as *const libc::c_char,
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
