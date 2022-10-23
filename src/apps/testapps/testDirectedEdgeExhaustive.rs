use ::libc;
extern "C" {
    pub type __sFILEX;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn areNeighborCells(origin: H3Index, destination: H3Index, out: *mut libc::c_int) -> H3Error;
    fn cellsToDirectedEdge(origin: H3Index, destination: H3Index, out: *mut H3Index) -> H3Error;
    fn isValidDirectedEdge(edge: H3Index) -> libc::c_int;
    fn getDirectedEdgeOrigin(edge: H3Index, out: *mut H3Index) -> H3Error;
    fn getDirectedEdgeDestination(edge: H3Index, out: *mut H3Index) -> H3Error;
    fn geoAlmostEqualThreshold(
        p1: *const LatLng,
        p2: *const LatLng,
        threshold: libc::c_double,
    ) -> bool;
    fn directedEdgeToBoundary(edge: H3Index, gb: *mut CellBoundary) -> H3Error;
    fn originToDirectedEdges(origin: H3Index, edges: *mut H3Index) -> H3Error;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn iterateAllIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    );
    fn iterateBaseCellIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
        baseCell: libc::c_int,
    );
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
    pub _extra: *mut __sFILEX,
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
unsafe extern "C" fn directedEdge_correctness_assertions(mut h3: H3Index) {
    let mut edges: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    let mut pentagon: libc::c_int = isPentagon(h3);
    if originToDirectedEdges(h3, edges.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0" as *const u8
                as *const libc::c_char,
            37 as libc::c_int,
            b"!(originToDirectedEdges(h3, edges))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if pentagon != 0 && i == 0 as libc::c_int {
            if !(edges[i as usize] == 0 as libc::c_int as libc::c_ulonglong) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    41 as libc::c_int,
                    b"edges[i] == H3_NULL\0" as *const u8 as *const libc::c_char,
                    b"last pentagon edge is empty\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        } else {
            if !(isValidDirectedEdge(edges[i as usize]) == 1 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    45 as libc::c_int,
                    b"H3_EXPORT(isValidDirectedEdge)(edges[i]) == 1\0" as *const u8
                        as *const libc::c_char,
                    b"edge is an edge\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut origin: H3Index = 0;
            if getDirectedEdgeOrigin(edges[i as usize], &mut origin) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    47 as libc::c_int,
                    b"!(getDirectedEdgeOrigin(edges[i], &origin))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if !(origin == h3) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    48 as libc::c_int,
                    b"origin == h3\0" as *const u8 as *const libc::c_char,
                    b"origin matches input origin\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut destination: H3Index = 0;
            if getDirectedEdgeDestination(edges[i as usize], &mut destination) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    52 as libc::c_int,
                    b"!(getDirectedEdgeDestination(edges[i], &destination))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut isNeighbor: libc::c_int = 0;
            if areNeighborCells(h3, destination, &mut isNeighbor) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    55 as libc::c_int,
                    b"!(areNeighborCells(h3, destination, &isNeighbor))\0" as *const u8
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
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    56 as libc::c_int,
                    b"isNeighbor\0" as *const u8 as *const libc::c_char,
                    b"destination is a neighbor\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
}
unsafe extern "C" fn directedEdge_boundary_assertions(mut h3: H3Index) {
    let mut edges: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    if originToDirectedEdges(h3, edges.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0" as *const u8
                as *const libc::c_char,
            62 as libc::c_int,
            b"!(originToDirectedEdges(h3, edges))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut destination: H3Index = 0;
    let mut revEdge: H3Index = 0;
    let mut edgeBoundary: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut revEdgeBoundary: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if !(edges[i as usize] == 0 as libc::c_int as libc::c_ulonglong) {
            if directedEdgeToBoundary(edges[i as usize], &mut edgeBoundary) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    71 as libc::c_int,
                    b"!(directedEdgeToBoundary(edges[i], &edgeBoundary))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if getDirectedEdgeDestination(edges[i as usize], &mut destination) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    73 as libc::c_int,
                    b"!(getDirectedEdgeDestination(edges[i], &destination))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if cellsToDirectedEdge(destination, h3, &mut revEdge) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    75 as libc::c_int,
                    b"!(cellsToDirectedEdge(destination, h3, &revEdge))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if directedEdgeToBoundary(revEdge, &mut revEdgeBoundary) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                    b"!(directedEdgeToBoundary(revEdge, &revEdgeBoundary))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if !(edgeBoundary.numVerts == revEdgeBoundary.numVerts) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    80 as libc::c_int,
                    b"edgeBoundary.numVerts == revEdgeBoundary.numVerts\0" as *const u8
                        as *const libc::c_char,
                    b"numVerts is equal for edge and reverse\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < edgeBoundary.numVerts {
                let mut almostEqual: libc::c_int = geoAlmostEqualThreshold(
                    &mut *(edgeBoundary.verts).as_mut_ptr().offset(j as isize),
                    &mut *(revEdgeBoundary.verts)
                        .as_mut_ptr()
                        .offset((revEdgeBoundary.numVerts - 1 as libc::c_int - j) as isize),
                    0.000001f64,
                ) as libc::c_int;
                if almostEqual == 0 {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"/Users/legnitto/src/h3/src/apps/testapps/testDirectedEdgeExhaustive.c\0"
                            as *const u8 as *const libc::c_char,
                        87 as libc::c_int,
                        b"almostEqual\0" as *const u8 as *const libc::c_char,
                        b"Got expected vertex\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                j += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn runTests() {
    currentTestName = b"directedEdge_correctness\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(directedEdge_correctness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(directedEdge_correctness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(directedEdge_correctness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        3 as libc::c_int,
        Some(directedEdge_correctness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        4 as libc::c_int,
        Some(directedEdge_correctness_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"directedEdge_boundary\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(directedEdge_boundary_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(directedEdge_boundary_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(directedEdge_boundary_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        3 as libc::c_int,
        Some(directedEdge_boundary_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        4 as libc::c_int,
        Some(directedEdge_boundary_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateBaseCellIndexesAtRes(
        5 as libc::c_int,
        Some(directedEdge_boundary_assertions as unsafe extern "C" fn(H3Index) -> ()),
        0 as libc::c_int,
    );
    iterateBaseCellIndexesAtRes(
        5 as libc::c_int,
        Some(directedEdge_boundary_assertions as unsafe extern "C" fn(H3Index) -> ()),
        14 as libc::c_int,
    );
    iterateBaseCellIndexesAtRes(
        5 as libc::c_int,
        Some(directedEdge_boundary_assertions as unsafe extern "C" fn(H3Index) -> ()),
        117 as libc::c_int,
    );
    iterateBaseCellIndexesAtRes(
        6 as libc::c_int,
        Some(directedEdge_boundary_assertions as unsafe extern "C" fn(H3Index) -> ()),
        14 as libc::c_int,
    );
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
