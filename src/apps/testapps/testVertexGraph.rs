use ::libc;
extern "C" {
    pub type __sFILEX;
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    static mut __stderrp: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
    fn setGeoDegs(p: *mut LatLng, latDegs: libc::c_double, lngDegs: libc::c_double);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn initVertexGraph(graph: *mut VertexGraph, numBuckets: libc::c_int, res: libc::c_int);
    fn destroyVertexGraph(graph: *mut VertexGraph);
    fn addVertexNode(
        graph: *mut VertexGraph,
        fromVtx: *const LatLng,
        toVtx: *const LatLng,
    ) -> *mut VertexNode;
    fn removeVertexNode(graph: *mut VertexGraph, node: *mut VertexNode) -> libc::c_int;
    fn findNodeForEdge(
        graph: *const VertexGraph,
        fromVtx: *const LatLng,
        toVtx: *const LatLng,
    ) -> *mut VertexNode;
    fn findNodeForVertex(graph: *const VertexGraph, fromVtx: *const LatLng) -> *mut VertexNode;
    fn firstVertexNode(graph: *const VertexGraph) -> *mut VertexNode;
    fn _hashVertex(vertex: *const LatLng, res: libc::c_int, numBuckets: libc::c_int) -> uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexNode {
    pub from: LatLng,
    pub to: LatLng,
    pub next: *mut VertexNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexGraph {
    pub buckets: *mut *mut VertexNode,
    pub numBuckets: libc::c_int,
    pub size: libc::c_int,
    pub res: libc::c_int,
}
static mut center: LatLng = LatLng { lat: 0., lng: 0. };
static mut vertex1: LatLng = LatLng { lat: 0., lng: 0. };
static mut vertex2: LatLng = LatLng { lat: 0., lng: 0. };
static mut vertex3: LatLng = LatLng { lat: 0., lng: 0. };
static mut vertex4: LatLng = LatLng { lat: 0., lng: 0. };
static mut vertex5: LatLng = LatLng { lat: 0., lng: 0. };
static mut vertex6: LatLng = LatLng { lat: 0., lng: 0. };
unsafe extern "C" fn runTests() {
    setGeoDegs(&mut center, 37.77362016769341f64, -122.41673772517154f64);
    setGeoDegs(&mut vertex1, 87.372002166f64, 166.160981117f64);
    setGeoDegs(&mut vertex2, 87.370101364f64, 166.160184306f64);
    setGeoDegs(&mut vertex3, 87.369088356f64, 166.196239997f64);
    setGeoDegs(&mut vertex4, 87.369975080f64, 166.233115768f64);
    setGeoDegs(
        &mut vertex5,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    setGeoDegs(
        &mut vertex6,
        -(10 as libc::c_int) as libc::c_double,
        -(10 as libc::c_int) as libc::c_double,
    );
    currentTestName = b"makeVertexGraph\0" as *const u8 as *const libc::c_char;
    let mut graph: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    initVertexGraph(&mut graph, 10 as libc::c_int, 9 as libc::c_int);
    if !(graph.numBuckets == 10 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            46 as libc::c_int,
            b"graph.numBuckets == 10\0" as *const u8 as *const libc::c_char,
            b"numBuckets set\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph.size == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            47 as libc::c_int,
            b"graph.size == 0\0" as *const u8 as *const libc::c_char,
            b"size set\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph);
    currentTestName = b"vertexHash\0" as *const u8 as *const libc::c_char;
    let mut centerIndex: H3Index = 0;
    let mut outline: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut hash1: uint32_t = 0;
    let mut hash2: uint32_t = 0;
    let mut numBuckets: libc::c_int = 1000 as libc::c_int;
    let mut res: libc::c_int = 0 as libc::c_int;
    while res < 11 as libc::c_int {
        if latLngToCell(&mut center, res, &mut centerIndex) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                    as *const libc::c_char,
                60 as libc::c_int,
                b"!(latLngToCell(&center, res, &centerIndex))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        cellToBoundary(centerIndex, &mut outline);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < outline.numVerts {
            hash1 = _hashVertex(
                &mut *(outline.verts).as_mut_ptr().offset(i as isize),
                res,
                numBuckets,
            );
            hash2 = _hashVertex(
                &mut *(outline.verts)
                    .as_mut_ptr()
                    .offset(((i + 1 as libc::c_int) % outline.numVerts) as isize),
                res,
                numBuckets,
            );
            if !(hash1 != hash2) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                        as *const libc::c_char,
                    66 as libc::c_int,
                    b"hash1 != hash2\0" as *const u8 as *const libc::c_char,
                    b"Hashes must not be equal\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            i += 1;
        }
        res += 1;
    }
    currentTestName = b"vertexHashNegative\0" as *const u8 as *const libc::c_char;
    let mut numBuckets_0: libc::c_int = 10 as libc::c_int;
    if !(_hashVertex(&mut vertex5, 5 as libc::c_int, numBuckets_0) < numBuckets_0 as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            74 as libc::c_int,
            b"_hashVertex(&vertex5, 5, numBuckets) < numBuckets\0" as *const u8
                as *const libc::c_char,
            b"zero vertex hashes correctly\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(_hashVertex(&mut vertex6, 5 as libc::c_int, numBuckets_0) < numBuckets_0 as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            76 as libc::c_int,
            b"_hashVertex(&vertex6, 5, numBuckets) < numBuckets\0" as *const u8
                as *const libc::c_char,
            b"negative coordinates vertex hashes correctly\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"addVertexNode\0" as *const u8 as *const libc::c_char;
    let mut graph_0: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    initVertexGraph(&mut graph_0, 10 as libc::c_int, 9 as libc::c_int);
    let mut node: *mut VertexNode = 0 as *mut VertexNode;
    let mut addedNode: *mut VertexNode = 0 as *mut VertexNode;
    addedNode = addVertexNode(&mut graph_0, &mut vertex1, &mut vertex2);
    node = findNodeForEdge(&mut graph_0, &mut vertex1, &mut vertex2);
    if node.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            88 as libc::c_int,
            b"node != NULL\0" as *const u8 as *const libc::c_char,
            b"Node found\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(node == addedNode) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            89 as libc::c_int,
            b"node == addedNode\0" as *const u8 as *const libc::c_char,
            b"Right node found\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_0.size == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            90 as libc::c_int,
            b"graph.size == 1\0" as *const u8 as *const libc::c_char,
            b"Graph size incremented\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    addedNode = addVertexNode(&mut graph_0, &mut vertex1, &mut vertex3);
    node = findNodeForEdge(&mut graph_0, &mut vertex1, &mut vertex3);
    if node.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            95 as libc::c_int,
            b"node != NULL\0" as *const u8 as *const libc::c_char,
            b"Node found after hash collision\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(node == addedNode) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            96 as libc::c_int,
            b"node == addedNode\0" as *const u8 as *const libc::c_char,
            b"Right node found\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_0.size == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            97 as libc::c_int,
            b"graph.size == 2\0" as *const u8 as *const libc::c_char,
            b"Graph size incremented\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    addedNode = addVertexNode(&mut graph_0, &mut vertex1, &mut vertex4);
    node = findNodeForEdge(&mut graph_0, &mut vertex1, &mut vertex4);
    if node.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            102 as libc::c_int,
            b"node != NULL\0" as *const u8 as *const libc::c_char,
            b"Node found after 2nd hash collision\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(node == addedNode) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            103 as libc::c_int,
            b"node == addedNode\0" as *const u8 as *const libc::c_char,
            b"Right node found\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_0.size == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            104 as libc::c_int,
            b"graph.size == 3\0" as *const u8 as *const libc::c_char,
            b"Graph size incremented\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    node = findNodeForEdge(&mut graph_0, &mut vertex1, &mut vertex2);
    addedNode = addVertexNode(&mut graph_0, &mut vertex1, &mut vertex2);
    if !(node == findNodeForEdge(&mut graph_0, &mut vertex1, &mut vertex2)) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            110 as libc::c_int,
            b"node == findNodeForEdge(&graph, &vertex1, &vertex2)\0" as *const u8
                as *const libc::c_char,
            b"Exact match did not change existing node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(node == addedNode) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            111 as libc::c_int,
            b"node == addedNode\0" as *const u8 as *const libc::c_char,
            b"Old node returned\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_0.size == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            112 as libc::c_int,
            b"graph.size == 3\0" as *const u8 as *const libc::c_char,
            b"Graph size was not changed\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_0);
    currentTestName = b"addVertexNodeDupe\0" as *const u8 as *const libc::c_char;
    let mut graph_1: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    initVertexGraph(&mut graph_1, 10 as libc::c_int, 9 as libc::c_int);
    let mut node_0: *mut VertexNode = 0 as *mut VertexNode;
    let mut addedNode_0: *mut VertexNode = 0 as *mut VertexNode;
    addedNode_0 = addVertexNode(&mut graph_1, &mut vertex1, &mut vertex2);
    node_0 = findNodeForEdge(&mut graph_1, &mut vertex1, &mut vertex2);
    if node_0.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            126 as libc::c_int,
            b"node != NULL\0" as *const u8 as *const libc::c_char,
            b"Node found\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(node_0 == addedNode_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            127 as libc::c_int,
            b"node == addedNode\0" as *const u8 as *const libc::c_char,
            b"Right node found\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_1.size == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            128 as libc::c_int,
            b"graph.size == 1\0" as *const u8 as *const libc::c_char,
            b"Graph size incremented\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    addedNode_0 = addVertexNode(&mut graph_1, &mut vertex1, &mut vertex2);
    if !(node_0 == addedNode_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            132 as libc::c_int,
            b"node == addedNode\0" as *const u8 as *const libc::c_char,
            b"addVertexNode returned the original node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_1.size == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            133 as libc::c_int,
            b"graph.size == 1\0" as *const u8 as *const libc::c_char,
            b"Graph size not incremented\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_1);
    currentTestName = b"findNodeForEdge\0" as *const u8 as *const libc::c_char;
    let mut graph_2: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    initVertexGraph(&mut graph_2, 10 as libc::c_int, 9 as libc::c_int);
    let mut node_1: *mut VertexNode = 0 as *mut VertexNode;
    node_1 = findNodeForEdge(&mut graph_2, &mut vertex1, &mut vertex2);
    if !node_1.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            146 as libc::c_int,
            b"node == NULL\0" as *const u8 as *const libc::c_char,
            b"Node lookup failed correctly for empty graph\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    addVertexNode(&mut graph_2, &mut vertex1, &mut vertex2);
    node_1 = findNodeForEdge(&mut graph_2, &mut vertex3, &mut vertex2);
    if !node_1.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            153 as libc::c_int,
            b"node == NULL\0" as *const u8 as *const libc::c_char,
            b"Node lookup failed correctly for different hash\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    node_1 = findNodeForEdge(&mut graph_2, &mut vertex1, &mut vertex3);
    if !node_1.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            158 as libc::c_int,
            b"node == NULL\0" as *const u8 as *const libc::c_char,
            b"Node lookup failed correctly for hash collision\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    addVertexNode(&mut graph_2, &mut vertex1, &mut vertex4);
    node_1 = findNodeForEdge(&mut graph_2, &mut vertex1, &mut vertex3);
    if !node_1.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            165 as libc::c_int,
            b"node == NULL\0" as *const u8 as *const libc::c_char,
            b"Node lookup failed correctly for collision w/iteration\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_2);
    currentTestName = b"findNodeForVertex\0" as *const u8 as *const libc::c_char;
    let mut graph_3: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    initVertexGraph(&mut graph_3, 10 as libc::c_int, 9 as libc::c_int);
    let mut node_2: *mut VertexNode = 0 as *mut VertexNode;
    node_2 = findNodeForVertex(&mut graph_3, &mut vertex1);
    if !node_2.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            177 as libc::c_int,
            b"node == NULL\0" as *const u8 as *const libc::c_char,
            b"Node lookup failed correctly for empty graph\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    addVertexNode(&mut graph_3, &mut vertex1, &mut vertex2);
    node_2 = findNodeForVertex(&mut graph_3, &mut vertex1);
    if node_2.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            182 as libc::c_int,
            b"node != NULL\0" as *const u8 as *const libc::c_char,
            b"Node lookup succeeded for correct node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    node_2 = findNodeForVertex(&mut graph_3, &mut vertex3);
    if !node_2.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            186 as libc::c_int,
            b"node == NULL\0" as *const u8 as *const libc::c_char,
            b"Node lookup failed correctly for different node\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_3);
    currentTestName = b"removeVertexNode\0" as *const u8 as *const libc::c_char;
    let mut graph_4: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    initVertexGraph(&mut graph_4, 10 as libc::c_int, 9 as libc::c_int);
    let mut node_3: *mut VertexNode = 0 as *mut VertexNode;
    let mut success: libc::c_int = 0;
    node_3 = addVertexNode(&mut graph_4, &mut vertex1, &mut vertex2);
    success = (removeVertexNode(&mut graph_4, node_3) == 0 as libc::c_int) as libc::c_int;
    if success == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            201 as libc::c_int,
            b"success\0" as *const u8 as *const libc::c_char,
            b"Removal successful\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(findNodeForVertex(&mut graph_4, &mut vertex1)).is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            203 as libc::c_int,
            b"findNodeForVertex(&graph, &vertex1) == NULL\0" as *const u8 as *const libc::c_char,
            b"Node lookup cannot find node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_4.size == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            204 as libc::c_int,
            b"graph.size == 0\0" as *const u8 as *const libc::c_char,
            b"Graph size decremented\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    addVertexNode(&mut graph_4, &mut vertex1, &mut vertex2);
    node_3 = addVertexNode(&mut graph_4, &mut vertex1, &mut vertex3);
    success = (removeVertexNode(&mut graph_4, node_3) == 0 as libc::c_int) as libc::c_int;
    if success == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            211 as libc::c_int,
            b"success\0" as *const u8 as *const libc::c_char,
            b"Removal successful\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(findNodeForEdge(&mut graph_4, &mut vertex1, &mut vertex3)).is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            213 as libc::c_int,
            b"findNodeForEdge(&graph, &vertex1, &vertex3) == NULL\0" as *const u8
                as *const libc::c_char,
            b"Node lookup cannot find node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !((*findNodeForEdge(&mut graph_4, &mut vertex1, &mut vertex2)).next).is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            215 as libc::c_int,
            b"findNodeForEdge(&graph, &vertex1, &vertex2)->next == NULL\0" as *const u8
                as *const libc::c_char,
            b"Base bucket node not pointing to node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_4.size == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            216 as libc::c_int,
            b"graph.size == 1\0" as *const u8 as *const libc::c_char,
            b"Graph size decremented\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    node_3 = findNodeForVertex(&mut graph_4, &mut vertex1);
    if !(removeVertexNode(&mut graph_4, node_3) == 0 as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testVertexGraph.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int,
            b"removeVertexNode(&graph, node) == 0\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    node_3 = addVertexNode(&mut graph_4, &mut vertex1, &mut vertex2);
    addVertexNode(&mut graph_4, &mut vertex1, &mut vertex3);
    success = (removeVertexNode(&mut graph_4, node_3) == 0 as libc::c_int) as libc::c_int;
    if success == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            227 as libc::c_int,
            b"success\0" as *const u8 as *const libc::c_char,
            b"Removal successful\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(findNodeForEdge(&mut graph_4, &mut vertex1, &mut vertex2)).is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            229 as libc::c_int,
            b"findNodeForEdge(&graph, &vertex1, &vertex2) == NULL\0" as *const u8
                as *const libc::c_char,
            b"Node lookup cannot find node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if (findNodeForEdge(&mut graph_4, &mut vertex1, &mut vertex3)).is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            231 as libc::c_int,
            b"findNodeForEdge(&graph, &vertex1, &vertex3) != NULL\0" as *const u8
                as *const libc::c_char,
            b"Node lookup can find previous end of list\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !((*findNodeForEdge(&mut graph_4, &mut vertex1, &mut vertex3)).next).is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            233 as libc::c_int,
            b"findNodeForEdge(&graph, &vertex1, &vertex3)->next == NULL\0" as *const u8
                as *const libc::c_char,
            b"Base bucket node not pointing to node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_4.size == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            234 as libc::c_int,
            b"graph.size == 1\0" as *const u8 as *const libc::c_char,
            b"Graph size decremented\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    node_3 = findNodeForVertex(&mut graph_4, &mut vertex1);
    if !(removeVertexNode(&mut graph_4, node_3) == 0 as libc::c_int) as libc::c_int as libc::c_long
        != 0
    {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testVertexGraph.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            b"removeVertexNode(&graph, node) == 0\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    addVertexNode(&mut graph_4, &mut vertex1, &mut vertex2);
    node_3 = addVertexNode(&mut graph_4, &mut vertex1, &mut vertex3);
    addVertexNode(&mut graph_4, &mut vertex1, &mut vertex4);
    success = (removeVertexNode(&mut graph_4, node_3) == 0 as libc::c_int) as libc::c_int;
    if success == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            246 as libc::c_int,
            b"success\0" as *const u8 as *const libc::c_char,
            b"Removal successful\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(findNodeForEdge(&mut graph_4, &mut vertex1, &mut vertex3)).is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            248 as libc::c_int,
            b"findNodeForEdge(&graph, &vertex1, &vertex3) == NULL\0" as *const u8
                as *const libc::c_char,
            b"Node lookup cannot find node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if (findNodeForEdge(&mut graph_4, &mut vertex1, &mut vertex4)).is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            250 as libc::c_int,
            b"findNodeForEdge(&graph, &vertex1, &vertex4) != NULL\0" as *const u8
                as *const libc::c_char,
            b"Node lookup can find previous end of list\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_4.size == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            251 as libc::c_int,
            b"graph.size == 2\0" as *const u8 as *const libc::c_char,
            b"Graph size decremented\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    node_3 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<VertexNode>() as libc::c_ulong,
    ) as *mut VertexNode;
    success = (removeVertexNode(&mut graph_4, node_3) == 0 as libc::c_int) as libc::c_int;
    if success != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            257 as libc::c_int,
            b"!success\0" as *const u8 as *const libc::c_char,
            b"Removal of non-existent node fails\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_4.size == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            258 as libc::c_int,
            b"graph.size == 2\0" as *const u8 as *const libc::c_char,
            b"Graph size unchanged\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(node_3 as *mut libc::c_void);
    destroyVertexGraph(&mut graph_4);
    currentTestName = b"firstVertexNode\0" as *const u8 as *const libc::c_char;
    let mut graph_5: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    initVertexGraph(&mut graph_5, 10 as libc::c_int, 9 as libc::c_int);
    let mut node_4: *mut VertexNode = 0 as *mut VertexNode;
    let mut addedNode_1: *mut VertexNode = 0 as *mut VertexNode;
    node_4 = firstVertexNode(&mut graph_5);
    if !node_4.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            272 as libc::c_int,
            b"node == NULL\0" as *const u8 as *const libc::c_char,
            b"No node found for empty graph\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    addedNode_1 = addVertexNode(&mut graph_5, &mut vertex1, &mut vertex2);
    node_4 = firstVertexNode(&mut graph_5);
    if !(node_4 == addedNode_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            277 as libc::c_int,
            b"node == addedNode\0" as *const u8 as *const libc::c_char,
            b"Node found\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_5);
    currentTestName = b"destroyEmptyVertexGraph\0" as *const u8 as *const libc::c_char;
    let mut graph_6: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    initVertexGraph(&mut graph_6, 10 as libc::c_int, 9 as libc::c_int);
    destroyVertexGraph(&mut graph_6);
    currentTestName = b"singleBucketVertexGraph\0" as *const u8 as *const libc::c_char;
    let mut graph_7: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    initVertexGraph(&mut graph_7, 1 as libc::c_int, 9 as libc::c_int);
    let mut node_5: *mut VertexNode = 0 as *mut VertexNode;
    if !(graph_7.numBuckets == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            293 as libc::c_int,
            b"graph.numBuckets == 1\0" as *const u8 as *const libc::c_char,
            b"1 bucket created\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    node_5 = firstVertexNode(&mut graph_7);
    if !node_5.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            296 as libc::c_int,
            b"node == NULL\0" as *const u8 as *const libc::c_char,
            b"No node found for empty graph\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    node_5 = addVertexNode(&mut graph_7, &mut vertex1, &mut vertex2);
    if node_5.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            299 as libc::c_int,
            b"node != NULL\0" as *const u8 as *const libc::c_char,
            b"Node added\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(firstVertexNode(&mut graph_7) == node_5) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            300 as libc::c_int,
            b"firstVertexNode(&graph) == node\0" as *const u8 as *const libc::c_char,
            b"First node is node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    addVertexNode(&mut graph_7, &mut vertex2, &mut vertex3);
    addVertexNode(&mut graph_7, &mut vertex3, &mut vertex4);
    if !(firstVertexNode(&mut graph_7) == node_5) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            304 as libc::c_int,
            b"firstVertexNode(&graph) == node\0" as *const u8 as *const libc::c_char,
            b"First node is still node\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_7.size == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            305 as libc::c_int,
            b"graph.size == 3\0" as *const u8 as *const libc::c_char,
            b"Graph size updated\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_7);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"vertexGraph\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"vertexGraph\0" as *const u8 as *const libc::c_char,
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
