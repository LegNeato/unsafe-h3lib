use ::libc;
extern "C" {
    pub type __sFILEX;
    fn cellToLocalIj(origin: H3Index, h3: H3Index, mode: uint32_t, out: *mut CoordIJ) -> H3Error;
    fn exit(_: libc::c_int) -> !;
    fn localIjToCell(
        origin: H3Index,
        ij: *const CoordIJ,
        mode: uint32_t,
        out: *mut H3Index,
    ) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _ijkMatches(c1: *const CoordIJK, c2: *const CoordIJK) -> libc::c_int;
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    fn _getBaseCellNeighbor(baseCell: libc::c_int, dir: Direction) -> libc::c_int;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    fn cellToLocalIjk(origin: H3Index, h3: H3Index, out: *mut CoordIJK) -> H3Error;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
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
unsafe extern "C" fn runTests() {
    let mut bc1: H3Index = 35184372088831 as libc::c_ulonglong;
    setH3Index(&mut bc1, 0 as libc::c_int, 15 as libc::c_int, CENTER_DIGIT);
    let mut bc2: H3Index = 35184372088831 as libc::c_ulonglong;
    setH3Index(&mut bc2, 0 as libc::c_int, 8 as libc::c_int, CENTER_DIGIT);
    let mut bc3: H3Index = 35184372088831 as libc::c_ulonglong;
    setH3Index(&mut bc3, 0 as libc::c_int, 31 as libc::c_int, CENTER_DIGIT);
    let mut pent1: H3Index = 35184372088831 as libc::c_ulonglong;
    setH3Index(&mut pent1, 0 as libc::c_int, 4 as libc::c_int, CENTER_DIGIT);
    currentTestName = b"ijkBaseCells\0" as *const u8 as *const libc::c_char;
    let mut ijk: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
    if !(cellToLocalIjk(pent1, bc1, &mut ijk) == E_SUCCESS as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            54 as libc::c_int,
            b"cellToLocalIjk(pent1, bc1, &ijk) == E_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"got ijk for base cells 4 and 15\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(_ijkMatches(
        &mut ijk,
        &*UNIT_VECS.as_ptr().offset(2 as libc::c_int as isize),
    ) == 1 as libc::c_int)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            56 as libc::c_int,
            b"_ijkMatches(&ijk, &UNIT_VECS[2]) == 1\0" as *const u8 as *const libc::c_char,
            b"neighboring base cell at 0,1,0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"ijBaseCells\0" as *const u8 as *const libc::c_char;
    let mut ij: CoordIJ = {
        let mut init = CoordIJ {
            i: 0 as libc::c_int,
            j: 0 as libc::c_int,
        };
        init
    };
    let mut origin: H3Index = 0x8029fffffffffff as libc::c_long as H3Index;
    let mut retrieved: H3Index = 0;
    if !(localIjToCell(
        origin,
        &mut ij,
        0 as libc::c_int as uint32_t,
        &mut retrieved,
    ) == E_SUCCESS as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            65 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(origin, &ij, 0, &retrieved) == E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"got origin back\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(retrieved == 0x8029fffffffffff as libc::c_long as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            66 as libc::c_int,
            b"retrieved == 0x8029fffffffffff\0" as *const u8 as *const libc::c_char,
            b"origin matches self\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    ij.i = 1 as libc::c_int;
    if !(localIjToCell(
        origin,
        &mut ij,
        0 as libc::c_int as uint32_t,
        &mut retrieved,
    ) == E_SUCCESS as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            70 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(origin, &ij, 0, &retrieved) == E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"got offset index\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(retrieved == 0x8051fffffffffff as libc::c_long as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            72 as libc::c_int,
            b"retrieved == 0x8051fffffffffff\0" as *const u8 as *const libc::c_char,
            b"modified index matches expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    ij.i = 2 as libc::c_int;
    if !(localIjToCell(
        origin,
        &mut ij,
        0 as libc::c_int as uint32_t,
        &mut retrieved,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            76 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(origin, &ij, 0, &retrieved) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"out of range base cell (1)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    ij.i = 0 as libc::c_int;
    ij.j = 2 as libc::c_int;
    if !(localIjToCell(
        origin,
        &mut ij,
        0 as libc::c_int as uint32_t,
        &mut retrieved,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            81 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(origin, &ij, 0, &retrieved) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"out of range base cell (2)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    ij.i = -(2 as libc::c_int);
    ij.j = -(2 as libc::c_int);
    if !(localIjToCell(
        origin,
        &mut ij,
        0 as libc::c_int as uint32_t,
        &mut retrieved,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            86 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(origin, &ij, 0, &retrieved) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"out of range base cell (3)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"ijOutOfRange\0" as *const u8 as *const libc::c_char;
    let numCoords: libc::c_int = 7 as libc::c_int;
    let coords: [CoordIJ; 7] = [
        {
            let mut init = CoordIJ {
                i: 0 as libc::c_int,
                j: 0 as libc::c_int,
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
                i: 2 as libc::c_int,
                j: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJ {
                i: 3 as libc::c_int,
                j: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJ {
                i: 4 as libc::c_int,
                j: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJ {
                i: -(4 as libc::c_int),
                j: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJ {
                i: 0 as libc::c_int,
                j: 4 as libc::c_int,
            };
            init
        },
    ];
    let expected: [H3Index; 7] = [
        0x81283ffffffffff as libc::c_long as H3Index,
        0x81293ffffffffff as libc::c_long as H3Index,
        0x8150bffffffffff as libc::c_long as H3Index,
        0x8151bffffffffff as libc::c_long as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numCoords {
        let mut result: H3Index = 0;
        let err: H3Error = localIjToCell(
            expected[0 as libc::c_int as usize],
            &*coords.as_ptr().offset(i as isize),
            0 as libc::c_int as uint32_t,
            &mut result,
        );
        if expected[i as usize] == 0 as libc::c_int as libc::c_ulonglong {
            if !(err != 0 as libc::c_int as libc::c_uint) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                        as *const libc::c_char,
                    106 as libc::c_int,
                    b"err != 0\0" as *const u8 as *const libc::c_char,
                    b"coordinates out of range\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        } else {
            if !(err == 0 as libc::c_int as libc::c_uint) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                        as *const libc::c_char,
                    108 as libc::c_int,
                    b"err == 0\0" as *const u8 as *const libc::c_char,
                    b"coordinates in range\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if !(result == expected[i as usize]) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                        as *const libc::c_char,
                    109 as libc::c_int,
                    b"result == expected[i]\0" as *const u8 as *const libc::c_char,
                    b"result matches expectation\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
    currentTestName = b"cellToLocalIjFailed\0" as *const u8 as *const libc::c_char;
    let mut ij_0: CoordIJ = CoordIJ { i: 0, j: 0 };
    if !(cellToLocalIj(bc1, bc1, 0 as libc::c_int as uint32_t, &mut ij_0)
        == 0 as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            118 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(bc1, bc1, 0, &ij) == 0\0" as *const u8
                as *const libc::c_char,
            b"found IJ (1)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ij_0.i == 0 as libc::c_int && ij_0.j == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            119 as libc::c_int,
            b"ij.i == 0 && ij.j == 0\0" as *const u8 as *const libc::c_char,
            b"ij correct (1)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellToLocalIj(bc1, pent1, 0 as libc::c_int as uint32_t, &mut ij_0)
        == 0 as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            121 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(bc1, pent1, 0, &ij) == 0\0" as *const u8
                as *const libc::c_char,
            b"found IJ (2)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ij_0.i == 1 as libc::c_int && ij_0.j == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            122 as libc::c_int,
            b"ij.i == 1 && ij.j == 0\0" as *const u8 as *const libc::c_char,
            b"ij correct (2)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellToLocalIj(bc1, bc2, 0 as libc::c_int as uint32_t, &mut ij_0)
        == 0 as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            124 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(bc1, bc2, 0, &ij) == 0\0" as *const u8
                as *const libc::c_char,
            b"found IJ (3)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ij_0.i == 0 as libc::c_int && ij_0.j == -(1 as libc::c_int)) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            125 as libc::c_int,
            b"ij.i == 0 && ij.j == -1\0" as *const u8 as *const libc::c_char,
            b"ij correct (3)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellToLocalIj(bc1, bc3, 0 as libc::c_int as uint32_t, &mut ij_0)
        == 0 as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            127 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(bc1, bc3, 0, &ij) == 0\0" as *const u8
                as *const libc::c_char,
            b"found IJ (4)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ij_0.i == -(1 as libc::c_int) && ij_0.j == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            128 as libc::c_int,
            b"ij.i == -1 && ij.j == 0\0" as *const u8 as *const libc::c_char,
            b"ij correct (4)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellToLocalIj(pent1, bc3, 0 as libc::c_int as uint32_t, &mut ij_0)
        == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            130 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(pent1, bc3, 0, &ij) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"found IJ (5)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToLocalIjInvalid\0" as *const u8 as *const libc::c_char;
    let mut ij_1: CoordIJ = CoordIJ { i: 0, j: 0 };
    let mut invalidIndex: H3Index = 0x7fffffffffffffff as libc::c_long as H3Index;
    invalidIndex = invalidIndex & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (((bc1 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
            as libc::c_int as uint64_t)
            << 52 as libc::c_int;
    if !(cellToLocalIj(bc1, invalidIndex, 0 as libc::c_int as uint32_t, &mut ij_1)
        == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            139 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(bc1, invalidIndex, 0, &ij) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"invalid index\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellToLocalIj(
        0x7fffffffffffffff as libc::c_long as H3Index,
        bc1,
        0 as libc::c_int as uint32_t,
        &mut ij_1,
    ) == E_RES_MISMATCH as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            142 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(0x7fffffffffffffff, bc1, 0, &ij) == E_RES_MISMATCH\0"
                as *const u8 as *const libc::c_char,
            b"invalid origin\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellToLocalIj(
        0x7fffffffffffffff as libc::c_long as H3Index,
        0x7fffffffffffffff as libc::c_long as H3Index,
        0 as libc::c_int as uint32_t,
        &mut ij_1,
    ) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0"
                as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(0x7fffffffffffffff, 0x7fffffffffffffff, 0, &ij) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"invalid origin and index\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"localIjToCellInvalid\0" as *const u8 as *const libc::c_char;
    let mut ij_2: CoordIJ = {
        let mut init = CoordIJ {
            i: 0 as libc::c_int,
            j: 0 as libc::c_int,
        };
        init
    };
    let mut index: H3Index = 0;
    if !(localIjToCell(
        0x7fffffffffffffff as libc::c_long as H3Index,
        &mut ij_2,
        0 as libc::c_int as uint32_t,
        &mut index,
    ) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            154 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(0x7fffffffffffffff, &ij, 0, &index) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"invalid origin for ijToH3\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"indexOnPentInvalid\0" as *const u8 as *const libc::c_char;
    let mut onPentInvalid: H3Index = 0;
    setH3Index(
        &mut onPentInvalid,
        1 as libc::c_int,
        4 as libc::c_int,
        INVALID_DIGIT,
    );
    let mut offPent: H3Index = 0;
    setH3Index(
        &mut offPent,
        1 as libc::c_int,
        3 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut ij_3: CoordIJ = CoordIJ { i: 0, j: 0 };
    if !(cellToLocalIj(
        offPent,
        onPentInvalid,
        0 as libc::c_int as uint32_t,
        &mut ij_3,
    ) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            168 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(offPent, onPentInvalid, 0, &ij) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"invalid index on pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut onPentValid: H3Index = 0;
    setH3Index(
        &mut onPentValid,
        1 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    if !(cellToLocalIj(
        onPentInvalid,
        onPentValid,
        0 as libc::c_int as uint32_t,
        &mut ij_3,
    ) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            174 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(onPentInvalid, onPentValid, 0, &ij) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"invalid both on pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(cellToLocalIj(
        onPentValid,
        onPentInvalid,
        0 as libc::c_int as uint32_t,
        &mut ij_3,
    ) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            177 as libc::c_int,
            b"H3_EXPORT(cellToLocalIj)(onPentValid, onPentInvalid, 0, &ij) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"invalid both on pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    ij_3.i = 0 as libc::c_int;
    ij_3.j = 0 as libc::c_int;
    let mut out: H3Index = 0;
    if !(localIjToCell(
        onPentInvalid,
        &mut ij_3,
        0 as libc::c_int as uint32_t,
        &mut out,
    ) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            184 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(onPentInvalid, &ij, 0, &out) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"invalid both on pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    ij_3.i = 3 as libc::c_int;
    ij_3.j = 3 as libc::c_int;
    if !(localIjToCell(
        onPentInvalid,
        &mut ij_3,
        0 as libc::c_int as uint32_t,
        &mut out,
    ) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            190 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(onPentInvalid, &ij, 0, &out) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"invalid origin on pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"onOffPentagonSame\0" as *const u8 as *const libc::c_char;
    let mut bc: libc::c_int = 0 as libc::c_int;
    while bc < 122 as libc::c_int {
        let mut res: libc::c_int = 1 as libc::c_int;
        while res <= 15 as libc::c_int {
            let mut startDir: Direction = K_AXES_DIGIT;
            if _isBaseCellPentagon(bc) != 0 {
                startDir += 1;
            }
            let mut dir: Direction = startDir;
            while (dir as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
                let mut internalOrigin: H3Index = 0;
                setH3Index(&mut internalOrigin, res, bc, dir);
                let mut externalOrigin: H3Index = 0;
                setH3Index(
                    &mut externalOrigin,
                    res,
                    _getBaseCellNeighbor(bc, dir),
                    CENTER_DIGIT,
                );
                let mut testDir: Direction = startDir;
                while (testDir as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
                    let mut testIndex: H3Index = 0;
                    setH3Index(&mut testIndex, res, bc, testDir);
                    let mut internalIj: CoordIJ = CoordIJ { i: 0, j: 0 };
                    let mut internalIjFailed: libc::c_int = cellToLocalIj(
                        internalOrigin,
                        testIndex,
                        0 as libc::c_int as uint32_t,
                        &mut internalIj,
                    ) as libc::c_int;
                    let mut externalIj: CoordIJ = CoordIJ { i: 0, j: 0 };
                    let mut externalIjFailed: libc::c_int = cellToLocalIj(
                        externalOrigin,
                        testIndex,
                        0 as libc::c_int as uint32_t,
                        &mut externalIj,
                    ) as libc::c_int;
                    if !((internalIjFailed != 0) as libc::c_int
                        == (externalIjFailed != 0) as libc::c_int)
                    {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0"
                                as *const u8 as *const libc::c_char,
                            229 as libc::c_int,
                            b"(bool)internalIjFailed == (bool)externalIjFailed\0" as *const u8
                                as *const libc::c_char,
                            b"internal/external failed matches when getting IJ\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    if !(internalIjFailed != 0) {
                        let mut internalIndex: H3Index = 0;
                        let mut internalIjFailed2: libc::c_int = localIjToCell(
                            internalOrigin,
                            &mut internalIj,
                            0 as libc::c_int as uint32_t,
                            &mut internalIndex,
                        )
                            as libc::c_int;
                        let mut externalIndex: H3Index = 0;
                        let mut externalIjFailed2: libc::c_int = localIjToCell(
                            externalOrigin,
                            &mut externalIj,
                            0 as libc::c_int as uint32_t,
                            &mut externalIndex,
                        )
                            as libc::c_int;
                        if !((internalIjFailed2 != 0) as libc::c_int
                            == (externalIjFailed2 != 0) as libc::c_int)
                        {
                            fprintf(
                                __stderrp,
                                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                    as *const libc::c_char,
                                currentSuiteName,
                                currentTestName,
                                b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0"
                                    as *const u8
                                    as *const libc::c_char,
                                245 as libc::c_int,
                                b"(bool)internalIjFailed2 == (bool)externalIjFailed2\0" as *const u8
                                    as *const libc::c_char,
                                b"internal/external failed matches when getting index\0"
                                    as *const u8
                                    as *const libc::c_char,
                            );
                            exit(1 as libc::c_int);
                        }
                        globalTestCount += 1;
                        printf(b".\0" as *const u8 as *const libc::c_char);
                        if !(internalIjFailed2 != 0) {
                            if !(internalIndex == externalIndex) {
                                fprintf(
                                    __stderrp,
                                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    currentSuiteName,
                                    currentTestName,
                                    b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0"
                                        as *const u8 as *const libc::c_char,
                                    252 as libc::c_int,
                                    b"internalIndex == externalIndex\0" as *const u8
                                        as *const libc::c_char,
                                    b"internal/external index matches\0" as *const u8
                                        as *const libc::c_char,
                                );
                                exit(1 as libc::c_int);
                            }
                            globalTestCount += 1;
                            printf(b".\0" as *const u8 as *const libc::c_char);
                        }
                    }
                    testDir += 1;
                }
                dir += 1;
            }
            res += 1;
        }
        bc += 1;
    }
    currentTestName = b"invalidMode\0" as *const u8 as *const libc::c_char;
    let mut ij_4: CoordIJ = CoordIJ { i: 0, j: 0 };
    let mut cell: H3Index = 0x85283473fffffff as libc::c_long as H3Index;
    if cellToLocalIj(cell, cell, 0 as libc::c_int as uint32_t, &mut ij_4) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            262 as libc::c_int,
            b"!(cellToLocalIj(cell, cell, 0, &ij))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i_0: uint32_t = 1 as libc::c_int as uint32_t;
    while i_0 <= 32 as libc::c_int as libc::c_uint {
        let mut ij2: CoordIJ = CoordIJ { i: 0, j: 0 };
        if !(cellToLocalIj(cell, cell, i_0, &mut ij2)
            == E_OPTION_INVALID as libc::c_int as libc::c_uint)
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                    as *const libc::c_char,
                268 as libc::c_int,
                b"H3_EXPORT(cellToLocalIj)(cell, cell, i, &ij2) == E_OPTION_INVALID\0" as *const u8
                    as *const libc::c_char,
                b"Invalid mode fail for cellToLocalIj\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut cell2: H3Index = 0;
        if !(localIjToCell(cell, &mut ij2, i_0, &mut cell2)
            == E_OPTION_INVALID as libc::c_int as libc::c_uint)
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                    as *const libc::c_char,
                272 as libc::c_int,
                b"H3_EXPORT(localIjToCell)(cell, &ij2, i, &cell2) == E_OPTION_INVALID\0"
                    as *const u8 as *const libc::c_char,
                b"Invalid mode fail for cellToLocalIj\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_0 = i_0.wrapping_add(1);
    }
    currentTestName = b"invalid_negativeIj\0" as *const u8 as *const libc::c_char;
    let mut index_0: H3Index = 0x200f202020202020 as libc::c_long as H3Index;
    let mut ij_5: CoordIJ = {
        let mut init = CoordIJ {
            i: -(14671840 as libc::c_int),
            j: -(2147483648 as libc::c_long) as libc::c_int,
        };
        init
    };
    let mut out_0: H3Index = 0;
    if !(localIjToCell(index_0, &mut ij_5, 0 as libc::c_int as uint32_t, &mut out_0)
        == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            281 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(index, &ij, 0, &out) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"Negative I and J components fail\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"localIjToCell_overflow_i\0" as *const u8 as *const libc::c_char;
    let mut origin_0: H3Index = 0;
    setH3Index(
        &mut origin_0,
        2 as libc::c_int,
        2 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut ij_6: CoordIJ = {
        let mut init = CoordIJ {
            i: -(2147483647 as libc::c_int) - 1 as libc::c_int,
            j: 2147483647 as libc::c_int,
        };
        init
    };
    let mut out_1: H3Index = 0;
    if !(localIjToCell(
        origin_0,
        &mut ij_6,
        0 as libc::c_int as uint32_t,
        &mut out_1,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            290 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(origin, &ij, 0, &out) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"High magnitude I and J components fail\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"localIjToCell_overflow_j\0" as *const u8 as *const libc::c_char;
    let mut origin_1: H3Index = 0;
    setH3Index(
        &mut origin_1,
        2 as libc::c_int,
        2 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut ij_7: CoordIJ = {
        let mut init = CoordIJ {
            i: 2147483647 as libc::c_int,
            j: -(2147483647 as libc::c_int) - 1 as libc::c_int,
        };
        init
    };
    let mut out_2: H3Index = 0;
    if !(localIjToCell(
        origin_1,
        &mut ij_7,
        0 as libc::c_int as uint32_t,
        &mut out_2,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            299 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(origin, &ij, 0, &out) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"High magnitude J and I components fail\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"localIjToCell_overflow_ij\0" as *const u8 as *const libc::c_char;
    let mut origin_2: H3Index = 0;
    setH3Index(
        &mut origin_2,
        2 as libc::c_int,
        2 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut ij_8: CoordIJ = {
        let mut init = CoordIJ {
            i: -(2147483647 as libc::c_int) - 1 as libc::c_int,
            j: -(2147483647 as libc::c_int) - 1 as libc::c_int,
        };
        init
    };
    let mut out_3: H3Index = 0;
    if !(localIjToCell(
        origin_2,
        &mut ij_8,
        0 as libc::c_int as uint32_t,
        &mut out_3,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testCellToLocalIj.c\0" as *const u8
                as *const libc::c_char,
            308 as libc::c_int,
            b"H3_EXPORT(localIjToCell)(origin, &ij, 0, &out) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"High magnitude J and I components fail\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
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
