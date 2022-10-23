use ::libc;
extern "C" {
    fn geoAlmostEqual(p1: *const LatLng, p2: *const LatLng) -> bool;
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn test_prefix_malloc(size: size_t) -> *mut libc::c_void;
    fn test_prefix_calloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn test_prefix_free(ptr: *mut libc::c_void);
}
pub type uint32_t = libc::c_uint;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
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
#[no_mangle]
pub unsafe extern "C" fn initVertexGraph(
    mut graph: *mut VertexGraph,
    mut numBuckets: libc::c_int,
    mut res: libc::c_int,
) {
    if numBuckets > 0 as libc::c_int {
        (*graph).buckets = test_prefix_calloc(
            numBuckets as size_t,
            ::core::mem::size_of::<*mut VertexNode>() as libc::c_ulong,
        ) as *mut *mut VertexNode;
        if ((*graph).buckets).is_null() as libc::c_int as libc::c_long != 0 {
            __assert_rtn(
                (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"initVertexGraph\0"))
                    .as_ptr(),
                b"vertexGraph.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int,
                b"graph->buckets != NULL\0" as *const u8 as *const libc::c_char,
            );
        } else {
        };
    } else {
        (*graph).buckets = 0 as *mut *mut VertexNode;
    }
    (*graph).numBuckets = numBuckets;
    (*graph).size = 0 as libc::c_int;
    (*graph).res = res;
}
#[no_mangle]
pub unsafe extern "C" fn destroyVertexGraph(mut graph: *mut VertexGraph) {
    let mut node: *mut VertexNode = std::ptr::null_mut::<VertexNode>();
    loop {
        node = firstVertexNode(graph);
        if node.is_null() {
            break;
        }
        removeVertexNode(graph, node);
    }
    test_prefix_free((*graph).buckets as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _hashVertex(
    mut vertex: *const LatLng,
    mut res: libc::c_int,
    mut numBuckets: libc::c_int,
) -> uint32_t {
    return fmod(
        fabs(
            ((*vertex).lat + (*vertex).lng)
                * pow(
                    10 as libc::c_int as libc::c_double,
                    (15 as libc::c_int - res) as libc::c_double,
                ),
        ),
        numBuckets as libc::c_double,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn _initVertexNode(
    mut node: *mut VertexNode,
    mut fromVtx: *const LatLng,
    mut toVtx: *const LatLng,
) {
    (*node).from = *fromVtx;
    (*node).to = *toVtx;
    (*node).next = std::ptr::null_mut::<VertexNode>();
}
#[no_mangle]
pub unsafe extern "C" fn addVertexNode(
    mut graph: *mut VertexGraph,
    mut fromVtx: *const LatLng,
    mut toVtx: *const LatLng,
) -> *mut VertexNode {
    let mut node: *mut VertexNode =
        test_prefix_malloc(::core::mem::size_of::<VertexNode>() as libc::c_ulong)
            as *mut VertexNode;
    if node.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"addVertexNode\0"))
                .as_ptr(),
            b"vertexGraph.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            b"node != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    _initVertexNode(node, fromVtx, toVtx);
    let mut index: uint32_t = _hashVertex(fromVtx, (*graph).res, (*graph).numBuckets);
    let mut currentNode: *mut VertexNode = *((*graph).buckets).offset(index as isize);
    if currentNode.is_null() {
        let ref mut fresh0 = *((*graph).buckets).offset(index as isize);
        *fresh0 = node;
    } else {
        loop {
            if geoAlmostEqual(&mut (*currentNode).from, fromVtx) as libc::c_int != 0
                && geoAlmostEqual(&mut (*currentNode).to, toVtx) as libc::c_int != 0
            {
                test_prefix_free(node as *mut libc::c_void);
                return currentNode;
            }
            if !((*currentNode).next).is_null() {
                currentNode = (*currentNode).next;
            }
            if ((*currentNode).next).is_null() {
                break;
            }
        }
        (*currentNode).next = node;
    }
    (*graph).size += 1;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn removeVertexNode(
    mut graph: *mut VertexGraph,
    mut node: *mut VertexNode,
) -> libc::c_int {
    let mut index: uint32_t = _hashVertex(&mut (*node).from, (*graph).res, (*graph).numBuckets);
    let mut currentNode: *mut VertexNode = *((*graph).buckets).offset(index as isize);
    let mut found: libc::c_int = 0 as libc::c_int;
    if !currentNode.is_null() {
        if currentNode == node {
            let ref mut fresh1 = *((*graph).buckets).offset(index as isize);
            *fresh1 = (*node).next;
            found = 1 as libc::c_int;
        }
        while found == 0 && !((*currentNode).next).is_null() {
            if (*currentNode).next == node {
                (*currentNode).next = (*node).next;
                found = 1 as libc::c_int;
            }
            currentNode = (*currentNode).next;
        }
    }
    if found != 0 {
        test_prefix_free(node as *mut libc::c_void);
        (*graph).size -= 1;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn findNodeForEdge(
    mut graph: *const VertexGraph,
    mut fromVtx: *const LatLng,
    mut toVtx: *const LatLng,
) -> *mut VertexNode {
    let mut index: uint32_t = _hashVertex(fromVtx, (*graph).res, (*graph).numBuckets);
    let mut node: *mut VertexNode = *((*graph).buckets).offset(index as isize);
    if !node.is_null() {
        loop {
            if geoAlmostEqual(&mut (*node).from, fromVtx) as libc::c_int != 0
                && (toVtx.is_null() || geoAlmostEqual(&mut (*node).to, toVtx) as libc::c_int != 0)
            {
                return node;
            }
            node = (*node).next;
            if node.is_null() {
                break;
            }
        }
    }
    return std::ptr::null_mut::<VertexNode>();
}
#[no_mangle]
pub unsafe extern "C" fn findNodeForVertex(
    mut graph: *const VertexGraph,
    mut fromVtx: *const LatLng,
) -> *mut VertexNode {
    return findNodeForEdge(graph, fromVtx, std::ptr::null::<LatLng>());
}
#[no_mangle]
pub unsafe extern "C" fn firstVertexNode(mut graph: *const VertexGraph) -> *mut VertexNode {
    let mut node: *mut VertexNode = std::ptr::null_mut::<VertexNode>();
    let mut currentIndex: libc::c_int = 0 as libc::c_int;
    while node.is_null() {
        if currentIndex < (*graph).numBuckets {
            node = *((*graph).buckets).offset(currentIndex as isize);
        } else {
            return std::ptr::null_mut::<VertexNode>();
        }
        currentIndex += 1;
    }
    return node;
}
