use ::libc;
extern "C" {
    fn directionForNeighbor(origin: H3Index, destination: H3Index) -> Direction;
    fn h3NeighborRotations(
        origin: H3Index,
        dir: Direction,
        rotations: *mut libc::c_int,
        out: *mut H3Index,
    ) -> H3Error;
    fn gridDisk(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn cellToParent(h: H3Index, parentRes: libc::c_int, parent: *mut H3Index) -> H3Error;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn _faceIjkToCellBoundary(
        h: *const FaceIJK,
        res: libc::c_int,
        start: libc::c_int,
        length: libc::c_int,
        g: *mut CellBoundary,
    );
    fn _faceIjkPentToCellBoundary(
        h: *const FaceIJK,
        res: libc::c_int,
        start: libc::c_int,
        length: libc::c_int,
        g: *mut CellBoundary,
    );
    fn _h3ToFaceIjk(h: H3Index, fijk: *mut FaceIJK) -> H3Error;
    fn vertexNumForDirection(origin: H3Index, direction: Direction) -> libc::c_int;
}
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
#[no_mangle]
pub unsafe extern "C" fn areNeighborCells(
    mut origin: H3Index,
    mut destination: H3Index,
    mut out: *mut libc::c_int,
) -> H3Error {
    if ((origin & (15 as libc::c_int as uint64_t) << 59 as libc::c_int) >> 59 as libc::c_int)
        as libc::c_int
        != 1 as libc::c_int
        || ((destination & (15 as libc::c_int as uint64_t) << 59 as libc::c_int)
            >> 59 as libc::c_int) as libc::c_int
            != 1 as libc::c_int
    {
        return E_CELL_INVALID as libc::c_int as H3Error;
    }
    if origin == destination {
        *out = 0 as libc::c_int;
        return E_SUCCESS as libc::c_int as H3Error;
    }
    if ((origin & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
        as libc::c_int
        != ((destination & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
            as libc::c_int
    {
        return E_RES_MISMATCH as libc::c_int as H3Error;
    }
    let mut parentRes: libc::c_int = ((origin & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int
        - 1 as libc::c_int;
    if parentRes > 0 as libc::c_int {
        let mut originParent: H3Index = 0;
        cellToParent(origin, parentRes, &mut originParent);
        let mut destinationParent: H3Index = 0;
        cellToParent(destination, parentRes, &mut destinationParent);
        if originParent == destinationParent {
            let mut originResDigit: Direction =
                (origin >> (15 as libc::c_int - (parentRes + 1 as libc::c_int)) * 3 as libc::c_int
                    & 7 as libc::c_int as uint64_t) as Direction;
            let mut destinationResDigit: Direction = (destination
                >> (15 as libc::c_int - (parentRes + 1 as libc::c_int)) * 3 as libc::c_int
                & 7 as libc::c_int as uint64_t)
                as Direction;
            if originResDigit as libc::c_uint == CENTER_DIGIT as libc::c_int as libc::c_uint
                || destinationResDigit as libc::c_uint
                    == CENTER_DIGIT as libc::c_int as libc::c_uint
            {
                *out = 1 as libc::c_int;
                return E_SUCCESS as libc::c_int as H3Error;
            }
            if originResDigit as libc::c_uint >= INVALID_DIGIT as libc::c_int as libc::c_uint {
                return E_CELL_INVALID as libc::c_int as H3Error;
            }
            if (originResDigit as libc::c_uint == K_AXES_DIGIT as libc::c_int as libc::c_uint
                || destinationResDigit as libc::c_uint
                    == K_AXES_DIGIT as libc::c_int as libc::c_uint)
                && isPentagon(originParent) != 0
            {
                return E_CELL_INVALID as libc::c_int as H3Error;
            }
            let neighborSetClockwise: [Direction; 7] = [
                CENTER_DIGIT,
                JK_AXES_DIGIT,
                IJ_AXES_DIGIT,
                J_AXES_DIGIT,
                IK_AXES_DIGIT,
                K_AXES_DIGIT,
                I_AXES_DIGIT,
            ];
            let neighborSetCounterclockwise: [Direction; 7] = [
                CENTER_DIGIT,
                IK_AXES_DIGIT,
                JK_AXES_DIGIT,
                K_AXES_DIGIT,
                IJ_AXES_DIGIT,
                I_AXES_DIGIT,
                J_AXES_DIGIT,
            ];
            if neighborSetClockwise[originResDigit as usize] as libc::c_uint
                == destinationResDigit as libc::c_uint
                || neighborSetCounterclockwise[originResDigit as usize] as libc::c_uint
                    == destinationResDigit as libc::c_uint
            {
                *out = 1 as libc::c_int;
                return E_SUCCESS as libc::c_int as H3Error;
            }
        }
    }
    let mut neighborRing: [H3Index; 7] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0];
    gridDisk(origin, 1 as libc::c_int, neighborRing.as_mut_ptr());
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        if neighborRing[i as usize] == destination {
            *out = 1 as libc::c_int;
            return E_SUCCESS as libc::c_int as H3Error;
        }
        i += 1;
    }
    *out = 0 as libc::c_int;
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn cellsToDirectedEdge(
    mut origin: H3Index,
    mut destination: H3Index,
    mut out: *mut H3Index,
) -> H3Error {
    let mut direction: Direction = directionForNeighbor(origin, destination);
    if direction as libc::c_uint == INVALID_DIGIT as libc::c_int as libc::c_uint {
        return E_NOT_NEIGHBORS as libc::c_int as H3Error;
    }
    let mut output: H3Index = origin;
    output = output & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
    output = output & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (direction as uint64_t) << 56 as libc::c_int;
    *out = output;
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn getDirectedEdgeOrigin(
    mut edge: H3Index,
    mut out: *mut H3Index,
) -> H3Error {
    if ((edge & (15 as libc::c_int as uint64_t) << 59 as libc::c_int) >> 59 as libc::c_int)
        as libc::c_int
        != 2 as libc::c_int
    {
        return E_DIR_EDGE_INVALID as libc::c_int as H3Error;
    }
    let mut origin: H3Index = edge;
    origin = origin & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    origin = origin & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (0 as libc::c_int as uint64_t) << 56 as libc::c_int;
    *out = origin;
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn getDirectedEdgeDestination(
    mut edge: H3Index,
    mut out: *mut H3Index,
) -> H3Error {
    let mut direction: Direction = ((edge & (7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        >> 56 as libc::c_int) as libc::c_int as Direction;
    let mut rotations: libc::c_int = 0 as libc::c_int;
    let mut origin: H3Index = 0;
    let mut originResult: H3Error = getDirectedEdgeOrigin(edge, &mut origin);
    if originResult != 0 {
        return originResult;
    }
    return h3NeighborRotations(origin, direction, &mut rotations, out);
}
#[no_mangle]
pub unsafe extern "C" fn isValidDirectedEdge(mut edge: H3Index) -> libc::c_int {
    let mut neighborDirection: Direction =
        ((edge & (7 as libc::c_int as uint64_t) << 56 as libc::c_int) >> 56 as libc::c_int)
            as libc::c_int as Direction;
    if neighborDirection as libc::c_uint <= CENTER_DIGIT as libc::c_int as libc::c_uint
        || neighborDirection as libc::c_uint >= NUM_DIGITS as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    let mut origin: H3Index = 0;
    let mut originResult: H3Error = getDirectedEdgeOrigin(edge, &mut origin);
    if originResult != 0 {
        return 0 as libc::c_int;
    }
    if isPentagon(origin) != 0
        && neighborDirection as libc::c_uint == K_AXES_DIGIT as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    return isValidCell(origin);
}
#[no_mangle]
pub unsafe extern "C" fn directedEdgeToCells(
    mut edge: H3Index,
    mut originDestination: *mut H3Index,
) -> H3Error {
    let mut originResult: H3Error = getDirectedEdgeOrigin(
        edge,
        &mut *originDestination.offset(0 as libc::c_int as isize),
    );
    if originResult != 0 {
        return originResult;
    }
    let mut destinationResult: H3Error = getDirectedEdgeDestination(
        edge,
        &mut *originDestination.offset(1 as libc::c_int as isize),
    );
    if destinationResult != 0 {
        return destinationResult;
    }
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn originToDirectedEdges(
    mut origin: H3Index,
    mut edges: *mut H3Index,
) -> H3Error {
    let mut isPent: libc::c_int = isPentagon(origin);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if isPent != 0 && i == 0 as libc::c_int {
            *edges.offset(i as isize) = 0 as libc::c_int as H3Index;
        } else {
            *edges.offset(i as isize) = origin;
            *edges.offset(i as isize) = *edges.offset(i as isize)
                & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
                | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
            *edges.offset(i as isize) = *edges.offset(i as isize)
                & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
                | ((i + 1 as libc::c_int) as uint64_t) << 56 as libc::c_int;
        }
        i += 1;
    }
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn directedEdgeToBoundary(
    mut edge: H3Index,
    mut cb: *mut CellBoundary,
) -> H3Error {
    let mut direction: Direction = ((edge & (7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        >> 56 as libc::c_int) as libc::c_int as Direction;
    let mut origin: H3Index = 0;
    let mut originResult: H3Error = getDirectedEdgeOrigin(edge, &mut origin);
    if originResult != 0 {
        return originResult;
    }
    let mut startVertex: libc::c_int = vertexNumForDirection(origin, direction);
    if startVertex == -(1 as libc::c_int) {
        (*cb).numVerts = 0 as libc::c_int;
        return E_DIR_EDGE_INVALID as libc::c_int as H3Error;
    }
    let mut fijk: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    let mut fijkResult: H3Error = _h3ToFaceIjk(origin, &mut fijk);
    if fijkResult != 0 {
        return fijkResult;
    }
    let mut res: libc::c_int = ((origin & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int;
    let mut isPent: libc::c_int = isPentagon(origin);
    if isPent != 0 {
        _faceIjkPentToCellBoundary(&mut fijk, res, startVertex, 2 as libc::c_int, cb);
    } else {
        _faceIjkToCellBoundary(&mut fijk, res, startVertex, 2 as libc::c_int, cb);
    }
    return E_SUCCESS as libc::c_int as H3Error;
}
