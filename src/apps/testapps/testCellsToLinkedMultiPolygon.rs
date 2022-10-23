extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;
extern crate unsafe_h3lib_testapps_lib;
use ::libc;
extern "C" {

    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn gridDisk(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellsToLinkedMultiPolygon(
        h3Set: *const H3Index,
        numHexes: libc::c_int,
        out: *mut LinkedGeoPolygon,
    ) -> H3Error;
    fn destroyLinkedMultiPolygon(polygon: *mut LinkedGeoPolygon);
    fn cellToCenterChild(h: H3Index, childRes: libc::c_int, child: *mut H3Index) -> H3Error;
    fn countLinkedPolygons(polygon: *mut LinkedGeoPolygon) -> libc::c_int;
    fn countLinkedLoops(polygon: *mut LinkedGeoPolygon) -> libc::c_int;
    fn countLinkedCoords(loop_0: *mut LinkedGeoLoop) -> libc::c_int;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkedLatLng {
    pub vertex: LatLng,
    pub next: *mut LinkedLatLng,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkedGeoLoop {
    pub first: *mut LinkedLatLng,
    pub last: *mut LinkedLatLng,
    pub next: *mut LinkedGeoLoop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkedGeoPolygon {
    pub first: *mut LinkedGeoLoop,
    pub last: *mut LinkedGeoLoop,
    pub next: *mut LinkedGeoPolygon,
}
unsafe extern "C" fn runTests() {
    currentTestName = b"empty\0" as *const u8 as *const libc::c_char;
    let mut polygon: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    if cellsToLinkedMultiPolygon(std::ptr::null::<H3Index>(), 0, &mut polygon) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            28 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(((void *)0), 0, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            30 as libc::c_int,
            b"countLinkedLoops(&polygon) == 0\0" as *const u8 as *const libc::c_char,
            b"No loops added to polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon);
    currentTestName = b"singleHex\0" as *const u8 as *const libc::c_char;
    let mut polygon_0: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set: [H3Index; 1] = [0x890dab6220bffff as libc::c_long as H3Index];
    let mut numHexes: libc::c_int = (::core::mem::size_of::<[H3Index; 1]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set.as_mut_ptr(), numHexes, &mut polygon_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            41 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_0) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            43 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"1 loop added to polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_0.first) == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            45 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 6\0" as *const u8 as *const libc::c_char,
            b"6 coords added to loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_0);
    currentTestName = b"invalid\0" as *const u8 as *const libc::c_char;
    let mut polygon_1: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_0: [H3Index; 1] = [0xfffffffffffffff as libc::c_long as H3Index];
    let mut numHexes_0: libc::c_int = (::core::mem::size_of::<[H3Index; 1]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if !(cellsToLinkedMultiPolygon(set_0.as_mut_ptr(), numHexes_0, &mut polygon_1)
        == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            57 as libc::c_int,
            b"H3_EXPORT(cellsToLinkedMultiPolygon)( set, numHexes, &polygon) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"Invalid set fails\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"contiguous2\0" as *const u8 as *const libc::c_char;
    let mut polygon_2: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_1: [H3Index; 2] = [
        0x8928308291bffff as libc::c_long as H3Index,
        0x89283082957ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_1: libc::c_int = (::core::mem::size_of::<[H3Index; 2]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_1.as_mut_ptr(), numHexes_1, &mut polygon_2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            66 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_2) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            68 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"1 loop added to polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_2.first) == 10 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            70 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 10\0" as *const u8 as *const libc::c_char,
            b"All coords added to loop except 2 shared\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_2);
    currentTestName = b"nonContiguous2\0" as *const u8 as *const libc::c_char;
    let mut polygon_3: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_2: [H3Index; 2] = [
        0x8928308291bffff as libc::c_long as H3Index,
        0x89283082943ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_2: libc::c_int = (::core::mem::size_of::<[H3Index; 2]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_2.as_mut_ptr(), numHexes_2, &mut polygon_3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            84 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_3) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            86 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"2 polygons added\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_3) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            88 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"1 loop on the first polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_3.first) == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            90 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 6\0" as *const u8 as *const libc::c_char,
            b"All coords for one hex added to first loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(polygon_3.next) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            92 as libc::c_int,
            b"countLinkedLoops(polygon.next) == 1\0" as *const u8 as *const libc::c_char,
            b"Loop count on second polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*polygon_3.next).first) == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            94 as libc::c_int,
            b"countLinkedCoords(polygon.next->first) == 6\0" as *const u8 as *const libc::c_char,
            b"All coords for one hex added to second polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_3);
    currentTestName = b"contiguous3\0" as *const u8 as *const libc::c_char;
    let mut polygon_4: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_3: [H3Index; 3] = [
        0x8928308288bffff as libc::c_long as H3Index,
        0x892830828d7ffff as libc::c_long as H3Index,
        0x8928308289bffff as libc::c_long as H3Index,
    ];
    let mut numHexes_3: libc::c_int = (::core::mem::size_of::<[H3Index; 3]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_3.as_mut_ptr(), numHexes_3, &mut polygon_4) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            106 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_4) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            108 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"1 loop added to polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_4.first) == 12 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            110 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 12\0" as *const u8 as *const libc::c_char,
            b"All coords added to loop except 6 shared\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_4);
    currentTestName = b"hole\0" as *const u8 as *const libc::c_char;
    let mut polygon_5: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_4: [H3Index; 6] = [
        0x892830828c7ffff as libc::c_long as H3Index,
        0x892830828d7ffff as libc::c_long as H3Index,
        0x8928308289bffff as libc::c_long as H3Index,
        0x89283082813ffff as libc::c_long as H3Index,
        0x8928308288fffff as libc::c_long as H3Index,
        0x89283082883ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_4: libc::c_int = (::core::mem::size_of::<[H3Index; 6]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    cellsToLinkedMultiPolygon(set_4.as_mut_ptr(), numHexes_4, &mut polygon_5);
    if !(countLinkedLoops(&mut polygon_5) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            124 as libc::c_int,
            b"countLinkedLoops(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"2 loops added to polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_5.first) == 6 as libc::c_int * 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            126 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 6 * 3\0" as *const u8 as *const libc::c_char,
            b"All outer coords added to first loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*polygon_5.first).next) == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            128 as libc::c_int,
            b"countLinkedCoords(polygon.first->next) == 6\0" as *const u8 as *const libc::c_char,
            b"All inner coords added to second loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_5);
    currentTestName = b"pentagon\0" as *const u8 as *const libc::c_char;
    let mut polygon_6: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_5: [H3Index; 1] = [0x851c0003fffffff as libc::c_long as H3Index];
    let mut numHexes_5: libc::c_int = (::core::mem::size_of::<[H3Index; 1]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_5.as_mut_ptr(), numHexes_5, &mut polygon_6) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            139 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_6) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            141 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"1 loop added to polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_6.first) == 10 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            143 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 10\0" as *const u8 as *const libc::c_char,
            b"10 coords (distorted pentagon) added to loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_6);
    currentTestName = b"twoRing\0" as *const u8 as *const libc::c_char;
    let mut polygon_7: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_6: [H3Index; 19] = [
        0x8930062838bffff as libc::c_long as H3Index,
        0x8930062838fffff as libc::c_long as H3Index,
        0x89300628383ffff as libc::c_long as H3Index,
        0x8930062839bffff as libc::c_long as H3Index,
        0x893006283d7ffff as libc::c_long as H3Index,
        0x893006283c7ffff as libc::c_long as H3Index,
        0x89300628313ffff as libc::c_long as H3Index,
        0x89300628317ffff as libc::c_long as H3Index,
        0x893006283bbffff as libc::c_long as H3Index,
        0x89300628387ffff as libc::c_long as H3Index,
        0x89300628397ffff as libc::c_long as H3Index,
        0x89300628393ffff as libc::c_long as H3Index,
        0x89300628067ffff as libc::c_long as H3Index,
        0x8930062806fffff as libc::c_long as H3Index,
        0x893006283d3ffff as libc::c_long as H3Index,
        0x893006283c3ffff as libc::c_long as H3Index,
        0x893006283cfffff as libc::c_long as H3Index,
        0x8930062831bffff as libc::c_long as H3Index,
        0x89300628303ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_6: libc::c_int = (::core::mem::size_of::<[H3Index; 19]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_6.as_mut_ptr(), numHexes_6, &mut polygon_7) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            162 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_7) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            164 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"1 loop added to polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_7.first)
        == 6 as libc::c_int * (2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int))
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            166 as libc::c_int,
            b"countLinkedCoords(polygon.first) == (6 * (2 * 2 + 1))\0" as *const u8
                as *const libc::c_char,
            b"Expected number of coords added to loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_7);
    currentTestName = b"twoRingUnordered\0" as *const u8 as *const libc::c_char;
    let mut polygon_8: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_7: [H3Index; 19] = [
        0x89300628393ffff as libc::c_long as H3Index,
        0x89300628383ffff as libc::c_long as H3Index,
        0x89300628397ffff as libc::c_long as H3Index,
        0x89300628067ffff as libc::c_long as H3Index,
        0x89300628387ffff as libc::c_long as H3Index,
        0x893006283bbffff as libc::c_long as H3Index,
        0x89300628313ffff as libc::c_long as H3Index,
        0x893006283cfffff as libc::c_long as H3Index,
        0x89300628303ffff as libc::c_long as H3Index,
        0x89300628317ffff as libc::c_long as H3Index,
        0x8930062839bffff as libc::c_long as H3Index,
        0x8930062838bffff as libc::c_long as H3Index,
        0x8930062806fffff as libc::c_long as H3Index,
        0x8930062838fffff as libc::c_long as H3Index,
        0x893006283d3ffff as libc::c_long as H3Index,
        0x893006283c3ffff as libc::c_long as H3Index,
        0x8930062831bffff as libc::c_long as H3Index,
        0x893006283d7ffff as libc::c_long as H3Index,
        0x893006283c7ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_7: libc::c_int = (::core::mem::size_of::<[H3Index; 19]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_7.as_mut_ptr(), numHexes_7, &mut polygon_8) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            185 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_8) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            187 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"1 loop added to polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_8.first)
        == 6 as libc::c_int * (2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int))
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            189 as libc::c_int,
            b"countLinkedCoords(polygon.first) == (6 * (2 * 2 + 1))\0" as *const u8
                as *const libc::c_char,
            b"Expected number of coords added to loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_8);
    currentTestName = b"nestedDonut\0" as *const u8 as *const libc::c_char;
    let mut polygon_9: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_8: [H3Index; 24] = [
        0x89283082813ffff as libc::c_long as H3Index,
        0x8928308281bffff as libc::c_long as H3Index,
        0x8928308280bffff as libc::c_long as H3Index,
        0x8928308280fffff as libc::c_long as H3Index,
        0x89283082807ffff as libc::c_long as H3Index,
        0x89283082817ffff as libc::c_long as H3Index,
        0x8928308289bffff as libc::c_long as H3Index,
        0x892830828d7ffff as libc::c_long as H3Index,
        0x892830828c3ffff as libc::c_long as H3Index,
        0x892830828cbffff as libc::c_long as H3Index,
        0x89283082853ffff as libc::c_long as H3Index,
        0x89283082843ffff as libc::c_long as H3Index,
        0x8928308284fffff as libc::c_long as H3Index,
        0x8928308287bffff as libc::c_long as H3Index,
        0x89283082863ffff as libc::c_long as H3Index,
        0x89283082867ffff as libc::c_long as H3Index,
        0x8928308282bffff as libc::c_long as H3Index,
        0x89283082823ffff as libc::c_long as H3Index,
        0x89283082837ffff as libc::c_long as H3Index,
        0x892830828afffff as libc::c_long as H3Index,
        0x892830828a3ffff as libc::c_long as H3Index,
        0x892830828b3ffff as libc::c_long as H3Index,
        0x89283082887ffff as libc::c_long as H3Index,
        0x89283082883ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_8: libc::c_int = (::core::mem::size_of::<[H3Index; 24]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_8.as_mut_ptr(), numHexes_8, &mut polygon_9) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            209 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_9) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            213 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_9) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            215 as libc::c_int,
            b"countLinkedLoops(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count on first polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_9.first) == 42 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            217 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 42\0" as *const u8 as *const libc::c_char,
            b"Got expected big outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*polygon_9.first).next) == 30 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            219 as libc::c_int,
            b"countLinkedCoords(polygon.first->next) == 30\0" as *const u8 as *const libc::c_char,
            b"Got expected big inner loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(polygon_9.next) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            221 as libc::c_int,
            b"countLinkedLoops(polygon.next) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count on second polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*polygon_9.next).first) == 18 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            223 as libc::c_int,
            b"countLinkedCoords(polygon.next->first) == 18\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*(*polygon_9.next).first).next) == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            225 as libc::c_int,
            b"countLinkedCoords(polygon.next->first->next) == 6\0" as *const u8
                as *const libc::c_char,
            b"Got expected inner loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_9);
    currentTestName = b"nestedDonutTransmeridian\0" as *const u8 as *const libc::c_char;
    let mut polygon_10: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_9: [H3Index; 24] = [
        0x897eb5722c7ffff as libc::c_long as H3Index,
        0x897eb5722cfffff as libc::c_long as H3Index,
        0x897eb572257ffff as libc::c_long as H3Index,
        0x897eb57220bffff as libc::c_long as H3Index,
        0x897eb572203ffff as libc::c_long as H3Index,
        0x897eb572213ffff as libc::c_long as H3Index,
        0x897eb57266fffff as libc::c_long as H3Index,
        0x897eb5722d3ffff as libc::c_long as H3Index,
        0x897eb5722dbffff as libc::c_long as H3Index,
        0x897eb573537ffff as libc::c_long as H3Index,
        0x897eb573527ffff as libc::c_long as H3Index,
        0x897eb57225bffff as libc::c_long as H3Index,
        0x897eb57224bffff as libc::c_long as H3Index,
        0x897eb57224fffff as libc::c_long as H3Index,
        0x897eb57227bffff as libc::c_long as H3Index,
        0x897eb572263ffff as libc::c_long as H3Index,
        0x897eb572277ffff as libc::c_long as H3Index,
        0x897eb57223bffff as libc::c_long as H3Index,
        0x897eb572233ffff as libc::c_long as H3Index,
        0x897eb5722abffff as libc::c_long as H3Index,
        0x897eb5722bbffff as libc::c_long as H3Index,
        0x897eb572287ffff as libc::c_long as H3Index,
        0x897eb572283ffff as libc::c_long as H3Index,
        0x897eb57229bffff as libc::c_long as H3Index,
    ];
    let mut numHexes_9: libc::c_int = (::core::mem::size_of::<[H3Index; 24]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_9.as_mut_ptr(), numHexes_9, &mut polygon_10) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            245 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_10) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            249 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_10) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            251 as libc::c_int,
            b"countLinkedLoops(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count on first polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_10.first) == 18 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            253 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 18\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*polygon_10.first).next) == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            255 as libc::c_int,
            b"countLinkedCoords(polygon.first->next) == 6\0" as *const u8 as *const libc::c_char,
            b"Got expected inner loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(polygon_10.next) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            257 as libc::c_int,
            b"countLinkedLoops(polygon.next) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count on second polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*polygon_10.next).first) == 42 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            259 as libc::c_int,
            b"countLinkedCoords(polygon.next->first) == 42\0" as *const u8 as *const libc::c_char,
            b"Got expected big outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*(*polygon_10.next).first).next) == 30 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            261 as libc::c_int,
            b"countLinkedCoords(polygon.next->first->next) == 30\0" as *const u8
                as *const libc::c_char,
            b"Got expected big inner loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_10);
    currentTestName = b"contiguous2distorted\0" as *const u8 as *const libc::c_char;
    let mut polygon_11: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_10: [H3Index; 2] = [
        0x894cc5365afffff as libc::c_long as H3Index,
        0x894cc536537ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_10: libc::c_int = (::core::mem::size_of::<[H3Index; 2]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_10.as_mut_ptr(), numHexes_10, &mut polygon_11) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            272 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_11) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            274 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"1 loop added to polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_11.first) == 12 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            276 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 12\0" as *const u8 as *const libc::c_char,
            b"All coords added to loop except 2 shared\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_11);
    currentTestName = b"negativeHashedCoordinates\0" as *const u8 as *const libc::c_char;
    let mut polygon_12: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_11: [H3Index; 2] = [
        0x88ad36c547fffff as libc::c_long as H3Index,
        0x88ad36c467fffff as libc::c_long as H3Index,
    ];
    let mut numHexes_11: libc::c_int = (::core::mem::size_of::<[H3Index; 2]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if cellsToLinkedMultiPolygon(set_11.as_mut_ptr(), numHexes_11, &mut polygon_12) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            286 as libc::c_int,
            b"!(cellsToLinkedMultiPolygon(set, numHexes, &polygon))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_12) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            288 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"2 polygons added\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_12) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            290 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"1 loop on the first polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_12.first) == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            292 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 6\0" as *const u8 as *const libc::c_char,
            b"All coords for one hex added to first loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(polygon_12.next) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            294 as libc::c_int,
            b"countLinkedLoops(polygon.next) == 1\0" as *const u8 as *const libc::c_char,
            b"Loop count on second polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*polygon_12.next).first) == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            296 as libc::c_int,
            b"countLinkedCoords(polygon.next->first) == 6\0" as *const u8 as *const libc::c_char,
            b"All coords for one hex added to second polygon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_12);
    currentTestName = b"specificLeak\0" as *const u8 as *const libc::c_char;
    let mut polygon_13: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let mut set_12: [H3Index; 2] = [
        0xd60006d60000f100 as libc::c_ulong as H3Index,
        0x3c3c403c1300d668 as libc::c_long as H3Index,
    ];
    let mut numHexes_12: libc::c_int = (::core::mem::size_of::<[H3Index; 2]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if !(cellsToLinkedMultiPolygon(set_12.as_mut_ptr(), numHexes_12, &mut polygon_13)
        == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                as *const libc::c_char,
            309 as libc::c_int,
            b"H3_EXPORT(cellsToLinkedMultiPolygon)(set, numHexes, &polygon) == E_FAILED\0"
                as *const u8 as *const libc::c_char,
            b"invalid cells fail\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"gridDiskResolutions\0" as *const u8 as *const libc::c_char;
    let mut baseCell: H3Index = 0x8073fffffffffff as libc::c_long as H3Index;
    let mut origin: H3Index = baseCell;
    let mut indexes: [H3Index; 19] = [
        0 as libc::c_int as H3Index,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut numHexes_13: libc::c_int = (::core::mem::size_of::<[H3Index; 19]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    let mut res: libc::c_int = 1 as libc::c_int;
    while res < 15 as libc::c_int {
        if cellToCenterChild(baseCell, res, &mut origin) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                323 as libc::c_int,
                b"!(cellToCenterChild(baseCell, res, &origin))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if gridDisk(origin, 2 as libc::c_int, indexes.as_mut_ptr()) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                324 as libc::c_int,
                b"!(gridDisk(origin, 2, indexes))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut polygon_14: LinkedGeoPolygon = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        if cellsToLinkedMultiPolygon(indexes.as_mut_ptr(), numHexes_13, &mut polygon_14) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                329 as libc::c_int,
                b"!(cellsToLinkedMultiPolygon( indexes, numHexes, &polygon))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(countLinkedPolygons(&mut polygon_14) == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                330 as libc::c_int,
                b"countLinkedPolygons(&polygon) == 1\0" as *const u8 as *const libc::c_char,
                b"1 polygon added\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(countLinkedLoops(&mut polygon_14) == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                332 as libc::c_int,
                b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
                b"1 loop on the first polygon\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(countLinkedCoords(polygon_14.first) == 30 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                334 as libc::c_int,
                b"countLinkedCoords(polygon.first) == 30\0" as *const u8 as *const libc::c_char,
                b"All coords for all hexes added to first loop\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        destroyLinkedMultiPolygon(&mut polygon_14);
        res += 1;
    }
    currentTestName = b"gridDiskResolutionsPentagon\0" as *const u8 as *const libc::c_char;
    let mut baseCell_0: H3Index = 0x8031fffffffffff as libc::c_long as H3Index;
    let mut origin_0: H3Index = baseCell_0;
    let mut diskIndexes: [H3Index; 7] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0];
    let mut indexes_0: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    let mut res_0: libc::c_int = 1 as libc::c_int;
    while res_0 < 15 as libc::c_int {
        if cellToCenterChild(baseCell_0, res_0, &mut origin_0) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                354 as libc::c_int,
                b"!(cellToCenterChild(baseCell, res, &origin))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if gridDisk(origin_0, 1 as libc::c_int, diskIndexes.as_mut_ptr()) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                355 as libc::c_int,
                b"!(gridDisk(origin, 1, diskIndexes))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut j: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 7 as libc::c_int {
            if diskIndexes[i as usize] != 0 {
                let fresh0 = j;
                j = j + 1;
                indexes_0[fresh0 as usize] = diskIndexes[i as usize];
            }
            i += 1;
        }
        if !(j == 6 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                361 as libc::c_int,
                b"j == 6\0" as *const u8 as *const libc::c_char,
                b"Filled all 6 indexes\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut polygon_15: LinkedGeoPolygon = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        if cellsToLinkedMultiPolygon(indexes_0.as_mut_ptr(), 6 as libc::c_int, &mut polygon_15) != 0
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                366 as libc::c_int,
                b"!(cellsToLinkedMultiPolygon(indexes, 6, &polygon))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(countLinkedPolygons(&mut polygon_15) == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                367 as libc::c_int,
                b"countLinkedPolygons(&polygon) == 1\0" as *const u8 as *const libc::c_char,
                b"1 polygon added\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(countLinkedLoops(&mut polygon_15) == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                369 as libc::c_int,
                b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
                b"1 loop on the first polygon\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(countLinkedCoords(polygon_15.first) == 15 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellsToLinkedMultiPolygon.c\0" as *const u8
                    as *const libc::c_char,
                371 as libc::c_int,
                b"countLinkedCoords(polygon.first) == 15\0" as *const u8 as *const libc::c_char,
                b"All coords for all hexes added to first loop\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        destroyLinkedMultiPolygon(&mut polygon_15);
        res_0 += 1;
    }
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"cellsToLinkedMultiPolygon\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"cellsToLinkedMultiPolygon\0" as *const u8 as *const libc::c_char,
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
