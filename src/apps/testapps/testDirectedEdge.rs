extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;
extern crate unsafe_h3lib_testapps_lib;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn gridDisk(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn gridRingUnsafe(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn edgeLengthRads(edge: H3Index, length: *mut libc::c_double) -> H3Error;
    fn getPentagons(res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn areNeighborCells(origin: H3Index, destination: H3Index, out: *mut libc::c_int) -> H3Error;
    fn cellsToDirectedEdge(origin: H3Index, destination: H3Index, out: *mut H3Index) -> H3Error;
    fn isValidDirectedEdge(edge: H3Index) -> libc::c_int;
    fn getDirectedEdgeOrigin(edge: H3Index, out: *mut H3Index) -> H3Error;
    fn getDirectedEdgeDestination(edge: H3Index, out: *mut H3Index) -> H3Error;
    fn directedEdgeToCells(edge: H3Index, originDestination: *mut H3Index) -> H3Error;
    fn originToDirectedEdges(origin: H3Index, edges: *mut H3Index) -> H3Error;
    fn directedEdgeToBoundary(edge: H3Index, gb: *mut CellBoundary) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn geoAlmostEqual(p1: *const LatLng, p2: *const LatLng) -> bool;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type int64_t = libc::c_longlong;
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
static mut sfGeo: LatLng = {
    let mut init = LatLng {
        lat: 0.659966917655f64,
        lng: -2.1364398519396f64,
    };
    init
};
unsafe extern "C" fn runTests() {
    currentTestName = b"areNeighborCells\0" as *const u8 as *const libc::c_char;
    let mut sf: H3Index = 0;
    if latLngToCell(&mut sfGeo, 9 as libc::c_int, &mut sf) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
            b"!(latLngToCell(&sfGeo, 9, &sf))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut ring: [H3Index; 7] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0];
    if gridRingUnsafe(sf, 1 as libc::c_int, ring.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            b"!(gridRingUnsafe(sf, 1, ring))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut isNeighbor: libc::c_int = 0;
    if areNeighborCells(sf, sf, &mut isNeighbor) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            b"!(areNeighborCells(sf, sf, &isNeighbor))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if isNeighbor != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            b"!isNeighbor\0" as *const u8 as *const libc::c_char,
            b"an index does not neighbor itself\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut neighbors: libc::c_int = 0 as libc::c_int;
    let mut neighborsSize: int64_t = 0;
    if maxGridDiskSize(1 as libc::c_int, &mut neighborsSize) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            b"!(maxGridDiskSize(1, &neighborsSize))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < neighborsSize {
        if ring[i as usize] != 0 as libc::c_int as libc::c_ulonglong {
            if areNeighborCells(sf, ring[i as usize], &mut isNeighbor) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    50 as libc::c_int,
                    b"!(areNeighborCells(sf, ring[i], &isNeighbor))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if isNeighbor != 0 {
                neighbors += 1;
            }
        }
        i += 1;
    }
    if !(neighbors == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            b"neighbors == 6\0" as *const u8 as *const libc::c_char,
            b"got the expected number of neighbors from a k-ring of 1\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut largerRing: [H3Index; 19] = [
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
    if gridRingUnsafe(sf, 2 as libc::c_int, largerRing.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            b"!(gridRingUnsafe(sf, 2, largerRing))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    neighbors = 0 as libc::c_int;
    if maxGridDiskSize(2 as libc::c_int, &mut neighborsSize) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            b"!(maxGridDiskSize(2, &neighborsSize))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i_0: int64_t = 0 as libc::c_int as int64_t;
    while i_0 < neighborsSize {
        if largerRing[i_0 as usize] != 0 as libc::c_int as libc::c_ulonglong {
            if areNeighborCells(sf, largerRing[i_0 as usize], &mut isNeighbor) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    67 as libc::c_int,
                    b"!(areNeighborCells(sf, largerRing[i], &isNeighbor))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if isNeighbor != 0 {
                neighbors += 1;
            }
        }
        i_0 += 1;
    }
    if !(neighbors == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            b"neighbors == 0\0" as *const u8 as *const libc::c_char,
            b"got no neighbors, as expected, from a k-ring of 2\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut sfBroken: H3Index = sf;
    sfBroken = sfBroken & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
    if !(areNeighborCells(sf, sfBroken, &mut isNeighbor)
        == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            b"H3_EXPORT(areNeighborCells)(sf, sfBroken, &isNeighbor) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"broken H3Indexes can't be neighbors\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(areNeighborCells(sfBroken, sf, &mut isNeighbor)
        == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            b"H3_EXPORT(areNeighborCells)(sfBroken, sf, &isNeighbor) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"broken H3Indexes can't be neighbors (reversed)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut sfBigger: H3Index = 0;
    if latLngToCell(&mut sfGeo, 7 as libc::c_int, &mut sfBigger) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            b"!(latLngToCell(&sfGeo, 7, &sfBigger))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(areNeighborCells(sf, sfBigger, &mut isNeighbor)
        == E_RES_MISMATCH as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
            b"H3_EXPORT(areNeighborCells)(sf, sfBigger, &isNeighbor) == E_RES_MISMATCH\0"
                as *const u8 as *const libc::c_char,
            b"hexagons of different resolution can't be neighbors\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if areNeighborCells(
        ring[2 as libc::c_int as usize],
        ring[1 as libc::c_int as usize],
        &mut isNeighbor,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            b"!(areNeighborCells(ring[2], ring[1], &isNeighbor))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if isNeighbor == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            b"isNeighbor\0" as *const u8 as *const libc::c_char,
            b"hexagons in a ring are neighbors\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"areNeighborCells_invalid\0" as *const u8 as *const libc::c_char;
    let mut origin: H3Index = 0;
    setH3Index(
        &mut origin,
        5 as libc::c_int,
        0 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut dest: H3Index = origin;
    origin = origin
        & !((7 as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int)
        | (INVALID_DIGIT as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int;
    dest = dest
        & !((7 as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int)
        | (JK_AXES_DIGIT as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int;
    let mut out: libc::c_int = 0;
    if !(areNeighborCells(origin, dest, &mut out) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            b"H3_EXPORT(areNeighborCells)(origin, dest, &out) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"Invalid index digit origin is rejected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    setH3Index(
        &mut origin,
        5 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    dest = origin;
    origin = origin
        & !((7 as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int)
        | (K_AXES_DIGIT as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int;
    dest = dest
        & !((7 as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int)
        | (IK_AXES_DIGIT as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int;
    if !(areNeighborCells(origin, dest, &mut out) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            b"H3_EXPORT(areNeighborCells)(origin, dest, &out) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"Invalid k subsequence origin is rejected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    origin = origin
        & !((7 as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int)
        | (IK_AXES_DIGIT as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int;
    dest = dest
        & !((7 as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int)
        | (K_AXES_DIGIT as libc::c_int as uint64_t)
            << (15 as libc::c_int - 5 as libc::c_int) * 3 as libc::c_int;
    if !(areNeighborCells(origin, dest, &mut out) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            b"H3_EXPORT(areNeighborCells)(origin, dest, &out) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"Invalid k subsequence destination is rejected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellsToDirectedEdgeAndFriends\0" as *const u8 as *const libc::c_char;
    let mut sf_0: H3Index = 0;
    if latLngToCell(&mut sfGeo, 9 as libc::c_int, &mut sf_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            b"!(latLngToCell(&sfGeo, 9, &sf))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut ring_0: [H3Index; 7] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0];
    if gridRingUnsafe(sf_0, 1 as libc::c_int, ring_0.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            b"!(gridRingUnsafe(sf, 1, ring))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut sf2: H3Index = ring_0[0 as libc::c_int as usize];
    let mut edge: H3Index = 0;
    if cellsToDirectedEdge(sf_0, sf2, &mut edge) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            b"!(cellsToDirectedEdge(sf, sf2, &edge))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut edgeOrigin: H3Index = 0;
    if getDirectedEdgeOrigin(edge, &mut edgeOrigin) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            b"!(getDirectedEdgeOrigin(edge, &edgeOrigin))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(sf_0 == edgeOrigin) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            b"sf == edgeOrigin\0" as *const u8 as *const libc::c_char,
            b"can retrieve the origin from the edge\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut edgeDestination: H3Index = 0;
    if getDirectedEdgeDestination(edge, &mut edgeDestination) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            b"!(getDirectedEdgeDestination(edge, &edgeDestination))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(sf2 == edgeDestination) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            b"sf2 == edgeDestination\0" as *const u8 as *const libc::c_char,
            b"can retrieve the destination from the edge\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut originDestination: [H3Index; 2] = [0 as libc::c_int as H3Index, 0];
    if directedEdgeToCells(edge, originDestination.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            b"!(directedEdgeToCells(edge, originDestination))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(originDestination[0 as libc::c_int as usize] == sf_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            b"originDestination[0] == sf\0" as *const u8 as *const libc::c_char,
            b"got the origin first in the pair request\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(originDestination[1 as libc::c_int as usize] == sf2) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            b"originDestination[1] == sf2\0" as *const u8 as *const libc::c_char,
            b"got the destination last in the pair request\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(directedEdgeToCells(0 as libc::c_int as H3Index, originDestination.as_mut_ptr())
        == E_DIR_EDGE_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            b"H3_EXPORT(directedEdgeToCells)(0, originDestination) == E_DIR_EDGE_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"directedEdgeToCells fails for invalid edges\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut invalidEdge: H3Index = 0;
    setH3Index(
        &mut invalidEdge,
        1 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    invalidEdge = invalidEdge & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (INVALID_DIGIT as libc::c_int as uint64_t) << 56 as libc::c_int;
    invalidEdge = invalidEdge & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
    if !(directedEdgeToCells(invalidEdge, originDestination.as_mut_ptr())
        != E_SUCCESS as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int,
            b"H3_EXPORT(directedEdgeToCells)(invalidEdge, originDestination) != E_SUCCESS\0"
                as *const u8 as *const libc::c_char,
            b"directedEdgeToCells fails for invalid edges\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut largerRing_0: [H3Index; 19] = [
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
    if gridRingUnsafe(sf_0, 2 as libc::c_int, largerRing_0.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            b"!(gridRingUnsafe(sf, 2, largerRing))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut sf3: H3Index = largerRing_0[0 as libc::c_int as usize];
    let mut notEdge: H3Index = 0;
    if !(cellsToDirectedEdge(sf_0, sf3, &mut notEdge)
        == E_NOT_NEIGHBORS as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            b"H3_EXPORT(cellsToDirectedEdge)(sf, sf3, &notEdge) == E_NOT_NEIGHBORS\0" as *const u8
                as *const libc::c_char,
            b"Non-neighbors can't have edges\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"getDirectedEdgeOriginBadInput\0" as *const u8 as *const libc::c_char;
    let mut hexagon: H3Index = 0x891ea6d6533ffff as libc::c_long as H3Index;
    let mut out_0: H3Index = 0;
    if !(getDirectedEdgeOrigin(hexagon, &mut out_0)
        == E_DIR_EDGE_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            b"H3_EXPORT(getDirectedEdgeOrigin)(hexagon, &out) == E_DIR_EDGE_INVALID\0" as *const u8
                as *const libc::c_char,
            b"getting the origin from a hexagon index returns error\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getDirectedEdgeOrigin(0 as libc::c_int as H3Index, &mut out_0)
        == E_DIR_EDGE_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            b"H3_EXPORT(getDirectedEdgeOrigin)(0, &out) == E_DIR_EDGE_INVALID\0" as *const u8
                as *const libc::c_char,
            b"getting the origin from a null index returns error\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"getDirectedEdgeOriginBadInput\0" as *const u8 as *const libc::c_char;
    let mut sf_1: H3Index = 0;
    if latLngToCell(&mut sfGeo, 9 as libc::c_int, &mut sf_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            b"!(latLngToCell(&sfGeo, 9, &sf))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut ring_1: [H3Index; 7] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0];
    if gridRingUnsafe(sf_1, 1 as libc::c_int, ring_1.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            b"!(gridRingUnsafe(sf, 1, ring))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut sf2_0: H3Index = ring_1[0 as libc::c_int as usize];
    let mut edge_0: H3Index = 0;
    if cellsToDirectedEdge(sf_1, sf2_0, &mut edge_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            b"!(cellsToDirectedEdge(sf, sf2, &edge))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    edge_0 = edge_0 & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (INVALID_DIGIT as libc::c_int as uint64_t) << 56 as libc::c_int;
    let mut out_1: H3Index = 0;
    if !(getDirectedEdgeDestination(edge_0, &mut out_1) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            b"H3_EXPORT(getDirectedEdgeDestination)(edge, &out) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"Invalid directed edge fails\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"getDirectedEdgeDestination\0" as *const u8 as *const libc::c_char;
    let mut hexagon_0: H3Index = 0x891ea6d6533ffff as libc::c_long as H3Index;
    let mut out_2: H3Index = 0;
    if !(getDirectedEdgeDestination(hexagon_0, &mut out_2)
        == E_DIR_EDGE_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            b"H3_EXPORT(getDirectedEdgeDestination)(hexagon, &out) == E_DIR_EDGE_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"getting the destination from a hexagon index returns 0\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getDirectedEdgeDestination(0 as libc::c_int as H3Index, &mut out_2)
        == E_DIR_EDGE_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int,
            b"H3_EXPORT(getDirectedEdgeDestination)(0, &out) == E_DIR_EDGE_INVALID\0" as *const u8
                as *const libc::c_char,
            b"getting the destination from a null index returns 0\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellsToDirectedEdgeFromPentagon\0" as *const u8 as *const libc::c_char;
    let mut pentagons: [H3Index; 12] =
        [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut ring_2: [H3Index; 7] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0];
    let mut pentagon: H3Index = 0;
    let mut edge_1: H3Index = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    while res < 15 as libc::c_int {
        if getPentagons(res, pentagons.as_mut_ptr()) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                217 as libc::c_int,
                b"!(getPentagons(res, pentagons))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut p: libc::c_int = 0 as libc::c_int;
        while p < 12 as libc::c_int {
            pentagon = pentagons[p as usize];
            if gridDisk(pentagon, 1 as libc::c_int, ring_2.as_mut_ptr()) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    220 as libc::c_int,
                    b"!(gridDisk(pentagon, 1, ring))\0" as *const u8 as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < 7 as libc::c_int {
                let mut neighbor: H3Index = ring_2[i_1 as usize];
                if !(neighbor == pentagon || neighbor == 0 as libc::c_int as libc::c_ulonglong) {
                    if cellsToDirectedEdge(pentagon, neighbor, &mut edge_1) != 0 {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8
                                as *const libc::c_char,
                            226 as libc::c_int,
                            b"!(cellsToDirectedEdge( pentagon, neighbor, &edge))\0" as *const u8
                                as *const libc::c_char,
                            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    if isValidDirectedEdge(edge_1) == 0 {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8
                                as *const libc::c_char,
                            228 as libc::c_int,
                            b"H3_EXPORT(isValidDirectedEdge)(edge)\0" as *const u8
                                as *const libc::c_char,
                            b"pentagon-to-neighbor is a valid edge\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    if cellsToDirectedEdge(neighbor, pentagon, &mut edge_1) != 0 {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8
                                as *const libc::c_char,
                            230 as libc::c_int,
                            b"!(cellsToDirectedEdge( neighbor, pentagon, &edge))\0" as *const u8
                                as *const libc::c_char,
                            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    if isValidDirectedEdge(edge_1) == 0 {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8
                                as *const libc::c_char,
                            232 as libc::c_int,
                            b"H3_EXPORT(isValidDirectedEdge)(edge)\0" as *const u8
                                as *const libc::c_char,
                            b"neighbor-to-pentagon is a valid edge\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                }
                i_1 += 1;
            }
            p += 1;
        }
        res += 1;
    }
    currentTestName = b"isValidDirectedEdge\0" as *const u8 as *const libc::c_char;
    let mut sf_2: H3Index = 0;
    if latLngToCell(&mut sfGeo, 9 as libc::c_int, &mut sf_2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int,
            b"!(latLngToCell(&sfGeo, 9, &sf))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut ring_3: [H3Index; 7] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0];
    if gridRingUnsafe(sf_2, 1 as libc::c_int, ring_3.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
            b"!(gridRingUnsafe(sf, 1, ring))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut sf2_1: H3Index = ring_3[0 as libc::c_int as usize];
    let mut edge_2: H3Index = 0;
    if cellsToDirectedEdge(sf_2, sf2_1, &mut edge_2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int,
            b"!(cellsToDirectedEdge(sf, sf2, &edge))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(isValidDirectedEdge(edge_2) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int,
            b"H3_EXPORT(isValidDirectedEdge)(edge) == 1\0" as *const u8 as *const libc::c_char,
            b"edges validate correctly\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(isValidDirectedEdge(sf_2) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            250 as libc::c_int,
            b"H3_EXPORT(isValidDirectedEdge)(sf) == 0\0" as *const u8 as *const libc::c_char,
            b"hexagons do not validate\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut undirectedEdge: H3Index = edge_2;
    undirectedEdge = undirectedEdge & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (3 as libc::c_int as uint64_t) << 59 as libc::c_int;
    if !(isValidDirectedEdge(undirectedEdge) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            255 as libc::c_int,
            b"H3_EXPORT(isValidDirectedEdge)(undirectedEdge) == 0\0" as *const u8
                as *const libc::c_char,
            b"undirected edges do not validate\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagonWithReserved: H3Index = sf_2;
    hexagonWithReserved = hexagonWithReserved
        & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 56 as libc::c_int;
    if !(isValidDirectedEdge(hexagonWithReserved) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int,
            b"H3_EXPORT(isValidDirectedEdge)(hexagonWithReserved) == 0\0" as *const u8
                as *const libc::c_char,
            b"hexagons with reserved bits do not validate\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fakeEdge: H3Index = sf_2;
    fakeEdge = fakeEdge & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
    if !(isValidDirectedEdge(fakeEdge) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int,
            b"H3_EXPORT(isValidDirectedEdge)(fakeEdge) == 0\0" as *const u8 as *const libc::c_char,
            b"edges without an edge specified don't work\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut invalidEdge_0: H3Index = sf_2;
    invalidEdge_0 = invalidEdge_0 & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
    invalidEdge_0 = invalidEdge_0 & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (INVALID_DIGIT as libc::c_int as uint64_t) << 56 as libc::c_int;
    if !(isValidDirectedEdge(invalidEdge_0) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            270 as libc::c_int,
            b"H3_EXPORT(isValidDirectedEdge)(invalidEdge) == 0\0" as *const u8
                as *const libc::c_char,
            b"edges with an invalid edge specified don't work\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut pentagon_0: H3Index = 0x821c07fffffffff as libc::c_long as H3Index;
    let mut goodPentagonalEdge: H3Index = pentagon_0;
    goodPentagonalEdge = goodPentagonalEdge
        & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
    goodPentagonalEdge = goodPentagonalEdge
        & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 56 as libc::c_int;
    if !(isValidDirectedEdge(goodPentagonalEdge) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            277 as libc::c_int,
            b"H3_EXPORT(isValidDirectedEdge)(goodPentagonalEdge) == 1\0" as *const u8
                as *const libc::c_char,
            b"pentagonal edge validates\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut badPentagonalEdge: H3Index = goodPentagonalEdge;
    badPentagonalEdge = badPentagonalEdge & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 56 as libc::c_int;
    if !(isValidDirectedEdge(badPentagonalEdge) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int,
            b"H3_EXPORT(isValidDirectedEdge)(badPentagonalEdge) == 0\0" as *const u8
                as *const libc::c_char,
            b"missing pentagonal edge does not validate\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut highBitEdge: H3Index = edge_2;
    highBitEdge = highBitEdge & !((1 as libc::c_int as uint64_t) << 63 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 63 as libc::c_int;
    if !(isValidDirectedEdge(highBitEdge) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            287 as libc::c_int,
            b"H3_EXPORT(isValidDirectedEdge)(highBitEdge) == 0\0" as *const u8
                as *const libc::c_char,
            b"high bit set edge does not validate\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"originToDirectedEdges\0" as *const u8 as *const libc::c_char;
    let mut sf_3: H3Index = 0;
    if latLngToCell(&mut sfGeo, 9 as libc::c_int, &mut sf_3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int,
            b"!(latLngToCell(&sfGeo, 9, &sf))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut edges: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    if originToDirectedEdges(sf_3, edges.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
            b"!(originToDirectedEdges(sf, edges))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < 6 as libc::c_int {
        if !(isValidDirectedEdge(edges[i_2 as usize]) == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                298 as libc::c_int,
                b"H3_EXPORT(isValidDirectedEdge)(edges[i]) == 1\0" as *const u8
                    as *const libc::c_char,
                b"edge is an edge\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut origin_0: H3Index = 0;
        if getDirectedEdgeOrigin(edges[i_2 as usize], &mut origin_0) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                301 as libc::c_int,
                b"!(getDirectedEdgeOrigin(edges[i], &origin))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(sf_3 == origin_0) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
                b"sf == origin\0" as *const u8 as *const libc::c_char,
                b"origin is correct\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut destination: H3Index = 0;
        if getDirectedEdgeDestination(edges[i_2 as usize], &mut destination) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                305 as libc::c_int,
                b"!(getDirectedEdgeDestination(edges[i], &destination))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(sf_3 != destination) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
                b"sf != destination\0" as *const u8 as *const libc::c_char,
                b"destination is not origin\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_2 += 1;
    }
    currentTestName = b"getH3DirectedEdgesFromPentagon\0" as *const u8 as *const libc::c_char;
    let mut pentagon_1: H3Index = 0x821c07fffffffff as libc::c_long as H3Index;
    let mut edges_0: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    if originToDirectedEdges(pentagon_1, edges_0.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            313 as libc::c_int,
            b"!(originToDirectedEdges(pentagon, edges))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut missingEdgeCount: libc::c_int = 0 as libc::c_int;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < 6 as libc::c_int {
        if edges_0[i_3 as usize] == 0 as libc::c_int as libc::c_ulonglong {
            missingEdgeCount += 1;
        } else {
            if !(isValidDirectedEdge(edges_0[i_3 as usize]) == 1 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    321 as libc::c_int,
                    b"H3_EXPORT(isValidDirectedEdge)(edges[i]) == 1\0" as *const u8
                        as *const libc::c_char,
                    b"edge is an edge\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut origin_1: H3Index = 0;
            if getDirectedEdgeOrigin(edges_0[i_3 as usize], &mut origin_1) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    324 as libc::c_int,
                    b"!(getDirectedEdgeOrigin(edges[i], &origin))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if !(pentagon_1 == origin_1) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    325 as libc::c_int,
                    b"pentagon == origin\0" as *const u8 as *const libc::c_char,
                    b"origin is correct\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut destination_0: H3Index = 0;
            if getDirectedEdgeDestination(edges_0[i_3 as usize], &mut destination_0) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    328 as libc::c_int,
                    b"!(getDirectedEdgeDestination( edges[i], &destination))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if !(pentagon_1 != destination_0) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    329 as libc::c_int,
                    b"pentagon != destination\0" as *const u8 as *const libc::c_char,
                    b"destination is not origin\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_3 += 1;
    }
    if !(missingEdgeCount == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            333 as libc::c_int,
            b"missingEdgeCount == 1\0" as *const u8 as *const libc::c_char,
            b"Only one edge was deleted for the pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"directedEdgeToBoundary\0" as *const u8 as *const libc::c_char;
    let mut sf_4: H3Index = 0;
    let mut boundary: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut edgeBoundary: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut edges_1: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    let expectedVertices: [[libc::c_int; 2]; 6] = [
        [3 as libc::c_int, 4 as libc::c_int],
        [1 as libc::c_int, 2 as libc::c_int],
        [2 as libc::c_int, 3 as libc::c_int],
        [5 as libc::c_int, 0 as libc::c_int],
        [4 as libc::c_int, 5 as libc::c_int],
        [0 as libc::c_int, 1 as libc::c_int],
    ];
    let mut res_0: libc::c_int = 0 as libc::c_int;
    while res_0 < 15 as libc::c_int {
        if latLngToCell(&mut sfGeo, res_0, &mut sf_4) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int,
                b"!(latLngToCell(&sfGeo, res, &sf))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if cellToBoundary(sf_4, &mut boundary) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                347 as libc::c_int,
                b"!(cellToBoundary(sf, &boundary))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if originToDirectedEdges(sf_4, edges_1.as_mut_ptr()) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                348 as libc::c_int,
                b"!(originToDirectedEdges(sf, edges))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut i_4: libc::c_int = 0 as libc::c_int;
        while i_4 < 6 as libc::c_int {
            if directedEdgeToBoundary(edges_1[i_4 as usize], &mut edgeBoundary) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    352 as libc::c_int,
                    b"!(directedEdgeToBoundary(edges[i], &edgeBoundary))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if !(edgeBoundary.numVerts == 2 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                    354 as libc::c_int,
                    b"edgeBoundary.numVerts == 2\0" as *const u8 as *const libc::c_char,
                    b"Got the expected number of vertices back\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < edgeBoundary.numVerts {
                if !geoAlmostEqual(
                    &mut *(edgeBoundary.verts).as_mut_ptr().offset(j as isize),
                    &mut *(boundary.verts).as_mut_ptr().offset(
                        *(*expectedVertices.as_ptr().offset(i_4 as isize))
                            .as_ptr()
                            .offset(j as isize) as isize,
                    ),
                ) {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testDirectedEdge.c\0"
                            as *const u8 as *const libc::c_char,
                        359 as libc::c_int,
                        b"geoAlmostEqual(&edgeBoundary.verts[j], &boundary.verts[expectedVertices[i][j]])\0"
                            as *const u8 as *const libc::c_char,
                        b"Got expected vertex\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                j += 1;
            }
            i_4 += 1;
        }
        res_0 += 1;
    }
    currentTestName =
        b"directedEdgeToBoundaryPentagonClassIII\0" as *const u8 as *const libc::c_char;
    let mut pentagon_2: H3Index = 0;
    let mut boundary_0: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut edgeBoundary_0: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut edges_2: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    let expectedVertices_0: [[libc::c_int; 3]; 6] = [
        [
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            -(1 as libc::c_int),
        ],
        [2 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int],
        [4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int],
        [8 as libc::c_int, 9 as libc::c_int, 0 as libc::c_int],
        [6 as libc::c_int, 7 as libc::c_int, 8 as libc::c_int],
        [0 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int],
    ];
    let mut res_1: libc::c_int = 1 as libc::c_int;
    while res_1 < 15 as libc::c_int {
        setH3Index(&mut pentagon_2, res_1, 24 as libc::c_int, CENTER_DIGIT);
        if cellToBoundary(pentagon_2, &mut boundary_0) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                376 as libc::c_int,
                b"!(cellToBoundary(pentagon, &boundary))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if originToDirectedEdges(pentagon_2, edges_2.as_mut_ptr()) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                377 as libc::c_int,
                b"!(originToDirectedEdges(pentagon, edges))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut missingEdgeCount_0: libc::c_int = 0 as libc::c_int;
        let mut i_5: libc::c_int = 0 as libc::c_int;
        while i_5 < 6 as libc::c_int {
            if edges_2[i_5 as usize] == 0 as libc::c_int as libc::c_ulonglong {
                missingEdgeCount_0 += 1;
            } else {
                if directedEdgeToBoundary(edges_2[i_5 as usize], &mut edgeBoundary_0) != 0 {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testDirectedEdge.c\0" as *const u8
                            as *const libc::c_char,
                        385 as libc::c_int,
                        b"!(directedEdgeToBoundary( edges[i], &edgeBoundary))\0" as *const u8
                            as *const libc::c_char,
                        b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                if !(edgeBoundary_0.numVerts == 3 as libc::c_int) {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testDirectedEdge.c\0" as *const u8
                            as *const libc::c_char,
                        389 as libc::c_int,
                        b"edgeBoundary.numVerts == 3\0" as *const u8 as *const libc::c_char,
                        b"Got the expected number of vertices back for a Class III pentagon\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                let mut j_0: libc::c_int = 0 as libc::c_int;
                while j_0 < edgeBoundary_0.numVerts {
                    if !geoAlmostEqual(
                        &mut *(edgeBoundary_0.verts).as_mut_ptr().offset(j_0 as isize),
                        &mut *(boundary_0.verts).as_mut_ptr().offset(
                            *(*expectedVertices_0.as_ptr().offset(i_5 as isize))
                                .as_ptr()
                                .offset(j_0 as isize) as isize,
                        ),
                    ) {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testDirectedEdge.c\0"
                                as *const u8 as *const libc::c_char,
                            394 as libc::c_int,
                            b"geoAlmostEqual( &edgeBoundary.verts[j], &boundary.verts[expectedVertices[i][j]])\0"
                                as *const u8 as *const libc::c_char,
                            b"Got expected vertex\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    j_0 += 1;
                }
            }
            i_5 += 1;
        }
        if !(missingEdgeCount_0 == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                399 as libc::c_int,
                b"missingEdgeCount == 1\0" as *const u8 as *const libc::c_char,
                b"Only one edge was deleted for the pentagon\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        res_1 += 2 as libc::c_int;
    }
    currentTestName =
        b"directedEdgeToBoundaryPentagonClassII\0" as *const u8 as *const libc::c_char;
    let mut pentagon_3: H3Index = 0;
    let mut boundary_1: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut edgeBoundary_1: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut edges_3: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    let expectedVertices_1: [[libc::c_int; 3]; 6] = [
        [-(1 as libc::c_int), -(1 as libc::c_int), 0],
        [1 as libc::c_int, 2 as libc::c_int, 0],
        [2 as libc::c_int, 3 as libc::c_int, 0],
        [4 as libc::c_int, 0 as libc::c_int, 0],
        [3 as libc::c_int, 4 as libc::c_int, 0],
        [0 as libc::c_int, 1 as libc::c_int, 0],
    ];
    let mut res_2: libc::c_int = 0 as libc::c_int;
    while res_2 < 15 as libc::c_int {
        setH3Index(&mut pentagon_3, res_2, 24 as libc::c_int, CENTER_DIGIT);
        if cellToBoundary(pentagon_3, &mut boundary_1) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                414 as libc::c_int,
                b"!(cellToBoundary(pentagon, &boundary))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if originToDirectedEdges(pentagon_3, edges_3.as_mut_ptr()) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                415 as libc::c_int,
                b"!(originToDirectedEdges(pentagon, edges))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut missingEdgeCount_1: libc::c_int = 0 as libc::c_int;
        let mut i_6: libc::c_int = 0 as libc::c_int;
        while i_6 < 6 as libc::c_int {
            if edges_3[i_6 as usize] == 0 as libc::c_int as libc::c_ulonglong {
                missingEdgeCount_1 += 1;
            } else {
                if directedEdgeToBoundary(edges_3[i_6 as usize], &mut edgeBoundary_1) != 0 {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testDirectedEdge.c\0" as *const u8
                            as *const libc::c_char,
                        423 as libc::c_int,
                        b"!(directedEdgeToBoundary( edges[i], &edgeBoundary))\0" as *const u8
                            as *const libc::c_char,
                        b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                if !(edgeBoundary_1.numVerts == 2 as libc::c_int) {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testDirectedEdge.c\0" as *const u8
                            as *const libc::c_char,
                        427 as libc::c_int,
                        b"edgeBoundary.numVerts == 2\0" as *const u8 as *const libc::c_char,
                        b"Got the expected number of vertices back for a Class II pentagon\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                let mut j_1: libc::c_int = 0 as libc::c_int;
                while j_1 < edgeBoundary_1.numVerts {
                    if !geoAlmostEqual(
                        &mut *(edgeBoundary_1.verts).as_mut_ptr().offset(j_1 as isize),
                        &mut *(boundary_1.verts).as_mut_ptr().offset(
                            *(*expectedVertices_1.as_ptr().offset(i_6 as isize))
                                .as_ptr()
                                .offset(j_1 as isize) as isize,
                        ),
                    ) {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testDirectedEdge.c\0"
                                as *const u8 as *const libc::c_char,
                            432 as libc::c_int,
                            b"geoAlmostEqual( &edgeBoundary.verts[j], &boundary.verts[expectedVertices[i][j]])\0"
                                as *const u8 as *const libc::c_char,
                            b"Got expected vertex\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    j_1 += 1;
                }
            }
            i_6 += 1;
        }
        if !(missingEdgeCount_1 == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
                437 as libc::c_int,
                b"missingEdgeCount == 1\0" as *const u8 as *const libc::c_char,
                b"Only one edge was deleted for the pentagon\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        res_2 += 2 as libc::c_int;
    }
    currentTestName = b"directedEdgeToBoundary_invalid\0" as *const u8 as *const libc::c_char;
    let mut sf_5: H3Index = 0;
    if latLngToCell(&mut sfGeo, 9 as libc::c_int, &mut sf_5) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int,
            b"!(latLngToCell(&sfGeo, 9, &sf))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut invalidEdge_1: H3Index = sf_5;
    invalidEdge_1 = invalidEdge_1 & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
    let mut cb: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    if !(directedEdgeToBoundary(invalidEdge_1, &mut cb)
        == E_DIR_EDGE_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            449 as libc::c_int,
            b"H3_EXPORT(directedEdgeToBoundary)(invalidEdge, &cb) == E_DIR_EDGE_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"directedEdgeToBoundary fails on invalid edge direction\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut invalidEdge2: H3Index = sf_5;
    invalidEdge2 = invalidEdge2 & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 56 as libc::c_int;
    invalidEdge2 = invalidEdge2 & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        | ((122 as libc::c_int + 1 as libc::c_int) as uint64_t) << 45 as libc::c_int;
    invalidEdge2 = invalidEdge2 & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
    if !(directedEdgeToBoundary(invalidEdge2, &mut cb) != E_SUCCESS as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            457 as libc::c_int,
            b"H3_EXPORT(directedEdgeToBoundary)(invalidEdge2, &cb) != E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"directedEdgeToBoundary fails on invalid edge indexing digit\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"edgeLength_invalid\0" as *const u8 as *const libc::c_char;
    let mut length: libc::c_double = 0.;
    if !(edgeLengthRads(0 as libc::c_int as H3Index, &mut length)
        == E_DIR_EDGE_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            464 as libc::c_int,
            b"H3_EXPORT(edgeLengthRads)(0, &length) == E_DIR_EDGE_INVALID\0" as *const u8
                as *const libc::c_char,
            b"Invalid edge has zero length\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut zero: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut h3: H3Index = 0;
    if latLngToCell(&mut zero, 0 as libc::c_int, &mut h3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            467 as libc::c_int,
            b"!(latLngToCell(&zero, 0, &h3))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(edgeLengthRads(h3, &mut length) == E_DIR_EDGE_INVALID as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testDirectedEdge.c\0" as *const u8 as *const libc::c_char,
            469 as libc::c_int,
            b"H3_EXPORT(edgeLengthRads)(h3, &length) == E_DIR_EDGE_INVALID\0" as *const u8
                as *const libc::c_char,
            b"Non-edge (cell) has zero edge length\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"directedEdge\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"directedEdge\0" as *const u8 as *const libc::c_char,
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
