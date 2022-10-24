#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib_miscapps;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    fn _baseCellToFaceIjk(baseCell: libc::c_int, h: *mut FaceIJK);
    fn _faceIjkToBaseCellCCWrot60(h: *const FaceIJK) -> libc::c_int;
    fn _faceIjkToBaseCell(h: *const FaceIJK) -> libc::c_int;
    fn _isBaseCellPolarPentagon(baseCell: libc::c_int) -> bool;
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    fn _ijkRotate60cw(ijk: *mut CoordIJK);
    fn _ijkRotate60ccw(ijk: *mut CoordIJK);
    fn _neighbor(ijk: *mut CoordIJK, digit: Direction);
    fn _unitIjkToDigit(ijk: *const CoordIJK) -> Direction;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    pub _extra: *mut libc::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaceIJK {
    pub face: libc::c_int,
    pub coord: CoordIJK,
}
#[no_mangle]
pub static mut NUM_DIRS: libc::c_int = 6 as libc::c_int;
unsafe extern "C" fn auditBaseCellNeighbors(
    mut baseCellNeighbors: *mut [libc::c_int; 7],
    mut baseCellRotations: *mut [libc::c_int; 7],
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 122 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j <= NUM_DIRS {
            if (*baseCellNeighbors.offset(i as isize))[j as usize] != 127 as libc::c_int {
                let mut ourDir: CoordIJK = {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                };
                _neighbor(&mut ourDir, j as Direction);
                let mut k: libc::c_int = 0 as libc::c_int;
                while k <= NUM_DIRS {
                    if (*baseCellNeighbors
                        .offset((*baseCellNeighbors.offset(i as isize))[j as usize] as isize))
                        [k as usize]
                        == i
                    {
                        break;
                    }
                    k += 1;
                }
                if k == NUM_DIRS + 1 as libc::c_int {
                    printf(
                        b"MISMATCH between %d and %d\n\0" as *const u8 as *const libc::c_char,
                        i,
                        (*baseCellNeighbors.offset(i as isize))[j as usize],
                    );
                }
                let mut theirDir: CoordIJK = {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                };
                _neighbor(&mut theirDir, k as Direction);
                let mut reverse: libc::c_int = 0 as libc::c_int;
                while reverse < 3 as libc::c_int {
                    _ijkRotate60ccw(&mut ourDir);
                    reverse += 1;
                }
                let mut rotate: libc::c_int = 0 as libc::c_int;
                while rotate < (*baseCellRotations.offset(i as isize))[j as usize] {
                    _ijkRotate60ccw(&mut ourDir);
                    rotate += 1;
                }
                if _isBaseCellPentagon((*baseCellNeighbors.offset(i as isize))[j as usize]) == 0
                    && (ourDir.i != theirDir.i || ourDir.j != theirDir.j || ourDir.k != theirDir.k)
                {
                    printf(
                        b"WRONG DIRECTION between %d and %d\n\0" as *const u8
                            as *const libc::c_char,
                        i,
                        (*baseCellNeighbors.offset(i as isize))[j as usize],
                    );
                }
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn generate() {
    let mut baseCellNeighbors: [[libc::c_int; 7]; 122] = [[0; 7]; 122];
    let mut baseCellRotations: [[libc::c_int; 7]; 122] = [[0; 7]; 122];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 122 as libc::c_int {
        if _isBaseCellPentagon(i) == 0 {
            let mut dir: libc::c_int = CENTER_DIGIT as libc::c_int;
            while dir <= NUM_DIRS {
                let mut fijk: FaceIJK = FaceIJK {
                    face: 0,
                    coord: CoordIJK { i: 0, j: 0, k: 0 },
                };
                _baseCellToFaceIjk(i, &mut fijk);
                _neighbor(&mut fijk.coord, dir as Direction);
                if fijk.coord.i < 3 as libc::c_int
                    && fijk.coord.j < 3 as libc::c_int
                    && fijk.coord.k < 3 as libc::c_int
                {
                    baseCellNeighbors[i as usize][dir as usize] = _faceIjkToBaseCell(&mut fijk);
                    baseCellRotations[i as usize][dir as usize] =
                        _faceIjkToBaseCellCCWrot60(&mut fijk);
                } else {
                    printf(b"UH OH: Went out of bounds\n\0" as *const u8 as *const libc::c_char);
                }
                dir += 1;
            }
        } else {
            baseCellNeighbors[i as usize][0 as libc::c_int as usize] = i;
            baseCellRotations[i as usize][0 as libc::c_int as usize] = 0 as libc::c_int;
            let mut dir_0: libc::c_int = 1 as libc::c_int;
            while dir_0 <= NUM_DIRS {
                baseCellNeighbors[i as usize][dir_0 as usize] = 127 as libc::c_int;
                baseCellRotations[i as usize][dir_0 as usize] = -(1 as libc::c_int);
                dir_0 += 1;
            }
            let mut f: libc::c_int = 0 as libc::c_int;
            while f < 20 as libc::c_int {
                let mut axis: libc::c_int = 0 as libc::c_int;
                while axis < 3 as libc::c_int {
                    let mut fijk_0: FaceIJK = {
                        FaceIJK {
                            face: f,
                            coord: {
                                CoordIJK {
                                    i: 0 as libc::c_int,
                                    j: 0 as libc::c_int,
                                    k: 0 as libc::c_int,
                                }
                            },
                        }
                    };
                    match axis {
                        0 => {
                            fijk_0.coord.k = 2 as libc::c_int;
                        }
                        1 => {
                            fijk_0.coord.j = 2 as libc::c_int;
                        }
                        2 => {
                            fijk_0.coord.i = 2 as libc::c_int;
                        }
                        _ => {}
                    }
                    if _faceIjkToBaseCell(&mut fijk_0) == i {
                        let mut neighborFijk: FaceIJK = {
                            FaceIJK {
                                face: fijk_0.face,
                                coord: {
                                    CoordIJK {
                                        i: fijk_0.coord.i / 2 as libc::c_int,
                                        j: fijk_0.coord.j / 2 as libc::c_int,
                                        k: fijk_0.coord.k / 2 as libc::c_int,
                                    }
                                },
                            }
                        };
                        let mut rotations: libc::c_int = _faceIjkToBaseCellCCWrot60(&mut fijk_0);
                        let mut ijk: CoordIJK = neighborFijk.coord;
                        let mut currRot: libc::c_int = 0 as libc::c_int;
                        while currRot < rotations {
                            _ijkRotate60ccw(&mut ijk);
                            currRot += 1;
                        }
                        let mut currRot_0: libc::c_int = 0 as libc::c_int;
                        while currRot_0 < 3 as libc::c_int {
                            _ijkRotate60ccw(&mut ijk);
                            currRot_0 += 1;
                        }
                        let mut dir_1: Direction = _unitIjkToDigit(&mut ijk);
                        if dir_1 as libc::c_uint == K_AXES_DIGIT as libc::c_int as libc::c_uint {
                            if _isBaseCellPentagon(i) != 0 {
                                _ijkRotate60cw(&mut ijk);
                                _ijkRotate60cw(&mut ijk);
                            } else {
                                _ijkRotate60ccw(&mut ijk);
                            }
                            dir_1 = _unitIjkToDigit(&mut ijk);
                        }
                        let mut rotAdj: libc::c_int = 0 as libc::c_int;
                        if _isBaseCellPolarPentagon(i) {
                            if dir_1 as libc::c_uint == IK_AXES_DIGIT as libc::c_int as libc::c_uint
                            {
                                rotAdj = 2 as libc::c_int;
                            } else if dir_1 as libc::c_uint
                                == IJ_AXES_DIGIT as libc::c_int as libc::c_uint
                            {
                                rotAdj = 4 as libc::c_int;
                            }
                        } else if dir_1 as libc::c_uint
                            == I_AXES_DIGIT as libc::c_int as libc::c_uint
                            || dir_1 as libc::c_uint == IK_AXES_DIGIT as libc::c_int as libc::c_uint
                        {
                            rotAdj = dir_1 as libc::c_int;
                        }
                        rotations = (rotations + rotAdj) % 6 as libc::c_int;
                        let mut neighborBc: libc::c_int = _faceIjkToBaseCell(&mut neighborFijk);
                        if i == 4 as libc::c_int {
                            let mut realNeighbors: [libc::c_int; 7] = [
                                4 as libc::c_int,
                                127 as libc::c_int,
                                15 as libc::c_int,
                                8 as libc::c_int,
                                3 as libc::c_int,
                                0 as libc::c_int,
                                12 as libc::c_int,
                            ];
                            neighborBc = realNeighbors[dir_1 as usize];
                        } else if i == 117 as libc::c_int {
                            let mut realNeighbors_0: [libc::c_int; 7] = [
                                117 as libc::c_int,
                                127 as libc::c_int,
                                109 as libc::c_int,
                                118 as libc::c_int,
                                113 as libc::c_int,
                                121 as libc::c_int,
                                106 as libc::c_int,
                            ];
                            neighborBc = realNeighbors_0[dir_1 as usize];
                        }
                        baseCellNeighbors[i as usize][dir_1 as usize] = neighborBc;
                        baseCellRotations[i as usize][dir_1 as usize] = rotations;
                    }
                    axis += 1;
                }
                f += 1;
            }
        }
        i += 1;
    }
    auditBaseCellNeighbors(
        baseCellNeighbors.as_mut_ptr(),
        baseCellRotations.as_mut_ptr(),
    );
    printf(
        b"const int baseCellNeighbors[NUM_BASE_CELLS][7] = {\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 122 as libc::c_int {
        printf(b"    {\0" as *const u8 as *const libc::c_char);
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 7 as libc::c_int {
            if j > 0 as libc::c_int {
                printf(b", \0" as *const u8 as *const libc::c_char);
            }
            if baseCellNeighbors[i_0 as usize][j as usize] != 127 as libc::c_int {
                printf(
                    b"%d\0" as *const u8 as *const libc::c_char,
                    baseCellNeighbors[i_0 as usize][j as usize],
                );
            } else {
                printf(b"INVALID_BASE_CELL\0" as *const u8 as *const libc::c_char);
            }
            j += 1;
        }
        printf(
            b"}, // base cell %d%s\n\0" as *const u8 as *const libc::c_char,
            i_0,
            if _isBaseCellPentagon(i_0) != 0 {
                b" (pentagon)\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        i_0 += 1;
    }
    printf(b"};\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"const int baseCellNeighbor60CCWRots[NUM_BASE_CELLS][7] = {\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 122 as libc::c_int {
        printf(
            b"    {%d, %d, %d, %d, %d, %d, %d}, // base cell %d%s\n\0" as *const u8
                as *const libc::c_char,
            baseCellRotations[i_1 as usize][0 as libc::c_int as usize],
            baseCellRotations[i_1 as usize][1 as libc::c_int as usize],
            baseCellRotations[i_1 as usize][2 as libc::c_int as usize],
            baseCellRotations[i_1 as usize][3 as libc::c_int as usize],
            baseCellRotations[i_1 as usize][4 as libc::c_int as usize],
            baseCellRotations[i_1 as usize][5 as libc::c_int as usize],
            baseCellRotations[i_1 as usize][6 as libc::c_int as usize],
            i_1,
            if _isBaseCellPentagon(i_1) != 0 {
                b" (pentagon)\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        i_1 += 1;
    }
    printf(b"};\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    if argc > 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"usage: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    generate();
    0
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ))
    }
}
