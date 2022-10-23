use ::libc;
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
pub struct CoordIJK {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub k: libc::c_int,
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
pub struct BaseCellData {
    pub homeFijk: FaceIJK,
    pub isPentagon: libc::c_int,
    pub cwOffsetPent: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseCellRotation {
    pub baseCell: libc::c_int,
    pub ccwRot60: libc::c_int,
}
#[no_mangle]
pub static mut baseCellNeighbors: [[libc::c_int; 7]; 122] = [
    [
        0 as libc::c_int,
        1 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        8 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        7 as libc::c_int,
        6 as libc::c_int,
        9 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        2 as libc::c_int,
        6 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        5 as libc::c_int,
    ],
    [
        3 as libc::c_int,
        13 as libc::c_int,
        1 as libc::c_int,
        7 as libc::c_int,
        4 as libc::c_int,
        12 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        4 as libc::c_int,
        127 as libc::c_int,
        15 as libc::c_int,
        8 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        12 as libc::c_int,
    ],
    [
        5 as libc::c_int,
        2 as libc::c_int,
        18 as libc::c_int,
        10 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        16 as libc::c_int,
    ],
    [
        6 as libc::c_int,
        14 as libc::c_int,
        11 as libc::c_int,
        17 as libc::c_int,
        1 as libc::c_int,
        9 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        7 as libc::c_int,
        21 as libc::c_int,
        9 as libc::c_int,
        19 as libc::c_int,
        3 as libc::c_int,
        13 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        8 as libc::c_int,
        5 as libc::c_int,
        22 as libc::c_int,
        16 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        15 as libc::c_int,
    ],
    [
        9 as libc::c_int,
        19 as libc::c_int,
        14 as libc::c_int,
        20 as libc::c_int,
        1 as libc::c_int,
        7 as libc::c_int,
        6 as libc::c_int,
    ],
    [
        10 as libc::c_int,
        11 as libc::c_int,
        24 as libc::c_int,
        23 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        18 as libc::c_int,
    ],
    [
        11 as libc::c_int,
        17 as libc::c_int,
        23 as libc::c_int,
        25 as libc::c_int,
        2 as libc::c_int,
        6 as libc::c_int,
        10 as libc::c_int,
    ],
    [
        12 as libc::c_int,
        28 as libc::c_int,
        13 as libc::c_int,
        26 as libc::c_int,
        4 as libc::c_int,
        15 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        13 as libc::c_int,
        26 as libc::c_int,
        21 as libc::c_int,
        29 as libc::c_int,
        3 as libc::c_int,
        12 as libc::c_int,
        7 as libc::c_int,
    ],
    [
        14 as libc::c_int,
        127 as libc::c_int,
        17 as libc::c_int,
        27 as libc::c_int,
        9 as libc::c_int,
        20 as libc::c_int,
        6 as libc::c_int,
    ],
    [
        15 as libc::c_int,
        22 as libc::c_int,
        28 as libc::c_int,
        31 as libc::c_int,
        4 as libc::c_int,
        8 as libc::c_int,
        12 as libc::c_int,
    ],
    [
        16 as libc::c_int,
        18 as libc::c_int,
        33 as libc::c_int,
        30 as libc::c_int,
        8 as libc::c_int,
        5 as libc::c_int,
        22 as libc::c_int,
    ],
    [
        17 as libc::c_int,
        11 as libc::c_int,
        14 as libc::c_int,
        6 as libc::c_int,
        35 as libc::c_int,
        25 as libc::c_int,
        27 as libc::c_int,
    ],
    [
        18 as libc::c_int,
        24 as libc::c_int,
        30 as libc::c_int,
        32 as libc::c_int,
        5 as libc::c_int,
        10 as libc::c_int,
        16 as libc::c_int,
    ],
    [
        19 as libc::c_int,
        34 as libc::c_int,
        20 as libc::c_int,
        36 as libc::c_int,
        7 as libc::c_int,
        21 as libc::c_int,
        9 as libc::c_int,
    ],
    [
        20 as libc::c_int,
        14 as libc::c_int,
        19 as libc::c_int,
        9 as libc::c_int,
        40 as libc::c_int,
        27 as libc::c_int,
        36 as libc::c_int,
    ],
    [
        21 as libc::c_int,
        38 as libc::c_int,
        19 as libc::c_int,
        34 as libc::c_int,
        13 as libc::c_int,
        29 as libc::c_int,
        7 as libc::c_int,
    ],
    [
        22 as libc::c_int,
        16 as libc::c_int,
        41 as libc::c_int,
        33 as libc::c_int,
        15 as libc::c_int,
        8 as libc::c_int,
        31 as libc::c_int,
    ],
    [
        23 as libc::c_int,
        24 as libc::c_int,
        11 as libc::c_int,
        10 as libc::c_int,
        39 as libc::c_int,
        37 as libc::c_int,
        25 as libc::c_int,
    ],
    [
        24 as libc::c_int,
        127 as libc::c_int,
        32 as libc::c_int,
        37 as libc::c_int,
        10 as libc::c_int,
        23 as libc::c_int,
        18 as libc::c_int,
    ],
    [
        25 as libc::c_int,
        23 as libc::c_int,
        17 as libc::c_int,
        11 as libc::c_int,
        45 as libc::c_int,
        39 as libc::c_int,
        35 as libc::c_int,
    ],
    [
        26 as libc::c_int,
        42 as libc::c_int,
        29 as libc::c_int,
        43 as libc::c_int,
        12 as libc::c_int,
        28 as libc::c_int,
        13 as libc::c_int,
    ],
    [
        27 as libc::c_int,
        40 as libc::c_int,
        35 as libc::c_int,
        46 as libc::c_int,
        14 as libc::c_int,
        20 as libc::c_int,
        17 as libc::c_int,
    ],
    [
        28 as libc::c_int,
        31 as libc::c_int,
        42 as libc::c_int,
        44 as libc::c_int,
        12 as libc::c_int,
        15 as libc::c_int,
        26 as libc::c_int,
    ],
    [
        29 as libc::c_int,
        43 as libc::c_int,
        38 as libc::c_int,
        47 as libc::c_int,
        13 as libc::c_int,
        26 as libc::c_int,
        21 as libc::c_int,
    ],
    [
        30 as libc::c_int,
        32 as libc::c_int,
        48 as libc::c_int,
        50 as libc::c_int,
        16 as libc::c_int,
        18 as libc::c_int,
        33 as libc::c_int,
    ],
    [
        31 as libc::c_int,
        41 as libc::c_int,
        44 as libc::c_int,
        53 as libc::c_int,
        15 as libc::c_int,
        22 as libc::c_int,
        28 as libc::c_int,
    ],
    [
        32 as libc::c_int,
        30 as libc::c_int,
        24 as libc::c_int,
        18 as libc::c_int,
        52 as libc::c_int,
        50 as libc::c_int,
        37 as libc::c_int,
    ],
    [
        33 as libc::c_int,
        30 as libc::c_int,
        49 as libc::c_int,
        48 as libc::c_int,
        22 as libc::c_int,
        16 as libc::c_int,
        41 as libc::c_int,
    ],
    [
        34 as libc::c_int,
        19 as libc::c_int,
        38 as libc::c_int,
        21 as libc::c_int,
        54 as libc::c_int,
        36 as libc::c_int,
        51 as libc::c_int,
    ],
    [
        35 as libc::c_int,
        46 as libc::c_int,
        45 as libc::c_int,
        56 as libc::c_int,
        17 as libc::c_int,
        27 as libc::c_int,
        25 as libc::c_int,
    ],
    [
        36 as libc::c_int,
        20 as libc::c_int,
        34 as libc::c_int,
        19 as libc::c_int,
        55 as libc::c_int,
        40 as libc::c_int,
        54 as libc::c_int,
    ],
    [
        37 as libc::c_int,
        39 as libc::c_int,
        52 as libc::c_int,
        57 as libc::c_int,
        24 as libc::c_int,
        23 as libc::c_int,
        32 as libc::c_int,
    ],
    [
        38 as libc::c_int,
        127 as libc::c_int,
        34 as libc::c_int,
        51 as libc::c_int,
        29 as libc::c_int,
        47 as libc::c_int,
        21 as libc::c_int,
    ],
    [
        39 as libc::c_int,
        37 as libc::c_int,
        25 as libc::c_int,
        23 as libc::c_int,
        59 as libc::c_int,
        57 as libc::c_int,
        45 as libc::c_int,
    ],
    [
        40 as libc::c_int,
        27 as libc::c_int,
        36 as libc::c_int,
        20 as libc::c_int,
        60 as libc::c_int,
        46 as libc::c_int,
        55 as libc::c_int,
    ],
    [
        41 as libc::c_int,
        49 as libc::c_int,
        53 as libc::c_int,
        61 as libc::c_int,
        22 as libc::c_int,
        33 as libc::c_int,
        31 as libc::c_int,
    ],
    [
        42 as libc::c_int,
        58 as libc::c_int,
        43 as libc::c_int,
        62 as libc::c_int,
        28 as libc::c_int,
        44 as libc::c_int,
        26 as libc::c_int,
    ],
    [
        43 as libc::c_int,
        62 as libc::c_int,
        47 as libc::c_int,
        64 as libc::c_int,
        26 as libc::c_int,
        42 as libc::c_int,
        29 as libc::c_int,
    ],
    [
        44 as libc::c_int,
        53 as libc::c_int,
        58 as libc::c_int,
        65 as libc::c_int,
        28 as libc::c_int,
        31 as libc::c_int,
        42 as libc::c_int,
    ],
    [
        45 as libc::c_int,
        39 as libc::c_int,
        35 as libc::c_int,
        25 as libc::c_int,
        63 as libc::c_int,
        59 as libc::c_int,
        56 as libc::c_int,
    ],
    [
        46 as libc::c_int,
        60 as libc::c_int,
        56 as libc::c_int,
        68 as libc::c_int,
        27 as libc::c_int,
        40 as libc::c_int,
        35 as libc::c_int,
    ],
    [
        47 as libc::c_int,
        38 as libc::c_int,
        43 as libc::c_int,
        29 as libc::c_int,
        69 as libc::c_int,
        51 as libc::c_int,
        64 as libc::c_int,
    ],
    [
        48 as libc::c_int,
        49 as libc::c_int,
        30 as libc::c_int,
        33 as libc::c_int,
        67 as libc::c_int,
        66 as libc::c_int,
        50 as libc::c_int,
    ],
    [
        49 as libc::c_int,
        127 as libc::c_int,
        61 as libc::c_int,
        66 as libc::c_int,
        33 as libc::c_int,
        48 as libc::c_int,
        41 as libc::c_int,
    ],
    [
        50 as libc::c_int,
        48 as libc::c_int,
        32 as libc::c_int,
        30 as libc::c_int,
        70 as libc::c_int,
        67 as libc::c_int,
        52 as libc::c_int,
    ],
    [
        51 as libc::c_int,
        69 as libc::c_int,
        54 as libc::c_int,
        71 as libc::c_int,
        38 as libc::c_int,
        47 as libc::c_int,
        34 as libc::c_int,
    ],
    [
        52 as libc::c_int,
        57 as libc::c_int,
        70 as libc::c_int,
        74 as libc::c_int,
        32 as libc::c_int,
        37 as libc::c_int,
        50 as libc::c_int,
    ],
    [
        53 as libc::c_int,
        61 as libc::c_int,
        65 as libc::c_int,
        75 as libc::c_int,
        31 as libc::c_int,
        41 as libc::c_int,
        44 as libc::c_int,
    ],
    [
        54 as libc::c_int,
        71 as libc::c_int,
        55 as libc::c_int,
        73 as libc::c_int,
        34 as libc::c_int,
        51 as libc::c_int,
        36 as libc::c_int,
    ],
    [
        55 as libc::c_int,
        40 as libc::c_int,
        54 as libc::c_int,
        36 as libc::c_int,
        72 as libc::c_int,
        60 as libc::c_int,
        73 as libc::c_int,
    ],
    [
        56 as libc::c_int,
        68 as libc::c_int,
        63 as libc::c_int,
        77 as libc::c_int,
        35 as libc::c_int,
        46 as libc::c_int,
        45 as libc::c_int,
    ],
    [
        57 as libc::c_int,
        59 as libc::c_int,
        74 as libc::c_int,
        78 as libc::c_int,
        37 as libc::c_int,
        39 as libc::c_int,
        52 as libc::c_int,
    ],
    [
        58 as libc::c_int,
        127 as libc::c_int,
        62 as libc::c_int,
        76 as libc::c_int,
        44 as libc::c_int,
        65 as libc::c_int,
        42 as libc::c_int,
    ],
    [
        59 as libc::c_int,
        63 as libc::c_int,
        78 as libc::c_int,
        79 as libc::c_int,
        39 as libc::c_int,
        45 as libc::c_int,
        57 as libc::c_int,
    ],
    [
        60 as libc::c_int,
        72 as libc::c_int,
        68 as libc::c_int,
        80 as libc::c_int,
        40 as libc::c_int,
        55 as libc::c_int,
        46 as libc::c_int,
    ],
    [
        61 as libc::c_int,
        53 as libc::c_int,
        49 as libc::c_int,
        41 as libc::c_int,
        81 as libc::c_int,
        75 as libc::c_int,
        66 as libc::c_int,
    ],
    [
        62 as libc::c_int,
        43 as libc::c_int,
        58 as libc::c_int,
        42 as libc::c_int,
        82 as libc::c_int,
        64 as libc::c_int,
        76 as libc::c_int,
    ],
    [
        63 as libc::c_int,
        127 as libc::c_int,
        56 as libc::c_int,
        45 as libc::c_int,
        79 as libc::c_int,
        59 as libc::c_int,
        77 as libc::c_int,
    ],
    [
        64 as libc::c_int,
        47 as libc::c_int,
        62 as libc::c_int,
        43 as libc::c_int,
        84 as libc::c_int,
        69 as libc::c_int,
        82 as libc::c_int,
    ],
    [
        65 as libc::c_int,
        58 as libc::c_int,
        53 as libc::c_int,
        44 as libc::c_int,
        86 as libc::c_int,
        76 as libc::c_int,
        75 as libc::c_int,
    ],
    [
        66 as libc::c_int,
        67 as libc::c_int,
        81 as libc::c_int,
        85 as libc::c_int,
        49 as libc::c_int,
        48 as libc::c_int,
        61 as libc::c_int,
    ],
    [
        67 as libc::c_int,
        66 as libc::c_int,
        50 as libc::c_int,
        48 as libc::c_int,
        87 as libc::c_int,
        85 as libc::c_int,
        70 as libc::c_int,
    ],
    [
        68 as libc::c_int,
        56 as libc::c_int,
        60 as libc::c_int,
        46 as libc::c_int,
        90 as libc::c_int,
        77 as libc::c_int,
        80 as libc::c_int,
    ],
    [
        69 as libc::c_int,
        51 as libc::c_int,
        64 as libc::c_int,
        47 as libc::c_int,
        89 as libc::c_int,
        71 as libc::c_int,
        84 as libc::c_int,
    ],
    [
        70 as libc::c_int,
        67 as libc::c_int,
        52 as libc::c_int,
        50 as libc::c_int,
        83 as libc::c_int,
        87 as libc::c_int,
        74 as libc::c_int,
    ],
    [
        71 as libc::c_int,
        89 as libc::c_int,
        73 as libc::c_int,
        91 as libc::c_int,
        51 as libc::c_int,
        69 as libc::c_int,
        54 as libc::c_int,
    ],
    [
        72 as libc::c_int,
        127 as libc::c_int,
        73 as libc::c_int,
        55 as libc::c_int,
        80 as libc::c_int,
        60 as libc::c_int,
        88 as libc::c_int,
    ],
    [
        73 as libc::c_int,
        91 as libc::c_int,
        72 as libc::c_int,
        88 as libc::c_int,
        54 as libc::c_int,
        71 as libc::c_int,
        55 as libc::c_int,
    ],
    [
        74 as libc::c_int,
        78 as libc::c_int,
        83 as libc::c_int,
        92 as libc::c_int,
        52 as libc::c_int,
        57 as libc::c_int,
        70 as libc::c_int,
    ],
    [
        75 as libc::c_int,
        65 as libc::c_int,
        61 as libc::c_int,
        53 as libc::c_int,
        94 as libc::c_int,
        86 as libc::c_int,
        81 as libc::c_int,
    ],
    [
        76 as libc::c_int,
        86 as libc::c_int,
        82 as libc::c_int,
        96 as libc::c_int,
        58 as libc::c_int,
        65 as libc::c_int,
        62 as libc::c_int,
    ],
    [
        77 as libc::c_int,
        63 as libc::c_int,
        68 as libc::c_int,
        56 as libc::c_int,
        93 as libc::c_int,
        79 as libc::c_int,
        90 as libc::c_int,
    ],
    [
        78 as libc::c_int,
        74 as libc::c_int,
        59 as libc::c_int,
        57 as libc::c_int,
        95 as libc::c_int,
        92 as libc::c_int,
        79 as libc::c_int,
    ],
    [
        79 as libc::c_int,
        78 as libc::c_int,
        63 as libc::c_int,
        59 as libc::c_int,
        93 as libc::c_int,
        95 as libc::c_int,
        77 as libc::c_int,
    ],
    [
        80 as libc::c_int,
        68 as libc::c_int,
        72 as libc::c_int,
        60 as libc::c_int,
        99 as libc::c_int,
        90 as libc::c_int,
        88 as libc::c_int,
    ],
    [
        81 as libc::c_int,
        85 as libc::c_int,
        94 as libc::c_int,
        101 as libc::c_int,
        61 as libc::c_int,
        66 as libc::c_int,
        75 as libc::c_int,
    ],
    [
        82 as libc::c_int,
        96 as libc::c_int,
        84 as libc::c_int,
        98 as libc::c_int,
        62 as libc::c_int,
        76 as libc::c_int,
        64 as libc::c_int,
    ],
    [
        83 as libc::c_int,
        127 as libc::c_int,
        74 as libc::c_int,
        70 as libc::c_int,
        100 as libc::c_int,
        87 as libc::c_int,
        92 as libc::c_int,
    ],
    [
        84 as libc::c_int,
        69 as libc::c_int,
        82 as libc::c_int,
        64 as libc::c_int,
        97 as libc::c_int,
        89 as libc::c_int,
        98 as libc::c_int,
    ],
    [
        85 as libc::c_int,
        87 as libc::c_int,
        101 as libc::c_int,
        102 as libc::c_int,
        66 as libc::c_int,
        67 as libc::c_int,
        81 as libc::c_int,
    ],
    [
        86 as libc::c_int,
        76 as libc::c_int,
        75 as libc::c_int,
        65 as libc::c_int,
        104 as libc::c_int,
        96 as libc::c_int,
        94 as libc::c_int,
    ],
    [
        87 as libc::c_int,
        83 as libc::c_int,
        102 as libc::c_int,
        100 as libc::c_int,
        67 as libc::c_int,
        70 as libc::c_int,
        85 as libc::c_int,
    ],
    [
        88 as libc::c_int,
        72 as libc::c_int,
        91 as libc::c_int,
        73 as libc::c_int,
        99 as libc::c_int,
        80 as libc::c_int,
        105 as libc::c_int,
    ],
    [
        89 as libc::c_int,
        97 as libc::c_int,
        91 as libc::c_int,
        103 as libc::c_int,
        69 as libc::c_int,
        84 as libc::c_int,
        71 as libc::c_int,
    ],
    [
        90 as libc::c_int,
        77 as libc::c_int,
        80 as libc::c_int,
        68 as libc::c_int,
        106 as libc::c_int,
        93 as libc::c_int,
        99 as libc::c_int,
    ],
    [
        91 as libc::c_int,
        73 as libc::c_int,
        89 as libc::c_int,
        71 as libc::c_int,
        105 as libc::c_int,
        88 as libc::c_int,
        103 as libc::c_int,
    ],
    [
        92 as libc::c_int,
        83 as libc::c_int,
        78 as libc::c_int,
        74 as libc::c_int,
        108 as libc::c_int,
        100 as libc::c_int,
        95 as libc::c_int,
    ],
    [
        93 as libc::c_int,
        79 as libc::c_int,
        90 as libc::c_int,
        77 as libc::c_int,
        109 as libc::c_int,
        95 as libc::c_int,
        106 as libc::c_int,
    ],
    [
        94 as libc::c_int,
        86 as libc::c_int,
        81 as libc::c_int,
        75 as libc::c_int,
        107 as libc::c_int,
        104 as libc::c_int,
        101 as libc::c_int,
    ],
    [
        95 as libc::c_int,
        92 as libc::c_int,
        79 as libc::c_int,
        78 as libc::c_int,
        109 as libc::c_int,
        108 as libc::c_int,
        93 as libc::c_int,
    ],
    [
        96 as libc::c_int,
        104 as libc::c_int,
        98 as libc::c_int,
        110 as libc::c_int,
        76 as libc::c_int,
        86 as libc::c_int,
        82 as libc::c_int,
    ],
    [
        97 as libc::c_int,
        127 as libc::c_int,
        98 as libc::c_int,
        84 as libc::c_int,
        103 as libc::c_int,
        89 as libc::c_int,
        111 as libc::c_int,
    ],
    [
        98 as libc::c_int,
        110 as libc::c_int,
        97 as libc::c_int,
        111 as libc::c_int,
        82 as libc::c_int,
        96 as libc::c_int,
        84 as libc::c_int,
    ],
    [
        99 as libc::c_int,
        80 as libc::c_int,
        105 as libc::c_int,
        88 as libc::c_int,
        106 as libc::c_int,
        90 as libc::c_int,
        113 as libc::c_int,
    ],
    [
        100 as libc::c_int,
        102 as libc::c_int,
        83 as libc::c_int,
        87 as libc::c_int,
        108 as libc::c_int,
        114 as libc::c_int,
        92 as libc::c_int,
    ],
    [
        101 as libc::c_int,
        102 as libc::c_int,
        107 as libc::c_int,
        112 as libc::c_int,
        81 as libc::c_int,
        85 as libc::c_int,
        94 as libc::c_int,
    ],
    [
        102 as libc::c_int,
        101 as libc::c_int,
        87 as libc::c_int,
        85 as libc::c_int,
        114 as libc::c_int,
        112 as libc::c_int,
        100 as libc::c_int,
    ],
    [
        103 as libc::c_int,
        91 as libc::c_int,
        97 as libc::c_int,
        89 as libc::c_int,
        116 as libc::c_int,
        105 as libc::c_int,
        111 as libc::c_int,
    ],
    [
        104 as libc::c_int,
        107 as libc::c_int,
        110 as libc::c_int,
        115 as libc::c_int,
        86 as libc::c_int,
        94 as libc::c_int,
        96 as libc::c_int,
    ],
    [
        105 as libc::c_int,
        88 as libc::c_int,
        103 as libc::c_int,
        91 as libc::c_int,
        113 as libc::c_int,
        99 as libc::c_int,
        116 as libc::c_int,
    ],
    [
        106 as libc::c_int,
        93 as libc::c_int,
        99 as libc::c_int,
        90 as libc::c_int,
        117 as libc::c_int,
        109 as libc::c_int,
        113 as libc::c_int,
    ],
    [
        107 as libc::c_int,
        127 as libc::c_int,
        101 as libc::c_int,
        94 as libc::c_int,
        115 as libc::c_int,
        104 as libc::c_int,
        112 as libc::c_int,
    ],
    [
        108 as libc::c_int,
        100 as libc::c_int,
        95 as libc::c_int,
        92 as libc::c_int,
        118 as libc::c_int,
        114 as libc::c_int,
        109 as libc::c_int,
    ],
    [
        109 as libc::c_int,
        108 as libc::c_int,
        93 as libc::c_int,
        95 as libc::c_int,
        117 as libc::c_int,
        118 as libc::c_int,
        106 as libc::c_int,
    ],
    [
        110 as libc::c_int,
        98 as libc::c_int,
        104 as libc::c_int,
        96 as libc::c_int,
        119 as libc::c_int,
        111 as libc::c_int,
        115 as libc::c_int,
    ],
    [
        111 as libc::c_int,
        97 as libc::c_int,
        110 as libc::c_int,
        98 as libc::c_int,
        116 as libc::c_int,
        103 as libc::c_int,
        119 as libc::c_int,
    ],
    [
        112 as libc::c_int,
        107 as libc::c_int,
        102 as libc::c_int,
        101 as libc::c_int,
        120 as libc::c_int,
        115 as libc::c_int,
        114 as libc::c_int,
    ],
    [
        113 as libc::c_int,
        99 as libc::c_int,
        116 as libc::c_int,
        105 as libc::c_int,
        117 as libc::c_int,
        106 as libc::c_int,
        121 as libc::c_int,
    ],
    [
        114 as libc::c_int,
        112 as libc::c_int,
        100 as libc::c_int,
        102 as libc::c_int,
        118 as libc::c_int,
        120 as libc::c_int,
        108 as libc::c_int,
    ],
    [
        115 as libc::c_int,
        110 as libc::c_int,
        107 as libc::c_int,
        104 as libc::c_int,
        120 as libc::c_int,
        119 as libc::c_int,
        112 as libc::c_int,
    ],
    [
        116 as libc::c_int,
        103 as libc::c_int,
        119 as libc::c_int,
        111 as libc::c_int,
        113 as libc::c_int,
        105 as libc::c_int,
        121 as libc::c_int,
    ],
    [
        117 as libc::c_int,
        127 as libc::c_int,
        109 as libc::c_int,
        118 as libc::c_int,
        113 as libc::c_int,
        121 as libc::c_int,
        106 as libc::c_int,
    ],
    [
        118 as libc::c_int,
        120 as libc::c_int,
        108 as libc::c_int,
        114 as libc::c_int,
        117 as libc::c_int,
        121 as libc::c_int,
        109 as libc::c_int,
    ],
    [
        119 as libc::c_int,
        111 as libc::c_int,
        115 as libc::c_int,
        110 as libc::c_int,
        121 as libc::c_int,
        116 as libc::c_int,
        120 as libc::c_int,
    ],
    [
        120 as libc::c_int,
        115 as libc::c_int,
        114 as libc::c_int,
        112 as libc::c_int,
        121 as libc::c_int,
        119 as libc::c_int,
        118 as libc::c_int,
    ],
    [
        121 as libc::c_int,
        116 as libc::c_int,
        120 as libc::c_int,
        119 as libc::c_int,
        117 as libc::c_int,
        113 as libc::c_int,
        118 as libc::c_int,
    ],
];
#[no_mangle]
pub static mut baseCellNeighbor60CCWRots: [[libc::c_int; 7]; 122] = [
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        1 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        1 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
    ],
];
static mut faceIjkBaseCells: [[[[BaseCellRotation; 3]; 3]; 3]; 20] = [
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 16 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 18 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 24 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 33 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 30 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 32 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 49 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 48 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 50 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 8 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 5 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 10 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 22 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 16 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 18 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 41 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 33 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 30 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 4 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 0 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 2 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 15 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 8 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 5 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 31 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 22 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 16 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 2 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 6 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 14 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 10 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 11 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 17 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 24 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 23 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 25 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 0 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 1 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 9 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 5 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 2 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 6 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 18 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 10 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 11 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 4 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 3 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 7 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 8 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 0 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 1 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 16 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 5 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 2 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 7 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 21 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 38 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 9 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 19 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 34 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 14 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 20 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 36 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 3 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 13 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 29 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 1 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 7 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 21 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 6 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 9 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 19 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 4 as libc::c_int,
                        ccwRot60: 2 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 12 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 26 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 0 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 3 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 13 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 2 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 1 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 7 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 26 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 42 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 58 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 29 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 43 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 62 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 38 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 47 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 64 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 12 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 28 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 44 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 13 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 26 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 42 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 21 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 29 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 43 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 4 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 15 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 31 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 3 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 12 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 28 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 7 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 13 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 26 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 31 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 41 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 49 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 44 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 53 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 61 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 58 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 65 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 75 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 15 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 22 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 33 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 28 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 31 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 41 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 42 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 44 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 53 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 4 as libc::c_int,
                        ccwRot60: 4 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 8 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 16 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 12 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 15 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 22 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 26 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 28 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 31 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 50 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 48 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 49 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 32 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 30 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 33 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 24 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 18 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 16 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 70 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 67 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 66 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 52 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 50 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 48 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 37 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 32 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 30 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 83 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 87 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 85 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 74 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 70 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 67 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 57 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 52 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 50 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 25 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 23 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 24 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 17 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 11 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 10 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 14 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 6 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 2 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 45 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 39 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 37 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 35 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 25 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 23 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 27 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 17 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 11 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 63 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 59 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 57 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 56 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 45 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 39 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 46 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 35 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 25 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 36 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 20 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 14 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 34 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 19 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 9 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 38 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 21 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 7 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 55 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 40 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 27 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 54 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 36 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 20 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 51 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 34 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 19 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 72 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 60 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 46 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 73 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 55 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 40 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 71 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 54 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 36 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 64 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 47 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 38 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 62 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 43 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 29 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 58 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 42 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 26 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 84 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 69 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 51 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 82 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 64 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 47 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 76 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 62 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 43 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 97 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 89 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 71 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 98 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 84 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 69 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 96 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 82 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 64 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 75 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 65 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 58 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 61 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 53 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 44 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 49 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 41 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 31 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 94 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 86 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 76 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 81 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 75 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 65 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 66 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 61 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 53 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 107 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 104 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 96 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 101 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 94 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 86 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 85 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 81 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 75 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 57 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 59 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 63 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 74 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 78 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 79 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 83 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 92 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 95 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 37 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 39 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 45 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 52 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 57 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 59 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 70 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 74 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 78 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 24 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 23 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 25 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 32 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 37 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 39 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 50 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 52 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 57 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 46 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 60 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 72 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 56 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 68 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 80 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 63 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 77 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 90 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 27 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 40 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 55 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 35 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 46 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 60 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 45 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 56 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 68 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 14 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 20 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 36 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 17 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 27 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 40 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 25 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 35 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 46 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 71 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 89 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 97 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 73 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 91 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 103 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 72 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 88 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 105 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 51 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 69 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 84 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 54 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 71 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 89 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 55 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 73 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 91 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 38 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 47 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 64 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 34 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 51 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 69 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 36 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 54 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 71 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 96 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 104 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 107 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 98 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 110 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 115 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 97 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 111 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 119 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 76 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 86 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 94 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 82 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 96 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 104 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 84 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 98 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 110 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 58 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 65 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 75 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 62 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 76 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 86 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 64 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 82 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 96 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 85 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 87 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 83 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 101 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 102 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 100 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 107 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 112 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 114 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 66 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 67 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 70 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 81 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 85 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 87 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 94 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 101 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 102 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 49 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 48 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 50 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 61 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 66 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 67 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 75 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 81 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 85 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 95 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 92 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 83 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 79 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 78 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 74 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 63 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 59 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 57 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 109 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 108 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 100 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 93 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 95 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 92 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 77 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 79 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 78 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 117 as libc::c_int,
                        ccwRot60: 4 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 118 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 114 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 106 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 109 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 108 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 90 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 93 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 95 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 90 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 77 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 63 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 80 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 68 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 56 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 72 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 60 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 46 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 106 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 93 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 79 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 99 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 90 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 77 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 88 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 80 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 68 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 117 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 109 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 95 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 113 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 106 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 93 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 105 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 99 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 90 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 105 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 88 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 72 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 103 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 91 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 73 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 97 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 89 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 71 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 113 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 99 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 80 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 116 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 105 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 88 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 111 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 103 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 91 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 117 as libc::c_int,
                        ccwRot60: 2 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 106 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 90 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 121 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 113 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 99 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 119 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 116 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 105 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 119 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 111 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 97 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 115 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 110 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 98 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 107 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 104 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 96 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 121 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 116 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 103 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 120 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 119 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 111 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 112 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 115 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 110 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 117 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 113 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 105 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 118 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 121 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 116 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 114 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 120 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 119 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
    [
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 114 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 112 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 107 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 100 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 102 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 101 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 83 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 87 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 85 as libc::c_int,
                        ccwRot60: 3 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 118 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 120 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 115 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 108 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 114 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 112 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 92 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 100 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 102 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
        [
            [
                {
                    BaseCellRotation {
                        baseCell: 117 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 121 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 119 as libc::c_int,
                        ccwRot60: 5 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 109 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 118 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 120 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
            [
                {
                    BaseCellRotation {
                        baseCell: 95 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 108 as libc::c_int,
                        ccwRot60: 1 as libc::c_int,
                    }
                },
                {
                    BaseCellRotation {
                        baseCell: 114 as libc::c_int,
                        ccwRot60: 0 as libc::c_int,
                    }
                },
            ],
        ],
    ],
];
#[no_mangle]
pub static mut baseCellData: [BaseCellData; 122] = [
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 1 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 2 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 1 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 2 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 0 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [-(1 as libc::c_int), -(1 as libc::c_int)],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 1 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 1 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 2 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 0 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 2 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 1 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 1 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 3 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 3 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 11 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [2 as libc::c_int, 6 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 4 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 0 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 6 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 0 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 2 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 7 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 2 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 0 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 6 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 10 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [1 as libc::c_int, 5 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 6 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 3 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 11 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 4 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 3 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 0 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 4 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 5 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 0 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 7 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 11 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 7 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 10 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 12 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [3 as libc::c_int, 7 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 6 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 7 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 4 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 3 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 3 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 4 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 6 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 11 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 8 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 5 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 14 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 9 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 5 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 12 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 10 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 4 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 12 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 7 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 11 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 10 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 13 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [4 as libc::c_int, 8 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 10 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 11 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 9 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 8 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 6 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [11 as libc::c_int, 15 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 8 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 9 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 14 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 5 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 16 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 8 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 5 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 12 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 7 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [12 as libc::c_int, 16 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 12 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 10 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 9 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 13 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 16 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 15 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 15 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 16 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 14 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 13 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 5 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [10 as libc::c_int, 19 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 8 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 14 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 9 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 14 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 17 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 12 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 16 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 17 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 15 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 16 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 9 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 15 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 13 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 8 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [13 as libc::c_int, 17 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 13 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 17 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 19 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 14 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 19 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 17 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 13 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 17 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 16 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 9 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [14 as libc::c_int, 18 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 15 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 15 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 18 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 18 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 19 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 17 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 19 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 18 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 1 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 18 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 19 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 2 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 1 as libc::c_int,
            cwOffsetPent: [-(1 as libc::c_int), -(1 as libc::c_int)],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 19 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 18 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 0 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 19 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 1 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
    {
        BaseCellData {
            homeFijk: {
                FaceIJK {
                    face: 18 as libc::c_int,
                    coord: {
                        CoordIJK {
                            i: 1 as libc::c_int,
                            j: 0 as libc::c_int,
                            k: 0 as libc::c_int,
                        }
                    },
                }
            },
            isPentagon: 0 as libc::c_int,
            cwOffsetPent: [0 as libc::c_int, 0 as libc::c_int],
        }
    },
];
#[no_mangle]
pub unsafe extern "C" fn _isBaseCellPentagon(mut baseCell: libc::c_int) -> libc::c_int {
    if baseCell < 0 as libc::c_int || baseCell >= 122 as libc::c_int {
        return 0 as libc::c_int;
    }
    baseCellData[baseCell as usize].isPentagon
}
#[no_mangle]
pub unsafe extern "C" fn _isBaseCellPolarPentagon(mut baseCell: libc::c_int) -> bool {
    baseCell == 4 as libc::c_int || baseCell == 117 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn _faceIjkToBaseCell(mut h: *const FaceIJK) -> libc::c_int {
    faceIjkBaseCells[(*h).face as usize][(*h).coord.i as usize][(*h).coord.j as usize]
        [(*h).coord.k as usize]
        .baseCell
}
#[no_mangle]
pub unsafe extern "C" fn _faceIjkToBaseCellCCWrot60(mut h: *const FaceIJK) -> libc::c_int {
    faceIjkBaseCells[(*h).face as usize][(*h).coord.i as usize][(*h).coord.j as usize]
        [(*h).coord.k as usize]
        .ccwRot60
}
#[no_mangle]
pub unsafe extern "C" fn _baseCellToFaceIjk(mut baseCell: libc::c_int, mut h: *mut FaceIJK) {
    *h = baseCellData[baseCell as usize].homeFijk;
}
#[no_mangle]
pub unsafe extern "C" fn _baseCellToCCWrot60(
    mut baseCell: libc::c_int,
    mut face: libc::c_int,
) -> libc::c_int {
    if face < 0 as libc::c_int || face > 20 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < 3 as libc::c_int {
                if faceIjkBaseCells[face as usize][i as usize][j as usize][k as usize].baseCell
                    == baseCell
                {
                    return faceIjkBaseCells[face as usize][i as usize][j as usize][k as usize]
                        .ccwRot60;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    -(1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn _baseCellIsCwOffset(
    mut baseCell: libc::c_int,
    mut testFace: libc::c_int,
) -> bool {
    baseCellData[baseCell as usize].cwOffsetPent[0 as libc::c_int as usize] == testFace
        || baseCellData[baseCell as usize].cwOffsetPent[1 as libc::c_int as usize] == testFace
}
#[no_mangle]
pub unsafe extern "C" fn _getBaseCellNeighbor(
    mut baseCell: libc::c_int,
    mut dir: Direction,
) -> libc::c_int {
    baseCellNeighbors[baseCell as usize][dir as usize]
}
#[no_mangle]
pub unsafe extern "C" fn _getBaseCellDirection(
    mut originBaseCell: libc::c_int,
    mut neighboringBaseCell: libc::c_int,
) -> Direction {
    let mut dir: Direction = CENTER_DIGIT;
    while (dir as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
        let mut testBaseCell: libc::c_int = _getBaseCellNeighbor(originBaseCell, dir);
        if testBaseCell == neighboringBaseCell {
            return dir;
        }
        dir += 1;
    }
    INVALID_DIGIT
}
#[no_mangle]
pub unsafe extern "C" fn res0CellCount() -> libc::c_int {
    122 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn getRes0Cells(mut out: *mut H3Index) -> H3Error {
    let mut bc: libc::c_int = 0 as libc::c_int;
    while bc < 122 as libc::c_int {
        let mut baseCell: H3Index = 35184372088831 as libc::c_ulonglong;
        baseCell = baseCell & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
            | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
        baseCell = baseCell & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
            | (bc as uint64_t) << 45 as libc::c_int;
        *out.offset(bc as isize) = baseCell;
        bc += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
