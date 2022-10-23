use ::libc;
extern "C" {
    pub type __sFILEX;
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn gridDiskDistances(
        origin: H3Index,
        k: libc::c_int,
        out: *mut H3Index,
        distances: *mut libc::c_int,
    ) -> H3Error;
    fn localIjToCell(
        origin: H3Index,
        ij: *const CoordIJ,
        mode: uint32_t,
        out: *mut H3Index,
    ) -> H3Error;
    fn cellToLocalIj(origin: H3Index, h3: H3Index, mode: uint32_t, out: *mut CoordIJ) -> H3Error;
    fn exit(_: libc::c_int) -> !;
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _ijkMatches(c1: *const CoordIJK, c2: *const CoordIJK) -> libc::c_int;
    fn _ijkAdd(h1: *const CoordIJK, h2: *const CoordIJK, sum: *mut CoordIJK);
    fn _ijkNormalize(c: *mut CoordIJK);
    fn _downAp7r(ijk: *mut CoordIJK);
    fn _neighbor(ijk: *mut CoordIJK, digit: Direction);
    fn _ijkRotate60ccw(ijk: *mut CoordIJK);
    fn h3NeighborRotations(
        origin: H3Index,
        dir: Direction,
        rotations: *mut libc::c_int,
        out: *mut H3Index,
    ) -> H3Error;
    fn ijToIjk(ij: *const CoordIJ, ijk: *mut CoordIJK) -> H3Error;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn iterateAllIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    );
    fn iterateAllIndexesAtResPartial(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
        maxBaseCell: libc::c_int,
    );
}
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
pub struct CoordIJ {
    pub i: libc::c_int,
    pub j: libc::c_int,
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
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
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
static mut UNIT_VECS: [CoordIJK; 7] = [
    {
        let mut init = CoordIJK {
            i: 0 as libc::c_int,
            j: 0 as libc::c_int,
            k: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = CoordIJK {
            i: 0 as libc::c_int,
            j: 0 as libc::c_int,
            k: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = CoordIJK {
            i: 0 as libc::c_int,
            j: 1 as libc::c_int,
            k: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = CoordIJK {
            i: 0 as libc::c_int,
            j: 1 as libc::c_int,
            k: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = CoordIJK {
            i: 1 as libc::c_int,
            j: 1 as libc::c_int,
            k: 0 as libc::c_int,
        };
        init
    },
];
static mut MAX_DISTANCES: [libc::c_int; 6] = [
    1 as libc::c_int,
    2 as libc::c_int,
    5 as libc::c_int,
    12 as libc::c_int,
    19 as libc::c_int,
    26 as libc::c_int,
];
static mut DIRECTIONS: [CoordIJ; 6] = [
    {
        let mut init = CoordIJ {
            i: 0 as libc::c_int,
            j: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = CoordIJ {
            i: -(1 as libc::c_int),
            j: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = CoordIJ {
            i: -(1 as libc::c_int),
            j: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = CoordIJ {
            i: 0 as libc::c_int,
            j: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = CoordIJ {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = CoordIJ {
            i: 1 as libc::c_int,
            j: 1 as libc::c_int,
        };
        init
    },
];
static mut NEXT_RING_DIRECTION: CoordIJ = {
    let mut init = CoordIJ {
        i: 1 as libc::c_int,
        j: 0 as libc::c_int,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn localIjToH3_identity_assertions(mut h3: H3Index) {
    let mut ij: CoordIJ = CoordIJ { i: 0, j: 0 };
    if !(cellToLocalIj(h3, h3, 0 as libc::c_int as uint32_t, &mut ij)
        == 0 as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            52 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(h3, h3, 0, &ij) == 0\0" as *const u8 as *const libc::c_char,
            b"able to setup localIjToH3 test\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut retrieved: H3Index = 0;
    if !(localIjToCell(h3, &mut ij, 0 as libc::c_int as uint32_t, &mut retrieved)
        == 0 as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            56 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(h3, &ij, 0, &retrieved) == 0\0" as *const u8
                as *const libc::c_char,
            b"got an index back from localIjTOh3\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(h3 == retrieved) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            57 as libc::c_int,
            b"h3 == retrieved\0" as *const u8 as *const libc::c_char,
            b"round trip through local IJ space works\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn h3ToLocalIj_coordinates_assertions(mut h3: H3Index) {
    let mut r: libc::c_int =
        ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    let mut ij: CoordIJ = CoordIJ { i: 0, j: 0 };
    if !(cellToLocalIj(h3, h3, 0 as libc::c_int as uint32_t, &mut ij)
        == 0 as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            70 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(h3, h3, 0, &ij) == 0\0" as *const u8 as *const libc::c_char,
            b"get ij for origin\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut ijk: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
    if ijToIjk(&mut ij, &mut ijk) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            72 as libc::c_int,
            b"!(ijToIjk(&ij, &ijk))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if r == 0 as libc::c_int {
        if !(_ijkMatches(
            &mut ijk,
            &*UNIT_VECS.as_ptr().offset(0 as libc::c_int as isize),
        ) == 1 as libc::c_int)
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                    as *const u8 as *const libc::c_char,
                74 as libc::c_int,
                b"_ijkMatches(&ijk, &UNIT_VECS[0]) == 1\0" as *const u8 as *const libc::c_char,
                b"res 0 cell at 0,0,0\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
    } else if r == 1 as libc::c_int {
        if !(_ijkMatches(
            &mut ijk,
            &*UNIT_VECS.as_ptr().offset(
                (h3 >> (15 as libc::c_int - 1 as libc::c_int) * 3 as libc::c_int
                    & 7 as libc::c_int as uint64_t) as Direction as isize,
            ),
        ) == 1 as libc::c_int)
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                    as *const u8 as *const libc::c_char,
                77 as libc::c_int,
                b"_ijkMatches(&ijk, &UNIT_VECS[H3_GET_INDEX_DIGIT(h3, 1)]) == 1\0" as *const u8
                    as *const libc::c_char,
                b"res 1 cell at expected coordinates\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
    } else if r == 2 as libc::c_int {
        let mut expected: CoordIJK =
            UNIT_VECS[(h3 >> (15 as libc::c_int - 1 as libc::c_int) * 3 as libc::c_int
                & 7 as libc::c_int as uint64_t) as Direction as usize];
        _downAp7r(&mut expected);
        _neighbor(
            &mut expected,
            (h3 >> (15 as libc::c_int - 2 as libc::c_int) * 3 as libc::c_int
                & 7 as libc::c_int as uint64_t) as Direction,
        );
        if !(_ijkMatches(&mut ijk, &mut expected) == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                    as *const u8 as *const libc::c_char,
                83 as libc::c_int,
                b"_ijkMatches(&ijk, &expected) == 1\0" as *const u8 as *const libc::c_char,
                b"res 2 cell at expected coordinates\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
    } else {
        if 0 as libc::c_int == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                    as *const u8 as *const libc::c_char,
                85 as libc::c_int,
                b"0\0" as *const u8 as *const libc::c_char,
                b"resolution supported by test function (coordinates)\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn h3ToLocalIj_neighbors_assertions(mut h3: H3Index) {
    let mut origin: CoordIJ = {
        let mut init = CoordIJ {
            i: 0 as libc::c_int,
            j: 0,
        };
        init
    };
    if !(cellToLocalIj(h3, h3, 0 as libc::c_int as uint32_t, &mut origin)
        == 0 as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            96 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(h3, h3, 0, &origin) == 0\0" as *const u8
                as *const libc::c_char,
            b"got ij for origin\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut originIjk: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
    if ijToIjk(&mut origin, &mut originIjk) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            98 as libc::c_int,
            b"!(ijToIjk(&origin, &originIjk))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut d: Direction = K_AXES_DIGIT;
    while (d as libc::c_uint) < INVALID_DIGIT as libc::c_int as libc::c_uint {
        if !(d as libc::c_uint == K_AXES_DIGIT as libc::c_int as libc::c_uint
            && isPentagon(h3) != 0)
        {
            let mut rotations: libc::c_int = 0 as libc::c_int;
            let mut offset: H3Index = 0;
            if h3NeighborRotations(h3, d, &mut rotations, &mut offset) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    107 as libc::c_int,
                    b"!(h3NeighborRotations(h3, d, &rotations, &offset))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut ij: CoordIJ = {
                let mut init = CoordIJ {
                    i: 0 as libc::c_int,
                    j: 0,
                };
                init
            };
            if !(cellToLocalIj(h3, offset, 0 as libc::c_int as uint32_t, &mut ij)
                == 0 as libc::c_int as libc::c_uint)
            {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    111 as libc::c_int,
                    b"H3_EXPORT(cellToLocalIj)(h3, offset, 0, &ij) == 0\0" as *const u8
                        as *const libc::c_char,
                    b"got ij for destination\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut ijk: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
            if ijToIjk(&mut ij, &mut ijk) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    113 as libc::c_int,
                    b"!(ijToIjk(&ij, &ijk))\0" as *const u8 as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut invertedIjk: CoordIJK = {
                let mut init = CoordIJK {
                    i: 0 as libc::c_int,
                    j: 0,
                    k: 0,
                };
                init
            };
            _neighbor(&mut invertedIjk, d);
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                _ijkRotate60ccw(&mut invertedIjk);
                i += 1;
            }
            _ijkAdd(&mut invertedIjk, &mut ijk, &mut ijk);
            _ijkNormalize(&mut ijk);
            if _ijkMatches(&mut ijk, &mut originIjk) == 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    122 as libc::c_int,
                    b"_ijkMatches(&ijk, &originIjk)\0" as *const u8 as *const libc::c_char,
                    b"back to origin\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        d += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn h3ToLocalIj_invalid_assertions(mut h3: H3Index) {
    let mut r: libc::c_int =
        ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if !(r > 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            132 as libc::c_int,
            b"r > 0\0" as *const u8 as *const libc::c_char,
            b"resolution supported by test function (invalid digits)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(r <= 5 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            133 as libc::c_int,
            b"r <= 5\0" as *const u8 as *const libc::c_char,
            b"resolution supported by test function (invalid digits)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut maxK: libc::c_int = MAX_DISTANCES[r as usize];
    let mut sz: int64_t = 0;
    if maxGridDiskSize(maxK, &mut sz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            137 as libc::c_int,
            b"!(maxGridDiskSize(maxK, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut neighbors: *mut H3Index = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let mut distances: *mut libc::c_int = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if gridDiskDistances(h3, maxK, neighbors, distances) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            142 as libc::c_int,
            b"!(gridDiskDistances(h3, maxK, neighbors, distances))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < sz {
        if !(*neighbors.offset(i as isize) == 0 as libc::c_int as libc::c_ulonglong) {
            let mut ij: CoordIJ = CoordIJ { i: 0, j: 0 };
            if cellToLocalIj(
                h3,
                *neighbors.offset(i as isize),
                0 as libc::c_int as uint32_t,
                &mut ij,
            ) == E_SUCCESS as libc::c_int as libc::c_uint
            {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j < 2 as libc::c_int {
                    let mut dir: Direction = (if j == 0 as libc::c_int {
                        INVALID_DIGIT as libc::c_int
                    } else {
                        K_AXES_DIGIT as libc::c_int
                    }) as Direction;
                    let mut h3Invalid: H3Index = h3;
                    h3Invalid = h3Invalid
                        & !((7 as libc::c_int as uint64_t)
                            << (15 as libc::c_int - 0 as libc::c_int) * 3 as libc::c_int)
                        | (dir as uint64_t)
                            << (15 as libc::c_int - 0 as libc::c_int) * 3 as libc::c_int;
                    let mut ij2: CoordIJ = CoordIJ { i: 0, j: 0 };
                    cellToLocalIj(
                        h3Invalid,
                        *neighbors.offset(i as isize),
                        0 as libc::c_int as uint32_t,
                        &mut ij2,
                    );
                    let mut neighborInvalid: H3Index = *neighbors.offset(i as isize);
                    neighborInvalid = neighborInvalid
                        & !((7 as libc::c_int as uint64_t)
                            << (15 as libc::c_int - 0 as libc::c_int) * 3 as libc::c_int)
                        | (dir as uint64_t)
                            << (15 as libc::c_int - 0 as libc::c_int) * 3 as libc::c_int;
                    cellToLocalIj(h3, neighborInvalid, 0 as libc::c_int as uint32_t, &mut ij2);
                    let mut out: H3Index = 0;
                    localIjToCell(h3Invalid, &mut ij, 0 as libc::c_int as uint32_t, &mut out);
                    j += 1;
                }
            }
        }
        i += 1;
    }
    free(distances as *mut libc::c_void);
    free(neighbors as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn localIjToH3_gridDisk_assertions(mut h3: H3Index) {
    let mut r: libc::c_int =
        ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if !(r <= 5 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            179 as libc::c_int,
            b"r <= 5\0" as *const u8 as *const libc::c_char,
            b"resolution supported by test function (gridDisk)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut maxK: libc::c_int = MAX_DISTANCES[r as usize];
    let mut sz: int64_t = 0;
    if maxGridDiskSize(maxK, &mut sz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            183 as libc::c_int,
            b"!(maxGridDiskSize(maxK, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut neighbors: *mut H3Index = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let mut distances: *mut libc::c_int = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if gridDiskDistances(h3, maxK, neighbors, distances) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            188 as libc::c_int,
            b"!(gridDiskDistances(h3, maxK, neighbors, distances))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < sz {
        if !(*neighbors.offset(i as isize) == 0 as libc::c_int as libc::c_ulonglong) {
            let mut ij: CoordIJ = CoordIJ { i: 0, j: 0 };
            if cellToLocalIj(
                h3,
                *neighbors.offset(i as isize),
                0 as libc::c_int as uint32_t,
                &mut ij,
            ) == 0 as libc::c_int as libc::c_uint
            {
                let mut retrieved: H3Index = 0;
                if !(localIjToCell(h3, &mut ij, 0 as libc::c_int as uint32_t, &mut retrieved)
                    == 0 as libc::c_int as libc::c_uint)
                {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                            as *const u8 as *const libc::c_char,
                        200 as libc::c_int,
                        b"H3_EXPORT(localIjToCell)(h3, &ij, 0, &retrieved) == 0\0" as *const u8
                            as *const libc::c_char,
                        b"retrieved index for unfolded coordinates\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                if !(retrieved == *neighbors.offset(i as isize)) {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                            as *const u8 as *const libc::c_char,
                        202 as libc::c_int,
                        b"retrieved == neighbors[i]\0" as *const u8 as *const libc::c_char,
                        b"round trip neighboring index matches expected\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
            }
        }
        i += 1;
    }
    free(distances as *mut libc::c_void);
    free(neighbors as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn localIjToH3_traverse_assertions(mut h3: H3Index) {
    let mut r: libc::c_int =
        ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if !(r <= 5 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            212 as libc::c_int,
            b"r <= 5\0" as *const u8 as *const libc::c_char,
            b"resolution supported by test function (traverse)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut k: libc::c_int = MAX_DISTANCES[r as usize];
    let mut ij: CoordIJ = CoordIJ { i: 0, j: 0 };
    if !(cellToLocalIj(h3, h3, 0 as libc::c_int as uint32_t, &mut ij)
        == 0 as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0" as *const u8
                as *const libc::c_char,
            217 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(h3, h3, 0, &ij) == 0\0" as *const u8 as *const libc::c_char,
            b"Got origin coordinates\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut ring: libc::c_int = 1 as libc::c_int;
    let mut direction: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while ring <= k {
        if direction == 0 as libc::c_int && i == 0 as libc::c_int {
            ij.i += NEXT_RING_DIRECTION.i;
            ij.j += NEXT_RING_DIRECTION.j;
        }
        ij.i += DIRECTIONS[direction as usize].i;
        ij.j += DIRECTIONS[direction as usize].j;
        let mut testH3: H3Index = 0;
        let mut failed: libc::c_int =
            localIjToCell(h3, &mut ij, 0 as libc::c_int as uint32_t, &mut testH3) as libc::c_int;
        if failed == 0 {
            if isValidCell(testH3) == 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    241 as libc::c_int,
                    b"H3_EXPORT(isValidCell)(testH3)\0" as *const u8 as *const libc::c_char,
                    b"test coordinates result in valid index\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut expectedIj: CoordIJ = CoordIJ { i: 0, j: 0 };
            let mut reverseFailed: libc::c_int =
                cellToLocalIj(h3, testH3, 0 as libc::c_int as uint32_t, &mut expectedIj)
                    as libc::c_int;
            if reverseFailed == 0 {
                if expectedIj.i != ij.i || expectedIj.j != ij.j {
                    let mut testTestH3: H3Index = 0;
                    if !(localIjToCell(
                        h3,
                        &mut expectedIj,
                        0 as libc::c_int as uint32_t,
                        &mut testTestH3,
                    ) == 0 as libc::c_int as libc::c_uint)
                    {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                                as *const u8 as *const libc::c_char,
                            256 as libc::c_int,
                            b"H3_EXPORT(localIjToCell)(h3, &expectedIj, 0, &testTestH3) == 0\0"
                                as *const u8 as *const libc::c_char,
                            b"converted coordinates again\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    if !(testH3 == testTestH3) {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testCellToLocalIjExhaustive.c\0"
                                as *const u8 as *const libc::c_char,
                            259 as libc::c_int,
                            b"testH3 == testTestH3\0" as *const u8
                                as *const libc::c_char,
                            b"index has normalizable coordinates in local IJ\0"
                                as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                }
            }
        }
        i += 1;
        if i == ring {
            i = 0 as libc::c_int;
            direction += 1;
            if direction == 6 as libc::c_int {
                direction = 0 as libc::c_int;
                ring += 1;
            }
        }
    }
}
unsafe extern "C" fn runTests() {
    currentTestName = b"localIjToH3_identity\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(localIjToH3_identity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(localIjToH3_identity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(localIjToH3_identity_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"h3ToLocalIj_coordinates\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(h3ToLocalIj_coordinates_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(h3ToLocalIj_coordinates_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(h3ToLocalIj_coordinates_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"h3ToLocalIj_neighbors\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(h3ToLocalIj_neighbors_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(h3ToLocalIj_neighbors_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(h3ToLocalIj_neighbors_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"h3ToLocalIj_invalid\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(h3ToLocalIj_invalid_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(h3ToLocalIj_invalid_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"localIjToH3_gridDisk\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(localIjToH3_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(localIjToH3_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(localIjToH3_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtResPartial(
        3 as libc::c_int,
        Some(localIjToH3_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
        27 as libc::c_int,
    );
    currentTestName = b"localIjToH3_traverse\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(localIjToH3_traverse_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(localIjToH3_traverse_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(localIjToH3_traverse_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtResPartial(
        3 as libc::c_int,
        Some(localIjToH3_traverse_assertions as unsafe extern "C" fn(H3Index) -> ()),
        27 as libc::c_int,
    );
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"h3ToLocalIj\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"h3ToLocalIj\0" as *const u8 as *const libc::c_char,
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
