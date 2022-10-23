extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;
extern crate unsafe_h3lib_testapps_lib;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    fn setGeoDegs(p: *mut LatLng, latDegs: libc::c_double, lngDegs: libc::c_double);
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn stringToH3(str: *const libc::c_char, out: *mut H3Index) -> H3Error;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn t_assertBoundary(h3: H3Index, b1: *const CellBoundary);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellBoundary {
    pub numVerts: libc::c_int,
    pub verts: [LatLng; 10],
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
unsafe extern "C" fn runTests() {
    currentTestName = b"latLngToCell_res\0" as *const u8 as *const libc::c_char;
    let mut h: H3Index = 0;
    let mut anywhere: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    if !(latLngToCell(&mut anywhere, -(1 as libc::c_int), &mut h)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
            b"H3_EXPORT(latLngToCell)(&anywhere, -1, &h) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"resolution below 0 is invalid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(latLngToCell(&mut anywhere, 16 as libc::c_int, &mut h)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            b"H3_EXPORT(latLngToCell)(&anywhere, 16, &h) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"resolution above 15 is invalid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"latLngToCell_coord\0" as *const u8 as *const libc::c_char;
    let mut h_0: H3Index = 0;
    let mut invalidLat: LatLng = {
        let mut init = LatLng {
            lat: ::core::f32::NAN as libc::c_double,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut invalidLng: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: ::core::f32::NAN as libc::c_double,
        };
        init
    };
    let mut invalidLatLng: LatLng = {
        let mut init = LatLng {
            lat: ::core::f32::INFINITY as libc::c_double,
            lng: -::core::f32::INFINITY as libc::c_double,
        };
        init
    };
    if !(latLngToCell(&mut invalidLat, 1 as libc::c_int, &mut h_0)
        == E_LATLNG_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            b"H3_EXPORT(latLngToCell)(&invalidLat, 1, &h) == E_LATLNG_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"invalid latitude is rejected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(latLngToCell(&mut invalidLng, 1 as libc::c_int, &mut h_0)
        == E_LATLNG_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            b"H3_EXPORT(latLngToCell)(&invalidLng, 1, &h) == E_LATLNG_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"invalid longitude is rejected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(latLngToCell(&mut invalidLatLng, 1 as libc::c_int, &mut h_0)
        == E_LATLNG_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            b"H3_EXPORT(latLngToCell)(&invalidLatLng, 1, &h) == E_LATLNG_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"coordinates with infinity are rejected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToBoundary_classIIIEdgeVertex\0" as *const u8 as *const libc::c_char;
    let mut hexes: [H3Index; 9] = [
        0x894cc5349b7ffff as libc::c_long as H3Index,
        0x894cc534d97ffff as libc::c_long as H3Index,
        0x894cc53682bffff as libc::c_long as H3Index,
        0x894cc536b17ffff as libc::c_long as H3Index,
        0x894cc53688bffff as libc::c_long as H3Index,
        0x894cead92cbffff as libc::c_long as H3Index,
        0x894cc536537ffff as libc::c_long as H3Index,
        0x894cc5acbabffff as libc::c_long as H3Index,
        0x894cc536597ffff as libc::c_long as H3Index,
    ];
    let mut numHexes: libc::c_int = (::core::mem::size_of::<[H3Index; 9]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    let mut b: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numHexes {
        cellToBoundary(hexes[i as usize], &mut b);
        if !(b.numVerts == 7 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int,
                b"b.numVerts == 7\0" as *const u8 as *const libc::c_char,
                b"got expected vertex count\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    currentTestName =
        b"cellToBoundary_classIIIEdgeVertex_exact\0" as *const u8 as *const libc::c_char;
    let mut h3: H3Index = 0;
    if stringToH3(
        b"894cc536537ffff\0" as *const u8 as *const libc::c_char,
        &mut h3,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            b"!(stringToH3(\"894cc536537ffff\", &h3))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut boundary: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    boundary.numVerts = 7 as libc::c_int;
    setGeoDegs(
        &mut *(boundary.verts)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        18.043333154f64,
        -66.27836523500002f64,
    );
    setGeoDegs(
        &mut *(boundary.verts)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        18.042238363f64,
        -66.27929062800001f64,
    );
    setGeoDegs(
        &mut *(boundary.verts)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        18.040818259f64,
        -66.27854193899998f64,
    );
    setGeoDegs(
        &mut *(boundary.verts)
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        18.040492975f64,
        -66.27686786700002f64,
    );
    setGeoDegs(
        &mut *(boundary.verts)
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize),
        18.041040385f64,
        -66.27640518300001f64,
    );
    setGeoDegs(
        &mut *(boundary.verts)
            .as_mut_ptr()
            .offset(5 as libc::c_int as isize),
        18.041757122f64,
        -66.27596711500001f64,
    );
    setGeoDegs(
        &mut *(boundary.verts)
            .as_mut_ptr()
            .offset(6 as libc::c_int as isize),
        18.043007860f64,
        -66.27669118199998f64,
    );
    t_assertBoundary(h3, &mut boundary);
    currentTestName = b"cellToBoundary_coslngConstrain\0" as *const u8 as *const libc::c_char;
    let mut h3_0: H3Index = 0x87dc6d364ffffff as libc::c_long as H3Index;
    let mut boundary_0: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    boundary_0.numVerts = 6 as libc::c_int;
    setGeoDegs(
        &mut *(boundary_0.verts)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        -52.0130533678236091f64,
        -34.6232931343713091f64,
    );
    setGeoDegs(
        &mut *(boundary_0.verts)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        -52.0041156384652012f64,
        -34.6096733160584549f64,
    );
    setGeoDegs(
        &mut *(boundary_0.verts)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
        -51.9929610229502472f64,
        -34.6165157145896387f64,
    );
    setGeoDegs(
        &mut *(boundary_0.verts)
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
        -51.9907410568096608f64,
        -34.6369680004259877f64,
    );
    setGeoDegs(
        &mut *(boundary_0.verts)
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize),
        -51.9996738734672377f64,
        -34.6505896528323660f64,
    );
    setGeoDegs(
        &mut *(boundary_0.verts)
            .as_mut_ptr()
            .offset(5 as libc::c_int as isize),
        -52.0108315681413629f64,
        -34.6437571897165668f64,
    );
    t_assertBoundary(h3_0, &mut boundary_0);
    currentTestName = b"cellToBoundary_failed\0" as *const u8 as *const libc::c_char;
    let mut h_1: H3Index = 0x87dc6d364ffffff as libc::c_long as H3Index;
    h_1 = h_1 & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        | ((122 as libc::c_int + 1 as libc::c_int) as uint64_t) << 45 as libc::c_int;
    let mut gb: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    if !(cellToBoundary(h_1, &mut gb) == E_CELL_INVALID as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            b"H3_EXPORT(cellToBoundary(h, &gb) == E_CELL_INVALID)\0" as *const u8
                as *const libc::c_char,
            b"cellToBoundary fails on invalid index\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToLatLngInvalid\0" as *const u8 as *const libc::c_char;
    let mut coord: LatLng = LatLng { lat: 0., lng: 0. };
    if !(cellToLatLng(0x7fffffffffffffff as libc::c_long as H3Index, &mut coord)
        == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            b"H3_EXPORT(cellToLatLng)(0x7fffffffffffffff, &coord) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"invalid cell gives error\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"version\0" as *const u8 as *const libc::c_char;
    if !(4 as libc::c_int >= 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            b"H3_VERSION_MAJOR >= 0\0" as *const u8 as *const libc::c_char,
            b"major version is set\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(0 as libc::c_int >= 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            b"H3_VERSION_MINOR >= 0\0" as *const u8 as *const libc::c_char,
            b"minor version is set\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(1 as libc::c_int >= 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Api.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            b"H3_VERSION_PATCH >= 0\0" as *const u8 as *const libc::c_char,
            b"patch version is set\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"h3Api\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"h3Api\0" as *const u8 as *const libc::c_char,
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
