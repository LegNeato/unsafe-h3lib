use ::libc;
extern "C" {
    fn getBaseCellNumber(h: H3Index) -> libc::c_int;
    fn isValidCell(h: H3Index) -> libc::c_int;
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
    fn _h3LeadingNonZeroDigit(h: H3Index) -> Direction;
    fn h3NeighborRotations(
        origin: H3Index,
        dir: Direction,
        rotations: *mut libc::c_int,
        out: *mut H3Index,
    ) -> H3Error;
    fn directionForNeighbor(origin: H3Index, destination: H3Index) -> Direction;
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    fn _isBaseCellPolarPentagon(baseCell: libc::c_int) -> bool;
    fn _baseCellToCCWrot60(baseCell: libc::c_int, face: libc::c_int) -> libc::c_int;
    fn _baseCellToFaceIjk(baseCell: libc::c_int, h: *mut FaceIJK);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PentagonDirectionFaces {
    pub baseCell: libc::c_int,
    pub faces: [libc::c_int; 5],
}
static mut pentagonDirectionFaces: [PentagonDirectionFaces; 12] = [
    {
        
        PentagonDirectionFaces {
            baseCell: 4 as libc::c_int,
            faces: [
                4 as libc::c_int,
                0 as libc::c_int,
                2 as libc::c_int,
                1 as libc::c_int,
                3 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 14 as libc::c_int,
            faces: [
                6 as libc::c_int,
                11 as libc::c_int,
                2 as libc::c_int,
                7 as libc::c_int,
                1 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 24 as libc::c_int,
            faces: [
                5 as libc::c_int,
                10 as libc::c_int,
                1 as libc::c_int,
                6 as libc::c_int,
                0 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 38 as libc::c_int,
            faces: [
                7 as libc::c_int,
                12 as libc::c_int,
                3 as libc::c_int,
                8 as libc::c_int,
                2 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 49 as libc::c_int,
            faces: [
                9 as libc::c_int,
                14 as libc::c_int,
                0 as libc::c_int,
                5 as libc::c_int,
                4 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 58 as libc::c_int,
            faces: [
                8 as libc::c_int,
                13 as libc::c_int,
                4 as libc::c_int,
                9 as libc::c_int,
                3 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 63 as libc::c_int,
            faces: [
                11 as libc::c_int,
                6 as libc::c_int,
                15 as libc::c_int,
                10 as libc::c_int,
                16 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 72 as libc::c_int,
            faces: [
                12 as libc::c_int,
                7 as libc::c_int,
                16 as libc::c_int,
                11 as libc::c_int,
                17 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 83 as libc::c_int,
            faces: [
                10 as libc::c_int,
                5 as libc::c_int,
                19 as libc::c_int,
                14 as libc::c_int,
                15 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 97 as libc::c_int,
            faces: [
                13 as libc::c_int,
                8 as libc::c_int,
                17 as libc::c_int,
                12 as libc::c_int,
                18 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 107 as libc::c_int,
            faces: [
                14 as libc::c_int,
                9 as libc::c_int,
                18 as libc::c_int,
                13 as libc::c_int,
                19 as libc::c_int,
            ],
        }
    },
    {
        
        PentagonDirectionFaces {
            baseCell: 117 as libc::c_int,
            faces: [
                15 as libc::c_int,
                19 as libc::c_int,
                17 as libc::c_int,
                18 as libc::c_int,
                16 as libc::c_int,
            ],
        }
    },
];
unsafe extern "C" fn vertexRotations(mut cell: H3Index, mut out: *mut libc::c_int) -> H3Error {
    let mut fijk: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    let mut err: H3Error = _h3ToFaceIjk(cell, &mut fijk);
    if err != 0 {
        return err;
    }
    let mut baseCell: libc::c_int = getBaseCellNumber(cell);
    let mut cellLeadingDigit: libc::c_int = _h3LeadingNonZeroDigit(cell) as libc::c_int;
    let mut baseFijk: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    _baseCellToFaceIjk(baseCell, &mut baseFijk);
    let mut ccwRot60: libc::c_int = _baseCellToCCWrot60(baseCell, fijk.face);
    if _isBaseCellPentagon(baseCell) != 0 {
        let mut dirFaces: PentagonDirectionFaces = PentagonDirectionFaces {
            baseCell: 0,
            faces: [0; 5],
        };
        let mut p: libc::c_int = 0 as libc::c_int;
        while p < 12 as libc::c_int {
            if pentagonDirectionFaces[p as usize].baseCell == baseCell {
                dirFaces = pentagonDirectionFaces[p as usize];
                break;
            } else {
                p += 1;
            }
        }
        if fijk.face != baseFijk.face
            && (_isBaseCellPolarPentagon(baseCell) as libc::c_int != 0
                || fijk.face
                    == dirFaces.faces[(IK_AXES_DIGIT as libc::c_int - 2 as libc::c_int) as usize])
        {
            ccwRot60 = (ccwRot60 + 1 as libc::c_int) % 6 as libc::c_int;
        }
        if cellLeadingDigit == JK_AXES_DIGIT as libc::c_int
            && fijk.face
                == dirFaces.faces[(IK_AXES_DIGIT as libc::c_int - 2 as libc::c_int) as usize]
        {
            ccwRot60 = (ccwRot60 + 5 as libc::c_int) % 6 as libc::c_int;
        } else if cellLeadingDigit == IK_AXES_DIGIT as libc::c_int
            && fijk.face
                == dirFaces.faces[(JK_AXES_DIGIT as libc::c_int - 2 as libc::c_int) as usize]
        {
            ccwRot60 = (ccwRot60 + 1 as libc::c_int) % 6 as libc::c_int;
        }
    }
    *out = ccwRot60;
    E_SUCCESS as libc::c_int as H3Error
}
static mut directionToVertexNumHex: [libc::c_int; 7] = [
    INVALID_DIGIT as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    5 as libc::c_int,
    4 as libc::c_int,
    0 as libc::c_int,
];
static mut directionToVertexNumPent: [libc::c_int; 7] = [
    INVALID_DIGIT as libc::c_int,
    INVALID_DIGIT as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    4 as libc::c_int,
    3 as libc::c_int,
    0 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn vertexNumForDirection(
    origin: H3Index,
    direction: Direction,
) -> libc::c_int {
    let mut isPent: libc::c_int = isPentagon(origin);
    if direction as libc::c_uint == CENTER_DIGIT as libc::c_int as libc::c_uint
        || direction as libc::c_uint >= INVALID_DIGIT as libc::c_int as libc::c_uint
        || isPent != 0 && direction as libc::c_uint == K_AXES_DIGIT as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    let mut rotations: libc::c_int = 0;
    let mut err: H3Error = vertexRotations(origin, &mut rotations);
    if err != 0 {
        return -(1 as libc::c_int);
    }
    if isPent != 0 {
        (directionToVertexNumPent[direction as usize] + 5 as libc::c_int - rotations)
            % 5 as libc::c_int
    } else {
        (directionToVertexNumHex[direction as usize] + 6 as libc::c_int - rotations)
            % 6 as libc::c_int
    }
}
static mut vertexNumToDirectionHex: [Direction; 6] = [
    IJ_AXES_DIGIT,
    J_AXES_DIGIT,
    JK_AXES_DIGIT,
    K_AXES_DIGIT,
    IK_AXES_DIGIT,
    I_AXES_DIGIT,
];
static mut vertexNumToDirectionPent: [Direction; 5] = [
    IJ_AXES_DIGIT,
    J_AXES_DIGIT,
    JK_AXES_DIGIT,
    IK_AXES_DIGIT,
    I_AXES_DIGIT,
];
#[no_mangle]
pub unsafe extern "C" fn directionForVertexNum(
    origin: H3Index,
    vertexNum: libc::c_int,
) -> Direction {
    let mut isPent: libc::c_int = isPentagon(origin);
    if vertexNum < 0 as libc::c_int
        || vertexNum
            > (if isPent != 0 {
                5 as libc::c_int
            } else {
                6 as libc::c_int
            }) - 1 as libc::c_int
    {
        return INVALID_DIGIT;
    }
    let mut rotations: libc::c_int = 0;
    let mut err: H3Error = vertexRotations(origin, &mut rotations);
    if err != 0 {
        return INVALID_DIGIT;
    }
    (if isPent != 0 {
        vertexNumToDirectionPent[((vertexNum + rotations) % 5 as libc::c_int) as usize]
            as libc::c_uint
    } else {
        vertexNumToDirectionHex[((vertexNum + rotations) % 6 as libc::c_int) as usize]
            as libc::c_uint
    }) as Direction
}
static mut DIRECTIONS: [Direction; 6] = [
    J_AXES_DIGIT,
    JK_AXES_DIGIT,
    K_AXES_DIGIT,
    IK_AXES_DIGIT,
    I_AXES_DIGIT,
    IJ_AXES_DIGIT,
];
static mut revNeighborDirectionsHex: [libc::c_int; 7] = [
    INVALID_DIGIT as libc::c_int,
    5 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    2 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn cellToVertex(
    mut cell: H3Index,
    mut vertexNum: libc::c_int,
    mut out: *mut H3Index,
) -> H3Error {
    let mut cellIsPentagon: libc::c_int = isPentagon(cell);
    let mut cellNumVerts: libc::c_int = if cellIsPentagon != 0 {
        5 as libc::c_int
    } else {
        6 as libc::c_int
    };
    let mut res: libc::c_int = ((cell & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int;
    if vertexNum < 0 as libc::c_int || vertexNum > cellNumVerts - 1 as libc::c_int {
        return E_DOMAIN as libc::c_int as H3Error;
    }
    let mut owner: H3Index = cell;
    let mut ownerVertexNum: libc::c_int = vertexNum;
    if res == 0 as libc::c_int
        || (cell >> ((15 as libc::c_int - res) * 3 as libc::c_int) & 7 as libc::c_int as uint64_t)
            as Direction as libc::c_uint
            != CENTER_DIGIT as libc::c_int as libc::c_uint
    {
        let mut left: Direction = directionForVertexNum(cell, vertexNum);
        if left as libc::c_uint == INVALID_DIGIT as libc::c_int as libc::c_uint {
            return E_FAILED as libc::c_int as H3Error;
        }
        let mut lRotations: libc::c_int = 0 as libc::c_int;
        let mut leftNeighbor: H3Index = 0;
        let mut leftNeighborError: H3Error =
            h3NeighborRotations(cell, left, &mut lRotations, &mut leftNeighbor);
        if leftNeighborError != 0 {
            return leftNeighborError;
        }
        if leftNeighbor < owner {
            owner = leftNeighbor;
        }
        if res == 0 as libc::c_int
            || (leftNeighbor >> ((15 as libc::c_int - res) * 3 as libc::c_int)
                & 7 as libc::c_int as uint64_t) as Direction as libc::c_uint
                != CENTER_DIGIT as libc::c_int as libc::c_uint
        {
            let mut right: Direction = directionForVertexNum(
                cell,
                (vertexNum - 1 as libc::c_int + cellNumVerts) % cellNumVerts,
            );
            if right as libc::c_uint == INVALID_DIGIT as libc::c_int as libc::c_uint {
                return E_FAILED as libc::c_int as H3Error;
            }
            let mut rRotations: libc::c_int = 0 as libc::c_int;
            let mut rightNeighbor: H3Index = 0;
            let mut rightNeighborError: H3Error =
                h3NeighborRotations(cell, right, &mut rRotations, &mut rightNeighbor);
            if rightNeighborError != 0 {
                return rightNeighborError;
            }
            if rightNeighbor < owner {
                owner = rightNeighbor;
                let mut dir: Direction = (if isPentagon(owner) != 0 {
                    directionForNeighbor(owner, cell) as libc::c_uint
                } else {
                    DIRECTIONS[((revNeighborDirectionsHex[right as usize] + rRotations)
                        % 6 as libc::c_int) as usize] as libc::c_uint
                }) as Direction;
                ownerVertexNum = vertexNumForDirection(owner, dir);
            }
        }
        if owner == leftNeighbor {
            let mut ownerIsPentagon: libc::c_int = isPentagon(owner);
            let mut dir_0: Direction = (if ownerIsPentagon != 0 {
                directionForNeighbor(owner, cell) as libc::c_uint
            } else {
                DIRECTIONS[((revNeighborDirectionsHex[left as usize] + lRotations)
                    % 6 as libc::c_int) as usize] as libc::c_uint
            }) as Direction;
            ownerVertexNum = vertexNumForDirection(owner, dir_0) + 1 as libc::c_int;
            if ownerVertexNum == 6 as libc::c_int
                || ownerIsPentagon != 0 && ownerVertexNum == 5 as libc::c_int
            {
                ownerVertexNum = 0 as libc::c_int;
            }
        }
    }
    let mut vertex: H3Index = owner;
    vertex = vertex & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (4 as libc::c_int as uint64_t) << 59 as libc::c_int;
    vertex = vertex & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (ownerVertexNum as uint64_t) << 56 as libc::c_int;
    *out = vertex;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn cellToVertexes(mut cell: H3Index, mut vertexes: *mut H3Index) -> H3Error {
    let mut isPent: bool = isPentagon(cell) != 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if i == 5 as libc::c_int && isPent as libc::c_int != 0 {
            *vertexes.offset(i as isize) = 0 as libc::c_int as H3Index;
        } else {
            let mut cellError: H3Error = cellToVertex(cell, i, &mut *vertexes.offset(i as isize));
            if cellError != 0 {
                return cellError;
            }
        }
        i += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn vertexToLatLng(mut vertex: H3Index, mut coord: *mut LatLng) -> H3Error {
    let mut vertexNum: libc::c_int = ((vertex
        & (7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        >> 56 as libc::c_int) as libc::c_int;
    let mut owner: H3Index = vertex;
    owner = owner & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    owner = owner & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (0 as libc::c_int as uint64_t) << 56 as libc::c_int;
    let mut gb: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut fijk: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    let mut fijkError: H3Error = _h3ToFaceIjk(owner, &mut fijk);
    if fijkError != 0 {
        return fijkError;
    }
    let mut res: libc::c_int = ((owner & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int;
    if isPentagon(owner) != 0 {
        _faceIjkPentToCellBoundary(&mut fijk, res, vertexNum, 1 as libc::c_int, &mut gb);
    } else {
        _faceIjkToCellBoundary(&mut fijk, res, vertexNum, 1 as libc::c_int, &mut gb);
    }
    *coord = gb.verts[0 as libc::c_int as usize];
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn isValidVertex(mut vertex: H3Index) -> libc::c_int {
    if ((vertex & (15 as libc::c_int as uint64_t) << 59 as libc::c_int) >> 59 as libc::c_int)
        as libc::c_int
        != 4 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let mut vertexNum: libc::c_int = ((vertex
        & (7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        >> 56 as libc::c_int) as libc::c_int;
    let mut owner: H3Index = vertex;
    owner = owner & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    owner = owner & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (0 as libc::c_int as uint64_t) << 56 as libc::c_int;
    if isValidCell(owner) == 0 {
        return 0 as libc::c_int;
    }
    let mut canonical: H3Index = 0;
    if cellToVertex(owner, vertexNum, &mut canonical) != 0 {
        return 0 as libc::c_int;
    }
    if vertex == canonical {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
