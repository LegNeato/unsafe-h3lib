#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut currentTestName: *const libc::c_char;
    static mut currentSuiteName: *const libc::c_char;
    static mut globalTestCount: libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
    fn gridDisk(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn cellToVertex(origin: H3Index, vertexNum: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToVertexes(origin: H3Index, vertexes: *mut H3Index) -> H3Error;
    fn vertexToLatLng(vertex: H3Index, point: *mut LatLng) -> H3Error;
    fn isValidVertex(vertex: H3Index) -> libc::c_int;
    fn geoAlmostEqualThreshold(
        p1: *const LatLng,
        p2: *const LatLng,
        threshold: libc::c_double,
    ) -> bool;
    fn iterateAllIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    );
    fn iterateBaseCellIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
        baseCell: libc::c_int,
    );
    fn vertexNumForDirection(origin: H3Index, direction: Direction) -> libc::c_int;
    fn directionForVertexNum(origin: H3Index, vertexNum: libc::c_int) -> Direction;
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
unsafe extern "C" fn directionForVertexNum_symmetry_assertions(mut h3: H3Index) {
    let mut numVerts: libc::c_int = if isPentagon(h3) != 0 {
        5 as libc::c_int
    } else {
        6 as libc::c_int
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numVerts {
        let mut dir: Direction = directionForVertexNum(h3, i);
        let mut vertexNum: libc::c_int = vertexNumForDirection(h3, dir);
        if vertexNum != i {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8 as *const libc::c_char,
                31 as libc::c_int,
                b"vertexNum == i\0" as *const u8 as *const libc::c_char,
                b"directionForVertexNum and vertexNumForDirection are symmetrical\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}
unsafe extern "C" fn cellToVertex_point_assertions(mut h3: H3Index) {
    let mut gb: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    cellToBoundary(h3, &mut gb);
    let mut numVerts: libc::c_int = if isPentagon(h3) != 0 {
        5 as libc::c_int
    } else {
        6 as libc::c_int
    };
    if numVerts < gb.numVerts {
        return;
    }
    let mut coord: LatLng = LatLng { lat: 0., lng: 0. };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numVerts {
        let mut vertex: H3Index = 0;
        if cellToVertex(h3, i, &mut vertex) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8 as *const libc::c_char,
                46 as libc::c_int,
                b"!(cellToVertex(h3, i, &vertex))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if vertexToLatLng(vertex, &mut coord) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int,
                b"!(vertexToLatLng(vertex, &coord))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut almostEqual: libc::c_int = geoAlmostEqualThreshold(
            &mut *(gb.verts).as_mut_ptr().offset(i as isize),
            &mut coord,
            0.000001f64,
        ) as libc::c_int;
        if almostEqual == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int,
                b"almostEqual\0" as *const u8 as *const libc::c_char,
                b"Vertex coordinates match boundary vertex\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}
unsafe extern "C" fn cellToVertex_uniqueness_assertions(mut h3: H3Index) {
    let mut originVerts: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    if cellToVertexes(h3, originVerts.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            b"!(cellToVertexes(h3, originVerts))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut v1: libc::c_int = 0 as libc::c_int;
    while v1 < 6 as libc::c_int - 1 as libc::c_int {
        let mut v2: libc::c_int = v1 + 1 as libc::c_int;
        while v2 < 6 as libc::c_int {
            if originVerts[v1 as usize] == originVerts[v2 as usize] {
                if 0 as libc::c_int == 0 {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8
                            as *const libc::c_char,
                        61 as libc::c_int,
                        b"false\0" as *const u8 as *const libc::c_char,
                        b"vertex should be unique\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
            }
            v2 += 1;
        }
        v1 += 1;
    }
}
unsafe extern "C" fn cellToVertex_validity_assertions(mut h3: H3Index) {
    let mut verts: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    if cellToVertexes(h3, verts.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            b"!(cellToVertexes(h3, verts))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int - 1 as libc::c_int {
        if verts[i as usize] != 0 as libc::c_int as libc::c_ulonglong {
            if isValidVertex(verts[i as usize]) == 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8
                        as *const libc::c_char,
                    73 as libc::c_int,
                    b"H3_EXPORT(isValidVertex(verts[i]))\0" as *const u8 as *const libc::c_char,
                    b"vertex is valid\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
}
unsafe extern "C" fn cellToVertex_neighbor_assertions(mut h3: H3Index) {
    let mut neighbors: [H3Index; 7] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0];
    let mut originVerts: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    let mut neighborVerts: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    if gridDisk(h3, 1 as libc::c_int, neighbors.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            b"!(gridDisk(h3, 1, neighbors))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToVertexes(h3, originVerts.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            b"!(cellToVertexes(h3, originVerts))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        let mut neighbor: H3Index = neighbors[i as usize];
        if !(neighbor == 0 as libc::c_int as libc::c_ulonglong || neighbor == h3) {
            if cellToVertexes(neighbor, neighborVerts.as_mut_ptr()) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8
                        as *const libc::c_char,
                    89 as libc::c_int,
                    b"!(cellToVertexes(neighbor, neighborVerts))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut intersection: libc::c_int = 0 as libc::c_int;
            let mut v1: libc::c_int = 0 as libc::c_int;
            while v1 < 6 as libc::c_int {
                let mut v2: libc::c_int = 0 as libc::c_int;
                while v2 < 6 as libc::c_int {
                    if neighborVerts[v1 as usize] == originVerts[v2 as usize] {
                        intersection += 1;
                    }
                    v2 += 1;
                }
                v1 += 1;
            }
            if intersection != 2 as libc::c_int {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testVertexExhaustive.c\0" as *const u8
                        as *const libc::c_char,
                    102 as libc::c_int,
                    b"intersection == 2\0" as *const u8 as *const libc::c_char,
                    b"Neighbor shares 2 unique vertexes with origin\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
}
unsafe extern "C" fn runTests() {
    currentTestName = b"directionForVertexNum_symmetry\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(directionForVertexNum_symmetry_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(directionForVertexNum_symmetry_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(directionForVertexNum_symmetry_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        3 as libc::c_int,
        Some(directionForVertexNum_symmetry_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        4 as libc::c_int,
        Some(directionForVertexNum_symmetry_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"cellToVertex_point\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(cellToVertex_point_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(cellToVertex_point_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(cellToVertex_point_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        3 as libc::c_int,
        Some(cellToVertex_point_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        4 as libc::c_int,
        Some(cellToVertex_point_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateBaseCellIndexesAtRes(
        5 as libc::c_int,
        Some(cellToVertex_point_assertions as unsafe extern "C" fn(H3Index) -> ()),
        0 as libc::c_int,
    );
    iterateBaseCellIndexesAtRes(
        5 as libc::c_int,
        Some(cellToVertex_point_assertions as unsafe extern "C" fn(H3Index) -> ()),
        14 as libc::c_int,
    );
    iterateBaseCellIndexesAtRes(
        5 as libc::c_int,
        Some(cellToVertex_point_assertions as unsafe extern "C" fn(H3Index) -> ()),
        117 as libc::c_int,
    );
    currentTestName = b"cellToVertex_neighbors\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(cellToVertex_neighbor_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(cellToVertex_neighbor_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(cellToVertex_neighbor_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        3 as libc::c_int,
        Some(cellToVertex_neighbor_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        4 as libc::c_int,
        Some(cellToVertex_neighbor_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"cellToVertex_uniqueness\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(cellToVertex_uniqueness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(cellToVertex_uniqueness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(cellToVertex_uniqueness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        3 as libc::c_int,
        Some(cellToVertex_uniqueness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        4 as libc::c_int,
        Some(cellToVertex_uniqueness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"cellToVertex_validity\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(cellToVertex_validity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(cellToVertex_validity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(cellToVertex_validity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        3 as libc::c_int,
        Some(cellToVertex_validity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        4 as libc::c_int,
        Some(cellToVertex_validity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"Vertex\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"Vertex\0" as *const u8 as *const libc::c_char,
    );
    runTests();
    printf(
        b"\nDONE: %d assertions\n\0" as *const u8 as *const libc::c_char,
        globalTestCount,
    );
    0 as libc::c_int
}
pub fn main() {
    unsafe { ::std::process::exit(main_0()) }
}
