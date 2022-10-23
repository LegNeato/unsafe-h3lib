use ::libc;
extern "C" {
    pub type __sFILEX;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn h3SetToVertexGraph(
        h3Set: *const H3Index,
        numHexes: libc::c_int,
        out: *mut VertexGraph,
    ) -> H3Error;
    fn destroyVertexGraph(graph: *mut VertexGraph);
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
unsafe extern "C" fn runTests() {
    currentTestName = b"empty\0" as *const u8 as *const libc::c_char;
    let mut graph: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    if h3SetToVertexGraph(std::ptr::null::<H3Index>as libc::c_int, &mut graph) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            27 as libc::c_int,
            b"!(h3SetToVertexGraph(((void *)0), 0, &graph))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
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
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            29 as libc::c_int,
            b"graph.size == 0\0" as *const u8 as *const libc::c_char,
            b"No edges added to graph\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph);
    currentTestName = b"singleHex\0" as *const u8 as *const libc::c_char;
    let mut graph_0: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    let mut set: [H3Index; 1] = [0x890dab6220bffff as libc::c_long as H3Index];
    let mut numHexes: libc::c_int = (::core::mem::size_of::<[H3Index; 1]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if h3SetToVertexGraph(set.as_mut_ptr(), numHexes, &mut graph_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            39 as libc::c_int,
            b"!(h3SetToVertexGraph(set, numHexes, &graph))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_0.size == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            40 as libc::c_int,
            b"graph.size == 6\0" as *const u8 as *const libc::c_char,
            b"All edges of one hex added to graph\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_0);
    currentTestName = b"nonContiguous2\0" as *const u8 as *const libc::c_char;
    let mut graph_1: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    let mut set_0: [H3Index; 2] = [
        0x8928308291bffff as libc::c_long as H3Index,
        0x89283082943ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_0: libc::c_int = (::core::mem::size_of::<[H3Index; 2]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if h3SetToVertexGraph(set_0.as_mut_ptr(), numHexes_0, &mut graph_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            50 as libc::c_int,
            b"!(h3SetToVertexGraph(set, numHexes, &graph))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_1.size == 12 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            52 as libc::c_int,
            b"graph.size == 12\0" as *const u8 as *const libc::c_char,
            b"All edges of two non-contiguous hexes added to graph\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_1);
    currentTestName = b"contiguous2\0" as *const u8 as *const libc::c_char;
    let mut graph_2: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    let mut set_1: [H3Index; 2] = [
        0x8928308291bffff as libc::c_long as H3Index,
        0x89283082957ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_1: libc::c_int = (::core::mem::size_of::<[H3Index; 2]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if h3SetToVertexGraph(set_1.as_mut_ptr(), numHexes_1, &mut graph_2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            62 as libc::c_int,
            b"!(h3SetToVertexGraph(set, numHexes, &graph))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_2.size == 10 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            63 as libc::c_int,
            b"graph.size == 10\0" as *const u8 as *const libc::c_char,
            b"All edges except 2 shared added to graph\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_2);
    currentTestName = b"contiguous2distorted\0" as *const u8 as *const libc::c_char;
    let mut graph_3: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    let mut set_2: [H3Index; 2] = [
        0x894cc5365afffff as libc::c_long as H3Index,
        0x894cc536537ffff as libc::c_long as H3Index,
    ];
    let mut numHexes_2: libc::c_int = (::core::mem::size_of::<[H3Index; 2]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if h3SetToVertexGraph(set_2.as_mut_ptr(), numHexes_2, &mut graph_3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            73 as libc::c_int,
            b"!(h3SetToVertexGraph(set, numHexes, &graph))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_3.size == 12 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            74 as libc::c_int,
            b"graph.size == 12\0" as *const u8 as *const libc::c_char,
            b"All edges except 2 shared added to graph\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_3);
    currentTestName = b"contiguous3\0" as *const u8 as *const libc::c_char;
    let mut graph_4: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    let mut set_3: [H3Index; 3] = [
        0x8928308288bffff as libc::c_long as H3Index,
        0x892830828d7ffff as libc::c_long as H3Index,
        0x8928308289bffff as libc::c_long as H3Index,
    ];
    let mut numHexes_3: libc::c_int = (::core::mem::size_of::<[H3Index; 3]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
        as libc::c_int;
    if h3SetToVertexGraph(set_3.as_mut_ptr(), numHexes_3, &mut graph_4) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            85 as libc::c_int,
            b"!(h3SetToVertexGraph(set, numHexes, &graph))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_4.size == 3 as libc::c_int * 4 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            87 as libc::c_int,
            b"graph.size == 3 * 4\0" as *const u8 as *const libc::c_char,
            b"All edges except 6 shared added to graph\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_4);
    currentTestName = b"hole\0" as *const u8 as *const libc::c_char;
    let mut graph_5: VertexGraph = VertexGraph {
        buckets: 0 as *mut *mut VertexNode,
        numBuckets: 0,
        size: 0,
        res: 0,
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
    if h3SetToVertexGraph(set_4.as_mut_ptr(), numHexes_4, &mut graph_5) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            99 as libc::c_int,
            b"!(h3SetToVertexGraph(set, numHexes, &graph))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(graph_5.size == 6 as libc::c_int * 3 as libc::c_int + 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testH3SetToVertexGraph.c\0" as *const u8
                as *const libc::c_char,
            101 as libc::c_int,
            b"graph.size == (6 * 3) + 6\0" as *const u8 as *const libc::c_char,
            b"All outer edges and inner hole edges added to graph\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyVertexGraph(&mut graph_5);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"h3SetToVertexGraph\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"h3SetToVertexGraph\0" as *const u8 as *const libc::c_char,
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
