use ::libc;
extern "C" {
    pub type __sFILEX;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellAreaRads2(h: H3Index, out: *mut libc::c_double) -> H3Error;
    fn cellAreaKm2(h: H3Index, out: *mut libc::c_double) -> H3Error;
    fn cellAreaM2(h: H3Index, out: *mut libc::c_double) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
static mut areasKm2: [libc::c_double; 16] = [
    2.562182162955496e+06f64,
    4.476842017201860e+05f64,
    6.596162242711056e+04f64,
    9.228872919002590e+03f64,
    1.318694490797110e+03f64,
    1.879593512281298e+02f64,
    2.687164354763186e+01f64,
    3.840848847060638e+00f64,
    5.486939641329893e-01f64,
    7.838600808637444e-02f64,
    1.119834221989390e-02f64,
    1.599777169186614e-03f64,
    2.285390931423380e-04f64,
    3.264850232091780e-05f64,
    4.664070326136774e-06f64,
    6.662957615868888e-07f64,
];
unsafe extern "C" fn runTests() {
    currentTestName = b"specific_cell_area\0" as *const u8 as *const libc::c_char;
    let mut gc: LatLng = {
        let mut init = LatLng {
            lat: 0.0f64,
            lng: 0.0f64,
        };
        init
    };
    let mut res: libc::c_int = 0 as libc::c_int;
    while res <= 15 as libc::c_int - 1 as libc::c_int {
        let mut cell: H3Index = 0;
        if latLngToCell(&mut gc, res, &mut cell) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testH3CellArea.c\0" as *const u8
                    as *const libc::c_char,
                41 as libc::c_int,
                b"!(latLngToCell(&gc, res, &cell))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut area: libc::c_double = 0.;
        if cellAreaKm2(cell, &mut area) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testH3CellArea.c\0" as *const u8
                    as *const libc::c_char,
                43 as libc::c_int,
                b"!(cellAreaKm2(cell, &area))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(fabs(area - areasKm2[res as usize]) < 1e-8f64) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testH3CellArea.c\0" as *const u8
                    as *const libc::c_char,
                46 as libc::c_int,
                b"fabs(area - areasKm2[res]) < 1e-8\0" as *const u8 as *const libc::c_char,
                b"cell area should match expectation\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        res += 1;
    }
    currentTestName = b"cell_area_invalid\0" as *const u8 as *const libc::c_char;
    let mut invalid: H3Index = 0xffffffffffffffff as libc::c_ulong as H3Index;
    let mut area_0: libc::c_double = 0.;
    if !(cellAreaRads2(invalid, &mut area_0) == E_CELL_INVALID as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3CellArea.c\0" as *const u8
                as *const libc::c_char,
            54 as libc::c_int,
            b"H3_EXPORT(cellAreaRads2)(invalid, &area) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"cellAreaRads2 invalid input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellAreaKm2(invalid, &mut area_0) == E_CELL_INVALID as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3CellArea.c\0" as *const u8
                as *const libc::c_char,
            56 as libc::c_int,
            b"H3_EXPORT(cellAreaKm2)(invalid, &area) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"cellAreaKm2 invalid input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellAreaM2(invalid, &mut area_0) == E_CELL_INVALID as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3CellArea.c\0" as *const u8
                as *const libc::c_char,
            58 as libc::c_int,
            b"H3_EXPORT(cellAreaM2)(invalid, &area) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"cellAreaM2 invalid input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"h3CellArea\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"h3CellArea\0" as *const u8 as *const libc::c_char,
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
