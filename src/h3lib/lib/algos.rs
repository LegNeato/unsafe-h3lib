use ::libc;
extern "C" {
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
    fn destroyLinkedMultiPolygon(polygon: *mut LinkedGeoPolygon);
    fn getNumCells(res: libc::c_int, out: *mut int64_t) -> H3Error;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn bboxHexEstimate(bbox: *const BBox, res: libc::c_int, out: *mut int64_t) -> H3Error;
    fn lineHexEstimate(
        origin: *const LatLng,
        destination: *const LatLng,
        res: libc::c_int,
        out: *mut int64_t,
    ) -> H3Error;
    fn normalizeMultiPolygon(root: *mut LinkedGeoPolygon) -> H3Error;
    fn destroyVertexGraph(graph: *mut VertexGraph);
    fn findNodeForVertex(graph: *const VertexGraph, fromVtx: *const LatLng) -> *mut VertexNode;
    fn removeVertexNode(graph: *mut VertexGraph, node: *mut VertexNode) -> libc::c_int;
    fn addLinkedCoord(loop_0: *mut LinkedGeoLoop, vertex: *const LatLng) -> *mut LinkedLatLng;
    fn addNewLinkedLoop(polygon: *mut LinkedGeoPolygon) -> *mut LinkedGeoLoop;
    fn firstVertexNode(graph: *const VertexGraph) -> *mut VertexNode;
    fn addVertexNode(
        graph: *mut VertexGraph,
        fromVtx: *const LatLng,
        toVtx: *const LatLng,
    ) -> *mut VertexNode;
    fn findNodeForEdge(
        graph: *const VertexGraph,
        fromVtx: *const LatLng,
        toVtx: *const LatLng,
    ) -> *mut VertexNode;
    fn initVertexGraph(graph: *mut VertexGraph, numBuckets: libc::c_int, res: libc::c_int);
    fn _rotate60ccw(digit: Direction) -> Direction;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn test_prefix_malloc(size: size_t) -> *mut libc::c_void;
    fn test_prefix_calloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn test_prefix_free(ptr: *mut libc::c_void);
    fn _isBaseCellPolarPentagon(baseCell: libc::c_int) -> bool;
    static baseCellData: [BaseCellData; 122];
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    static baseCellNeighbors: [[libc::c_int; 7]; 122];
    static baseCellNeighbor60CCWRots: [[libc::c_int; 7]; 122];
    fn _baseCellIsCwOffset(baseCell: libc::c_int, testFace: libc::c_int) -> bool;
    fn isResolutionClassIII(r: libc::c_int) -> libc::c_int;
    fn _h3LeadingNonZeroDigit(h: H3Index) -> Direction;
    fn _h3RotatePent60ccw(h: H3Index) -> H3Index;
    fn _h3Rotate60ccw(h: H3Index) -> H3Index;
    fn _h3Rotate60cw(h: H3Index) -> H3Index;
    fn bboxesFromGeoPolygon(polygon: *const GeoPolygon, bboxes: *mut BBox);
    fn pointInsidePolygon(
        geoPolygon: *const GeoPolygon,
        bboxes: *const BBox,
        coord: *const LatLng,
    ) -> bool;
    fn bboxFromGeoLoop(loop_0: *const GeoLoop, bbox: *mut BBox);
}
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoLoop {
    pub numVerts: libc::c_int,
    pub verts: *mut LatLng,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoPolygon {
    pub geoloop: GeoLoop,
    pub numHoles: libc::c_int,
    pub holes: *mut GeoLoop,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaceIJK {
    pub face: libc::c_int,
    pub coord: CoordIJK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoordIJK {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub k: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseCellData {
    pub homeFijk: FaceIJK,
    pub isPentagon: libc::c_int,
    pub cwOffsetPent: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BBox {
    pub north: libc::c_double,
    pub south: libc::c_double,
    pub east: libc::c_double,
    pub west: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexGraph {
    pub buckets: *mut *mut VertexNode,
    pub numBuckets: libc::c_int,
    pub size: libc::c_int,
    pub res: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexNode {
    pub from: LatLng,
    pub to: LatLng,
    pub next: *mut VertexNode,
}
static mut DIRECTIONS: [Direction; 6] = [
    J_AXES_DIGIT,
    JK_AXES_DIGIT,
    K_AXES_DIGIT,
    IK_AXES_DIGIT,
    I_AXES_DIGIT,
    IJ_AXES_DIGIT,
];
static mut NEXT_RING_DIRECTION: Direction = I_AXES_DIGIT;
static mut NEW_DIGIT_II: [[Direction; 7]; 7] = [
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
    ],
    [
        K_AXES_DIGIT,
        I_AXES_DIGIT,
        JK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        IK_AXES_DIGIT,
        J_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        K_AXES_DIGIT,
        I_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        IK_AXES_DIGIT,
    ],
    [
        JK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
    ],
    [
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        K_AXES_DIGIT,
    ],
    [
        IK_AXES_DIGIT,
        J_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        JK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        I_AXES_DIGIT,
    ],
    [
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        IK_AXES_DIGIT,
        J_AXES_DIGIT,
        K_AXES_DIGIT,
        I_AXES_DIGIT,
        JK_AXES_DIGIT,
    ],
];
static mut NEW_ADJUSTMENT_II: [[Direction; 7]; 7] = [
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
    ],
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        JK_AXES_DIGIT,
        JK_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        I_AXES_DIGIT,
        I_AXES_DIGIT,
        IJ_AXES_DIGIT,
    ],
    [
        CENTER_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
    ],
];
static mut NEW_DIGIT_III: [[Direction; 7]; 7] = [
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
    ],
    [
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
    ],
    [
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
    ],
    [
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
    ],
    [
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
    ],
    [
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
    ],
];
static mut NEW_ADJUSTMENT_III: [[Direction; 7]; 7] = [
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
        JK_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
        J_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
    ],
    [
        CENTER_DIGIT,
        JK_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        I_AXES_DIGIT,
    ],
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        IK_AXES_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        I_AXES_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
    ],
];
static mut K_ALL_CELLS_AT_RES_15: libc::c_int = 13780510 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn maxGridDiskSize(mut k: libc::c_int, mut out: *mut int64_t) -> H3Error {
    if k < 0 as libc::c_int {
        return E_DOMAIN as libc::c_int as H3Error;
    }
    if k >= K_ALL_CELLS_AT_RES_15 {
        return getNumCells(15 as libc::c_int, out);
    }
    *out = 3 as libc::c_int as libc::c_longlong
        * k as int64_t
        * (k as int64_t + 1 as libc::c_int as libc::c_longlong)
        + 1 as libc::c_int as libc::c_longlong;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn gridDisk(
    mut origin: H3Index,
    mut k: libc::c_int,
    mut out: *mut H3Index,
) -> H3Error {
    gridDiskDistances(origin, k, out, std::ptr::null_mut::<libc::c_int>())
}
#[no_mangle]
pub unsafe extern "C" fn gridDiskDistances(
    mut origin: H3Index,
    mut k: libc::c_int,
    mut out: *mut H3Index,
    mut distances: *mut libc::c_int,
) -> H3Error {
    let failed: H3Error = gridDiskDistancesUnsafe(origin, k, out, distances);
    if failed != 0 {
        let mut maxIdx: int64_t = 0;
        let mut err: H3Error = maxGridDiskSize(k, &mut maxIdx);
        if err != 0 {
            return err;
        }
        memset(
            out as *mut libc::c_void,
            0 as libc::c_int,
            (maxIdx as libc::c_ulonglong).wrapping_mul(::core::mem::size_of::<H3Index>()
                as libc::c_ulong
                as libc::c_ulonglong) as libc::c_ulong,
        );
        if distances.is_null() {
            distances = test_prefix_calloc(
                maxIdx as size_t,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int;
            if distances.is_null() {
                return E_MEMORY_ALLOC as libc::c_int as H3Error;
            }
            let mut result: H3Error =
                _gridDiskDistancesInternal(origin, k, out, distances, maxIdx, 0 as libc::c_int);
            test_prefix_free(distances as *mut libc::c_void);
            result
        } else {
            memset(
                distances as *mut libc::c_void,
                0 as libc::c_int,
                (maxIdx as libc::c_ulonglong).wrapping_mul(::core::mem::size_of::<libc::c_int>()
                    as libc::c_ulong
                    as libc::c_ulonglong) as libc::c_ulong,
            );
            _gridDiskDistancesInternal(origin, k, out, distances, maxIdx, 0 as libc::c_int)
        }
    } else {
        E_SUCCESS as libc::c_int as H3Error
    }
}
#[no_mangle]
pub unsafe extern "C" fn _gridDiskDistancesInternal(
    mut origin: H3Index,
    mut k: libc::c_int,
    mut out: *mut H3Index,
    mut distances: *mut libc::c_int,
    mut maxIdx: int64_t,
    mut curK: libc::c_int,
) -> H3Error {
    let mut off: int64_t = origin.wrapping_rem(maxIdx as libc::c_ulonglong) as int64_t;
    while *out.offset(off as isize) != 0 as libc::c_int as libc::c_ulonglong
        && *out.offset(off as isize) != origin
    {
        off = (off + 1 as libc::c_int as libc::c_longlong) % maxIdx;
    }
    if *out.offset(off as isize) == origin && *distances.offset(off as isize) <= curK {
        return E_SUCCESS as libc::c_int as H3Error;
    }
    *out.offset(off as isize) = origin;
    *distances.offset(off as isize) = curK;
    if curK >= k {
        return E_SUCCESS as libc::c_int as H3Error;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let mut rotations: libc::c_int = 0 as libc::c_int;
        let mut nextNeighbor: H3Index = 0;
        let mut neighborResult: H3Error = h3NeighborRotations(
            origin,
            DIRECTIONS[i as usize],
            &mut rotations,
            &mut nextNeighbor,
        );
        if neighborResult != E_PENTAGON as libc::c_int as libc::c_uint {
            if neighborResult != E_SUCCESS as libc::c_int as libc::c_uint {
                return neighborResult;
            }
            neighborResult = _gridDiskDistancesInternal(
                nextNeighbor,
                k,
                out,
                distances,
                maxIdx,
                curK + 1 as libc::c_int,
            );
            if neighborResult != 0 {
                return neighborResult;
            }
        }
        i += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn gridDiskDistancesSafe(
    mut origin: H3Index,
    mut k: libc::c_int,
    mut out: *mut H3Index,
    mut distances: *mut libc::c_int,
) -> H3Error {
    let mut maxIdx: int64_t = 0;
    let mut err: H3Error = maxGridDiskSize(k, &mut maxIdx);
    if err != 0 {
        return err;
    }
    _gridDiskDistancesInternal(origin, k, out, distances, maxIdx, 0 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn h3NeighborRotations(
    mut origin: H3Index,
    mut dir: Direction,
    mut rotations: *mut libc::c_int,
    mut out: *mut H3Index,
) -> H3Error {
    let mut current: H3Index = origin;
    if (dir as libc::c_uint) < CENTER_DIGIT as libc::c_int as libc::c_uint
        || dir as libc::c_uint >= INVALID_DIGIT as libc::c_int as libc::c_uint
    {
        return E_FAILED as libc::c_int as H3Error;
    }
    *rotations %= 6 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < *rotations {
        dir = _rotate60ccw(dir);
        i += 1;
    }
    let mut newRotations: libc::c_int = 0 as libc::c_int;
    let mut oldBaseCell: libc::c_int = ((current
        & (127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        >> 45 as libc::c_int) as libc::c_int;
    if oldBaseCell < 0 as libc::c_int || oldBaseCell >= 122 as libc::c_int {
        return E_CELL_INVALID as libc::c_int as H3Error;
    }
    let mut oldLeadingDigit: Direction = _h3LeadingNonZeroDigit(current);
    let mut r: libc::c_int = ((current & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int
        - 1 as libc::c_int;
    loop {
        if r == -(1 as libc::c_int) {
            current = current & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
                | (baseCellNeighbors[oldBaseCell as usize][dir as usize] as uint64_t)
                    << 45 as libc::c_int;
            newRotations = baseCellNeighbor60CCWRots[oldBaseCell as usize][dir as usize];
            if ((current & (127 as libc::c_int as uint64_t) << 45 as libc::c_int)
                >> 45 as libc::c_int) as libc::c_int
                == 127 as libc::c_int
            {
                current = current & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
                    | (baseCellNeighbors[oldBaseCell as usize]
                        [IK_AXES_DIGIT as libc::c_int as usize] as uint64_t)
                        << 45 as libc::c_int;
                newRotations = baseCellNeighbor60CCWRots[oldBaseCell as usize]
                    [IK_AXES_DIGIT as libc::c_int as usize];
                current = _h3Rotate60ccw(current);
                *rotations += 1 as libc::c_int;
            }
            break;
        } else {
            let mut oldDigit: Direction =
                (current >> ((15 as libc::c_int - (r + 1 as libc::c_int)) * 3 as libc::c_int)
                    & 7 as libc::c_int as uint64_t) as Direction;
            let mut nextDir: Direction = CENTER_DIGIT;
            if oldDigit as libc::c_uint == INVALID_DIGIT as libc::c_int as libc::c_uint {
                return E_CELL_INVALID as libc::c_int as H3Error;
            } else if isResolutionClassIII(r + 1 as libc::c_int) != 0 {
                current = current
                    & !((7 as libc::c_int as uint64_t)
                        << ((15 as libc::c_int - (r + 1 as libc::c_int)) * 3 as libc::c_int))
                    | (NEW_DIGIT_II[oldDigit as usize][dir as usize] as uint64_t)
                        << ((15 as libc::c_int - (r + 1 as libc::c_int)) * 3 as libc::c_int);
                nextDir = NEW_ADJUSTMENT_II[oldDigit as usize][dir as usize];
            } else {
                current = current
                    & !((7 as libc::c_int as uint64_t)
                        << ((15 as libc::c_int - (r + 1 as libc::c_int)) * 3 as libc::c_int))
                    | (NEW_DIGIT_III[oldDigit as usize][dir as usize] as uint64_t)
                        << ((15 as libc::c_int - (r + 1 as libc::c_int)) * 3 as libc::c_int);
                nextDir = NEW_ADJUSTMENT_III[oldDigit as usize][dir as usize];
            }
            if nextDir as libc::c_uint == CENTER_DIGIT as libc::c_int as libc::c_uint {
                break;
            }
            dir = nextDir;
            r -= 1;
        }
    }
    let mut newBaseCell: libc::c_int = ((current
        & (127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        >> 45 as libc::c_int) as libc::c_int;
    if _isBaseCellPentagon(newBaseCell) != 0 {
        let mut alreadyAdjustedKSubsequence: libc::c_int = 0 as libc::c_int;
        if _h3LeadingNonZeroDigit(current) as libc::c_uint
            == K_AXES_DIGIT as libc::c_int as libc::c_uint
        {
            if oldBaseCell != newBaseCell {
                if _baseCellIsCwOffset(
                    newBaseCell,
                    baseCellData[oldBaseCell as usize].homeFijk.face,
                ) {
                    current = _h3Rotate60cw(current);
                } else {
                    current = _h3Rotate60ccw(current);
                }
                alreadyAdjustedKSubsequence = 1 as libc::c_int;
            } else if oldLeadingDigit as libc::c_uint == CENTER_DIGIT as libc::c_int as libc::c_uint
            {
                return E_PENTAGON as libc::c_int as H3Error;
            } else if oldLeadingDigit as libc::c_uint
                == JK_AXES_DIGIT as libc::c_int as libc::c_uint
            {
                current = _h3Rotate60ccw(current);
                *rotations += 1 as libc::c_int;
            } else if oldLeadingDigit as libc::c_uint
                == IK_AXES_DIGIT as libc::c_int as libc::c_uint
            {
                current = _h3Rotate60cw(current);
                *rotations += 5 as libc::c_int;
            } else {
                return E_FAILED as libc::c_int as H3Error;
            }
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < newRotations {
            current = _h3RotatePent60ccw(current);
            i_0 += 1;
        }
        if oldBaseCell != newBaseCell {
            if _isBaseCellPolarPentagon(newBaseCell) {
                if oldBaseCell != 118 as libc::c_int
                    && oldBaseCell != 8 as libc::c_int
                    && _h3LeadingNonZeroDigit(current) as libc::c_uint
                        != JK_AXES_DIGIT as libc::c_int as libc::c_uint
                {
                    *rotations += 1 as libc::c_int;
                }
            } else if _h3LeadingNonZeroDigit(current) as libc::c_uint
                == IK_AXES_DIGIT as libc::c_int as libc::c_uint
                && alreadyAdjustedKSubsequence == 0
            {
                *rotations += 1 as libc::c_int;
            }
        }
    } else {
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < newRotations {
            current = _h3Rotate60ccw(current);
            i_1 += 1;
        }
    }
    *rotations = (*rotations + newRotations) % 6 as libc::c_int;
    *out = current;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn directionForNeighbor(
    mut origin: H3Index,
    mut destination: H3Index,
) -> Direction {
    let mut isPent: bool = isPentagon(origin) != 0;
    let mut direction: Direction = (if isPent as libc::c_int != 0 {
        J_AXES_DIGIT as libc::c_int
    } else {
        K_AXES_DIGIT as libc::c_int
    }) as Direction;
    while (direction as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
        let mut neighbor: H3Index = 0;
        let mut rotations: libc::c_int = 0 as libc::c_int;
        let mut neighborError: H3Error =
            h3NeighborRotations(origin, direction, &mut rotations, &mut neighbor);
        if neighborError == 0 && neighbor == destination {
            return direction;
        }
        direction += 1;
    }
    INVALID_DIGIT
}
#[no_mangle]
pub unsafe extern "C" fn gridDiskUnsafe(
    mut origin: H3Index,
    mut k: libc::c_int,
    mut out: *mut H3Index,
) -> H3Error {
    gridDiskDistancesUnsafe(origin, k, out, std::ptr::null_mut::<libc::c_int>())
}
#[no_mangle]
pub unsafe extern "C" fn gridDiskDistancesUnsafe(
    mut origin: H3Index,
    mut k: libc::c_int,
    mut out: *mut H3Index,
    mut distances: *mut libc::c_int,
) -> H3Error {
    if k < 0 as libc::c_int {
        return E_DOMAIN as libc::c_int as H3Error;
    }
    let mut idx: libc::c_int = 0 as libc::c_int;
    *out.offset(idx as isize) = origin;
    if !distances.is_null() {
        *distances.offset(idx as isize) = 0 as libc::c_int;
    }
    idx += 1;
    if isPentagon(origin) != 0 {
        return E_PENTAGON as libc::c_int as H3Error;
    }
    let mut ring: libc::c_int = 1 as libc::c_int;
    let mut direction: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut rotations: libc::c_int = 0 as libc::c_int;
    while ring <= k {
        if direction == 0 as libc::c_int && i == 0 as libc::c_int {
            let mut neighborResult: H3Error =
                h3NeighborRotations(origin, NEXT_RING_DIRECTION, &mut rotations, &mut origin);
            if neighborResult != 0 {
                return neighborResult;
            }
            if isPentagon(origin) != 0 {
                return E_PENTAGON as libc::c_int as H3Error;
            }
        }
        let mut neighborResult_0: H3Error = h3NeighborRotations(
            origin,
            DIRECTIONS[direction as usize],
            &mut rotations,
            &mut origin,
        );
        if neighborResult_0 != 0 {
            return neighborResult_0;
        }
        *out.offset(idx as isize) = origin;
        if !distances.is_null() {
            *distances.offset(idx as isize) = ring;
        }
        idx += 1;
        i += 1;
        if i == ring {
            i = 0 as libc::c_int;
            direction += 1;
            if direction == 6 as libc::c_int {
                direction = 0 as libc::c_int;
                ring += 1;
            }
        }
        if isPentagon(origin) != 0 {
            return E_PENTAGON as libc::c_int as H3Error;
        }
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn gridDisksUnsafe(
    mut h3Set: *mut H3Index,
    mut length: libc::c_int,
    mut k: libc::c_int,
    mut out: *mut H3Index,
) -> H3Error {
    let mut segment: *mut H3Index = std::ptr::null_mut::<H3Index>();
    let mut segmentSize: int64_t = 0;
    let mut err: H3Error = maxGridDiskSize(k, &mut segmentSize);
    if err != 0 {
        return err;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < length {
        segment = out.offset((i as libc::c_longlong * segmentSize) as isize);
        let mut failed: H3Error = gridDiskUnsafe(*h3Set.offset(i as isize), k, segment);
        if failed != 0 {
            return failed;
        }
        i += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn gridRingUnsafe(
    mut origin: H3Index,
    mut k: libc::c_int,
    mut out: *mut H3Index,
) -> H3Error {
    if k == 0 as libc::c_int {
        *out.offset(0 as libc::c_int as isize) = origin;
        return E_SUCCESS as libc::c_int as H3Error;
    }
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut rotations: libc::c_int = 0 as libc::c_int;
    if isPentagon(origin) != 0 {
        return E_PENTAGON as libc::c_int as H3Error;
    }
    let mut ring: libc::c_int = 0 as libc::c_int;
    while ring < k {
        let mut neighborResult: H3Error =
            h3NeighborRotations(origin, NEXT_RING_DIRECTION, &mut rotations, &mut origin);
        if neighborResult != 0 {
            return neighborResult;
        }
        if isPentagon(origin) != 0 {
            return E_PENTAGON as libc::c_int as H3Error;
        }
        ring += 1;
    }
    let mut lastIndex: H3Index = origin;
    *out.offset(idx as isize) = origin;
    idx += 1;
    let mut direction: libc::c_int = 0 as libc::c_int;
    while direction < 6 as libc::c_int {
        let mut pos: libc::c_int = 0 as libc::c_int;
        while pos < k {
            let mut neighborResult_0: H3Error = h3NeighborRotations(
                origin,
                DIRECTIONS[direction as usize],
                &mut rotations,
                &mut origin,
            );
            if neighborResult_0 != 0 {
                return neighborResult_0;
            }
            if pos != k - 1 as libc::c_int || direction != 5 as libc::c_int {
                *out.offset(idx as isize) = origin;
                idx += 1;
                if isPentagon(origin) != 0 {
                    return E_PENTAGON as libc::c_int as H3Error;
                }
            }
            pos += 1;
        }
        direction += 1;
    }
    if lastIndex != origin {
        E_PENTAGON as libc::c_int as H3Error
    } else {
        E_SUCCESS as libc::c_int as H3Error
    }
}
#[no_mangle]
pub unsafe extern "C" fn maxPolygonToCellsSize(
    mut geoPolygon: *const GeoPolygon,
    mut res: libc::c_int,
    mut flags: uint32_t,
    mut out: *mut int64_t,
) -> H3Error {
    if flags != 0 as libc::c_int as libc::c_uint {
        return E_OPTION_INVALID as libc::c_int as H3Error;
    }
    let mut bbox: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    let geoloop: GeoLoop = (*geoPolygon).geoloop;
    bboxFromGeoLoop(&geoloop, &mut bbox);
    let mut numHexagons: int64_t = 0;
    let mut estimateErr: H3Error = bboxHexEstimate(&bbox, res, &mut numHexagons);
    if estimateErr != 0 {
        return estimateErr;
    }
    let mut totalVerts: libc::c_int = geoloop.numVerts;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*geoPolygon).numHoles {
        totalVerts += (*((*geoPolygon).holes).offset(i as isize)).numVerts;
        i += 1;
    }
    if numHexagons < totalVerts as libc::c_longlong {
        numHexagons = totalVerts as int64_t;
    }
    numHexagons += 12 as libc::c_int as libc::c_longlong;
    *out = numHexagons;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn _getEdgeHexagons(
    mut geoloop: *const GeoLoop,
    mut numHexagons: int64_t,
    mut res: libc::c_int,
    mut numSearchHexes: *mut int64_t,
    mut search: *mut H3Index,
    mut found: *mut H3Index,
) -> H3Error {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*geoloop).numVerts {
        let mut origin: LatLng = *((*geoloop).verts).offset(i as isize);
        let mut destination: LatLng = if i == (*geoloop).numVerts - 1 as libc::c_int {
            *((*geoloop).verts).offset(0 as libc::c_int as isize)
        } else {
            *((*geoloop).verts).offset((i + 1 as libc::c_int) as isize)
        };
        let mut numHexesEstimate: int64_t = 0;
        let mut estimateErr: H3Error =
            lineHexEstimate(&origin, &destination, res, &mut numHexesEstimate);
        if estimateErr != 0 {
            return estimateErr;
        }
        let mut j: int64_t = 0 as libc::c_int as int64_t;
        while j < numHexesEstimate {
            let mut interpolate: LatLng = LatLng { lat: 0., lng: 0. };
            interpolate.lat = origin.lat * (numHexesEstimate - j) as libc::c_double
                / numHexesEstimate as libc::c_double
                + destination.lat * j as libc::c_double / numHexesEstimate as libc::c_double;
            interpolate.lng = origin.lng * (numHexesEstimate - j) as libc::c_double
                / numHexesEstimate as libc::c_double
                + destination.lng * j as libc::c_double / numHexesEstimate as libc::c_double;
            let mut pointHex: H3Index = 0;
            let mut e: H3Error = latLngToCell(&interpolate, res, &mut pointHex);
            if e != 0 {
                return e;
            }
            let mut loc: int64_t =
                pointHex.wrapping_rem(numHexagons as libc::c_ulonglong) as int64_t;
            let mut loopCount: int64_t = 0 as libc::c_int as int64_t;
            while *found.offset(loc as isize) != 0 as libc::c_int as libc::c_ulonglong {
                if loopCount > numHexagons {
                    return E_FAILED as libc::c_int as H3Error;
                }
                if *found.offset(loc as isize) == pointHex {
                    break;
                }
                loc = (loc + 1 as libc::c_int as libc::c_longlong) % numHexagons;
                loopCount += 1;
            }
            if *found.offset(loc as isize) != pointHex {
                *found.offset(loc as isize) = pointHex;
                *search.offset(*numSearchHexes as isize) = pointHex;
                *numSearchHexes += 1;
            }
            j += 1;
        }
        i += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn polygonToCells(
    mut geoPolygon: *const GeoPolygon,
    mut res: libc::c_int,
    mut flags: uint32_t,
    mut out: *mut H3Index,
) -> H3Error {
    if flags != 0 as libc::c_int as libc::c_uint {
        return E_OPTION_INVALID as libc::c_int as H3Error;
    }
    let mut bboxes: *mut BBox = test_prefix_malloc(
        (((*geoPolygon).numHoles + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<BBox>() as libc::c_ulong),
    ) as *mut BBox;
    if bboxes.is_null() {
        return E_MEMORY_ALLOC as libc::c_int as H3Error;
    }
    bboxesFromGeoPolygon(geoPolygon, bboxes);
    let mut numHexagons: int64_t = 0;
    let mut numHexagonsError: H3Error =
        maxPolygonToCellsSize(geoPolygon, res, flags, &mut numHexagons);
    if numHexagonsError != 0 {
        test_prefix_free(bboxes as *mut libc::c_void);
        return numHexagonsError;
    }
    let mut search: *mut H3Index = test_prefix_calloc(
        numHexagons as size_t,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if search.is_null() {
        test_prefix_free(bboxes as *mut libc::c_void);
        return E_MEMORY_ALLOC as libc::c_int as H3Error;
    }
    let mut found: *mut H3Index = test_prefix_calloc(
        numHexagons as size_t,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if found.is_null() {
        test_prefix_free(bboxes as *mut libc::c_void);
        test_prefix_free(search as *mut libc::c_void);
        return E_MEMORY_ALLOC as libc::c_int as H3Error;
    }
    let mut numSearchHexes: int64_t = 0 as libc::c_int as int64_t;
    let mut numFoundHexes: int64_t = 0 as libc::c_int as int64_t;
    let geoloop: GeoLoop = (*geoPolygon).geoloop;
    let mut edgeHexError: H3Error = _getEdgeHexagons(
        &geoloop,
        numHexagons,
        res,
        &mut numSearchHexes,
        search,
        found,
    );
    if edgeHexError != 0 {
        test_prefix_free(search as *mut libc::c_void);
        test_prefix_free(found as *mut libc::c_void);
        test_prefix_free(bboxes as *mut libc::c_void);
        return edgeHexError;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*geoPolygon).numHoles {
        let mut hole: *mut GeoLoop = &mut *((*geoPolygon).holes).offset(i as isize) as *mut GeoLoop;
        edgeHexError = _getEdgeHexagons(hole, numHexagons, res, &mut numSearchHexes, search, found);
        if edgeHexError != 0 {
            test_prefix_free(search as *mut libc::c_void);
            test_prefix_free(found as *mut libc::c_void);
            test_prefix_free(bboxes as *mut libc::c_void);
            return edgeHexError;
        }
        i += 1;
    }
    let mut i_0: int64_t = 0 as libc::c_int as int64_t;
    while i_0 < numHexagons {
        *found.offset(i_0 as isize) = 0 as libc::c_int as H3Index;
        i_0 += 1;
    }
    while numSearchHexes > 0 as libc::c_int as libc::c_longlong {
        let mut currentSearchNum: int64_t = 0 as libc::c_int as int64_t;
        let mut i_1: int64_t = 0 as libc::c_int as int64_t;
        while currentSearchNum < numSearchHexes {
            let mut ring: [H3Index; 7] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0];
            let mut searchHex: H3Index = *search.offset(i_1 as isize);
            gridDisk(searchHex, 1 as libc::c_int, ring.as_mut_ptr());
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < 7 as libc::c_int {
                if ring[j as usize] != 0 as libc::c_int as libc::c_ulonglong {
                    let mut hex: H3Index = ring[j as usize];
                    let mut loc: int64_t =
                        hex.wrapping_rem(numHexagons as libc::c_ulonglong) as int64_t;
                    let mut loopCount: int64_t = 0 as libc::c_int as int64_t;
                    while *out.offset(loc as isize) != 0 as libc::c_int as libc::c_ulonglong {
                        if loopCount > numHexagons {
                            test_prefix_free(search as *mut libc::c_void);
                            test_prefix_free(found as *mut libc::c_void);
                            test_prefix_free(bboxes as *mut libc::c_void);
                            return E_FAILED as libc::c_int as H3Error;
                        }
                        if *out.offset(loc as isize) == hex {
                            break;
                        }
                        loc = (loc + 1 as libc::c_int as libc::c_longlong) % numHexagons;
                        loopCount += 1;
                    }
                    if *out.offset(loc as isize) != hex {
                        let mut hexCenter: LatLng = LatLng { lat: 0., lng: 0. };
                        cellToLatLng(hex, &mut hexCenter);
                        if pointInsidePolygon(geoPolygon, bboxes, &hexCenter) {
                            *out.offset(loc as isize) = hex;
                            *found.offset(numFoundHexes as isize) = hex;
                            numFoundHexes += 1;
                        }
                    }
                }
                j += 1;
            }
            currentSearchNum += 1;
            i_1 += 1;
        }
        std::mem::swap(&mut search, &mut found);
        let mut j_0: int64_t = 0 as libc::c_int as int64_t;
        while j_0 < numSearchHexes {
            *found.offset(j_0 as isize) = 0 as libc::c_int as H3Index;
            j_0 += 1;
        }
        numSearchHexes = numFoundHexes;
        numFoundHexes = 0 as libc::c_int as int64_t;
    }
    test_prefix_free(bboxes as *mut libc::c_void);
    test_prefix_free(search as *mut libc::c_void);
    test_prefix_free(found as *mut libc::c_void);
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn h3SetToVertexGraph(
    mut h3Set: *const H3Index,
    numHexes: libc::c_int,
    mut graph: *mut VertexGraph,
) -> H3Error {
    let mut vertices: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut fromVtx: *mut LatLng = std::ptr::null_mut::<LatLng>();
    let mut toVtx: *mut LatLng = std::ptr::null_mut::<LatLng>();
    let mut edge: *mut VertexNode = std::ptr::null_mut::<VertexNode>();
    if numHexes < 1 as libc::c_int {
        initVertexGraph(graph, 0 as libc::c_int, 0 as libc::c_int);
        return E_SUCCESS as libc::c_int as H3Error;
    }
    let mut res: libc::c_int = ((*h3Set.offset(0 as libc::c_int as isize)
        & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int;
    let minBuckets: libc::c_int = 6 as libc::c_int;
    let mut numBuckets: libc::c_int = if numHexes > minBuckets {
        numHexes
    } else {
        minBuckets
    };
    initVertexGraph(graph, numBuckets, res);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numHexes {
        let mut boundaryErr: H3Error = cellToBoundary(*h3Set.offset(i as isize), &mut vertices);
        if boundaryErr != 0 {
            destroyVertexGraph(graph);
            return boundaryErr;
        }
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < vertices.numVerts {
            fromVtx = &mut *(vertices.verts).as_mut_ptr().offset(j as isize) as *mut LatLng;
            toVtx = &mut *(vertices.verts)
                .as_mut_ptr()
                .offset(((j + 1 as libc::c_int) % vertices.numVerts) as isize)
                as *mut LatLng;
            edge = findNodeForEdge(graph, toVtx, fromVtx);
            if !edge.is_null() {
                removeVertexNode(graph, edge);
            } else {
                addVertexNode(graph, fromVtx, toVtx);
            }
            j += 1;
        }
        i += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn _vertexGraphToLinkedGeo(
    mut graph: *mut VertexGraph,
    mut out: *mut LinkedGeoPolygon,
) {
    *out = {
        LinkedGeoPolygon {
            first: std::ptr::null_mut::<LinkedGeoLoop>(),
            last: std::ptr::null_mut::<LinkedGeoLoop>(),
            next: std::ptr::null_mut::<LinkedGeoPolygon>(),
        }
    };
    let mut loop_0: *mut LinkedGeoLoop = std::ptr::null_mut::<LinkedGeoLoop>();
    let mut edge: *mut VertexNode = std::ptr::null_mut::<VertexNode>();
    let mut nextVtx: LatLng = LatLng { lat: 0., lng: 0. };
    loop {
        edge = firstVertexNode(graph);
        if edge.is_null() {
            break;
        }
        loop_0 = addNewLinkedLoop(out);
        loop {
            addLinkedCoord(loop_0, &(*edge).from);
            nextVtx = (*edge).to;
            removeVertexNode(graph, edge);
            edge = findNodeForVertex(graph, &nextVtx);
            if edge.is_null() {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cellsToLinkedMultiPolygon(
    mut h3Set: *const H3Index,
    numHexes: libc::c_int,
    mut out: *mut LinkedGeoPolygon,
) -> H3Error {
    let mut graph: VertexGraph = VertexGraph {
        buckets: std::ptr::null_mut::<*mut VertexNode>(),
        numBuckets: 0,
        size: 0,
        res: 0,
    };
    let mut err: H3Error = h3SetToVertexGraph(h3Set, numHexes, &mut graph);
    if err != 0 {
        return err;
    }
    _vertexGraphToLinkedGeo(&mut graph, out);
    destroyVertexGraph(&mut graph);
    let mut normalizeResult: H3Error = normalizeMultiPolygon(out);
    if normalizeResult != 0 {
        destroyLinkedMultiPolygon(out);
    }
    normalizeResult
}
