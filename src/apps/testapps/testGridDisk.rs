extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn gridDiskDistancesUnsafe(
        origin: H3Index,
        k: libc::c_int,
        out: *mut H3Index,
        distances: *mut libc::c_int,
    ) -> H3Error;
    fn gridDiskDistancesSafe(
        origin: H3Index,
        k: libc::c_int,
        out: *mut H3Index,
        distances: *mut libc::c_int,
    ) -> H3Error;
    fn gridDisk(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn gridDiskDistances(
        origin: H3Index,
        k: libc::c_int,
        out: *mut H3Index,
        distances: *mut libc::c_int,
    ) -> H3Error;
    fn getNumCells(res: libc::c_int, out: *mut int64_t) -> H3Error;
    fn h3NeighborRotations(
        origin: H3Index,
        dir: Direction,
        rotations: *mut libc::c_int,
        out: *mut H3Index,
    ) -> H3Error;
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    fn _baseCellToFaceIjk(baseCell: libc::c_int, h: *mut FaceIJK);
    fn _baseCellIsCwOffset(baseCell: libc::c_int, testFace: libc::c_int) -> bool;
    fn _getBaseCellNeighbor(baseCell: libc::c_int, dir: Direction) -> libc::c_int;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn iterateAllIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    );
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
unsafe extern "C" fn gridDisk_equals_gridDiskDistancesSafe_assertions(mut h3: H3Index) {
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < 3 as libc::c_int {
        let mut kSz: int64_t = 0;
        if maxGridDiskSize(k, &mut kSz) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                33 as libc::c_int,
                b"!(maxGridDiskSize(k, &kSz))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut neighbors: *mut H3Index = calloc(
            kSz as libc::c_ulong,
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
        ) as *mut H3Index;
        let mut distances: *mut libc::c_int = calloc(
            kSz as libc::c_ulong,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        if gridDiskDistances(h3, k, neighbors, distances) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                38 as libc::c_int,
                b"!(gridDiskDistances(h3, k, neighbors, distances))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut internalNeighbors: *mut H3Index = calloc(
            kSz as libc::c_ulong,
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
        ) as *mut H3Index;
        let mut internalDistances: *mut libc::c_int = calloc(
            kSz as libc::c_ulong,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        if gridDiskDistancesSafe(h3, k, internalNeighbors, internalDistances) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                43 as libc::c_int,
                b"!(gridDiskDistancesSafe( h3, k, internalNeighbors, internalDistances))\0"
                    as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut found: libc::c_int = 0 as libc::c_int;
        let mut internalFound: libc::c_int = 0 as libc::c_int;
        let mut iNeighbor: int64_t = 0 as libc::c_int as int64_t;
        while iNeighbor < kSz {
            if *neighbors.offset(iNeighbor as isize) != 0 as libc::c_int as libc::c_ulonglong {
                found += 1;
                let mut iInternal: int64_t = 0 as libc::c_int as int64_t;
                while iInternal < kSz {
                    if *internalNeighbors.offset(iInternal as isize)
                        == *neighbors.offset(iNeighbor as isize)
                    {
                        internalFound += 1;
                        if !(*distances.offset(iNeighbor as isize)
                            == *internalDistances.offset(iInternal as isize))
                        {
                            fprintf(
                                __stderrp,
                                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                    as *const libc::c_char,
                                currentSuiteName,
                                currentTestName,
                                b"src/apps/testapps/testGridDisk.c\0" as *const u8
                                    as *const libc::c_char,
                                58 as libc::c_int,
                                b"distances[iNeighbor] == internalDistances[iInternal]\0"
                                    as *const u8
                                    as *const libc::c_char,
                                b"External and internal agree on distance\0" as *const u8
                                    as *const libc::c_char,
                            );
                            exit(1 as libc::c_int);
                        }
                        globalTestCount += 1;
                        printf(b".\0" as *const u8 as *const libc::c_char);
                        break;
                    } else {
                        iInternal += 1;
                    }
                }
                if !(found == internalFound) {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                        66 as libc::c_int,
                        b"found == internalFound\0" as *const u8 as *const libc::c_char,
                        b"External and internal implementations produce same output\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
            }
            iNeighbor += 1;
        }
        free(internalDistances as *mut libc::c_void);
        free(internalNeighbors as *mut libc::c_void);
        free(distances as *mut libc::c_void);
        free(neighbors as *mut libc::c_void);
        k += 1;
    }
}
unsafe extern "C" fn runTests() {
    currentTestName = b"gridDisk0\0" as *const u8 as *const libc::c_char;
    let mut sf: LatLng = {
        let mut init = LatLng {
            lat: 0.659966917655f64,
            lng: 2 as libc::c_int as libc::c_double * 3.14159f64 - 2.1364398519396f64,
        };
        init
    };
    let mut sfHex0: H3Index = 0;
    if latLngToCell(&mut sf, 0 as libc::c_int, &mut sfHex0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            b"!(latLngToCell(&sf, 0, &sfHex0))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut k1: [H3Index; 7] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut k1Dist: [libc::c_int; 7] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut expectedK1: [H3Index; 7] = [
        0x8029fffffffffff as libc::c_long as H3Index,
        0x801dfffffffffff as libc::c_long as H3Index,
        0x8013fffffffffff as libc::c_long as H3Index,
        0x8027fffffffffff as libc::c_long as H3Index,
        0x8049fffffffffff as libc::c_long as H3Index,
        0x8051fffffffffff as libc::c_long as H3Index,
        0x8037fffffffffff as libc::c_long as H3Index,
    ];
    if gridDiskDistances(
        sfHex0,
        1 as libc::c_int,
        k1.as_mut_ptr(),
        k1Dist.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            b"!(gridDiskDistances(sfHex0, 1, k1, k1Dist))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        if !(k1[i as usize] != 0 as libc::c_int as libc::c_ulonglong) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int,
                b"k1[i] != 0\0" as *const u8 as *const libc::c_char,
                b"index is populated\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut inList: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 7 as libc::c_int {
            if k1[i as usize] == expectedK1[j as usize] {
                if !(k1Dist[i as usize]
                    == (if k1[i as usize] == sfHex0 {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }))
                {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                        98 as libc::c_int,
                        b"k1Dist[i] == (k1[i] == sfHex0 ? 0 : 1)\0" as *const u8
                            as *const libc::c_char,
                        b"distance is as expected\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                inList += 1;
            }
            j += 1;
        }
        if !(inList == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                b"inList == 1\0" as *const u8 as *const libc::c_char,
                b"index found in expected set\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    currentTestName = b"gridDisk0_PolarPentagon\0" as *const u8 as *const libc::c_char;
    let mut polar: H3Index = 0;
    setH3Index(&mut polar, 0 as libc::c_int, 4 as libc::c_int, CENTER_DIGIT);
    let mut k2: [H3Index; 7] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut k2Dist: [libc::c_int; 7] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut expectedK2: [H3Index; 7] = [
        0x8009fffffffffff as libc::c_long as H3Index,
        0x8007fffffffffff as libc::c_long as H3Index,
        0x8001fffffffffff as libc::c_long as H3Index,
        0x8011fffffffffff as libc::c_long as H3Index,
        0x801ffffffffffff as libc::c_long as H3Index,
        0x8019fffffffffff as libc::c_long as H3Index,
        0 as libc::c_int as H3Index,
    ];
    if gridDiskDistances(
        polar,
        1 as libc::c_int,
        k2.as_mut_ptr(),
        k2Dist.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            b"!(gridDiskDistances(polar, 1, k2, k2Dist))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut k2present: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 7 as libc::c_int {
        if k2[i_0 as usize] != 0 as libc::c_int as libc::c_ulonglong {
            k2present += 1;
            let mut inList_0: libc::c_int = 0 as libc::c_int;
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < 7 as libc::c_int {
                if k2[i_0 as usize] == expectedK2[j_0 as usize] {
                    if !(k2Dist[i_0 as usize]
                        == (if k2[i_0 as usize] == polar {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }))
                    {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testGridDisk.c\0" as *const u8
                                as *const libc::c_char,
                            128 as libc::c_int,
                            b"k2Dist[i] == (k2[i] == polar ? 0 : 1)\0" as *const u8
                                as *const libc::c_char,
                            b"distance is as expected\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    inList_0 += 1;
                }
                j_0 += 1;
            }
            if !(inList_0 == 1 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                    132 as libc::c_int,
                    b"inList == 1\0" as *const u8 as *const libc::c_char,
                    b"index found in expected set\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_0 += 1;
    }
    if !(k2present == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            b"k2present == 6\0" as *const u8 as *const libc::c_char,
            b"pentagon has 5 neighbors\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"gridDisk1_PolarPentagon\0" as *const u8 as *const libc::c_char;
    let mut polar_0: H3Index = 0;
    setH3Index(
        &mut polar_0,
        1 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut k2_0: [H3Index; 7] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut k2Dist_0: [libc::c_int; 7] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut expectedK2_0: [H3Index; 7] = [
        0x81083ffffffffff as libc::c_long as H3Index,
        0x81093ffffffffff as libc::c_long as H3Index,
        0x81097ffffffffff as libc::c_long as H3Index,
        0x8108fffffffffff as libc::c_long as H3Index,
        0x8108bffffffffff as libc::c_long as H3Index,
        0x8109bffffffffff as libc::c_long as H3Index,
        0 as libc::c_int as H3Index,
    ];
    if gridDiskDistances(
        polar_0,
        1 as libc::c_int,
        k2_0.as_mut_ptr(),
        k2Dist_0.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            b"!(gridDiskDistances(polar, 1, k2, k2Dist))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut k2present_0: libc::c_int = 0 as libc::c_int;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 7 as libc::c_int {
        if k2_0[i_1 as usize] != 0 as libc::c_int as libc::c_ulonglong {
            k2present_0 += 1;
            let mut inList_1: libc::c_int = 0 as libc::c_int;
            let mut j_1: libc::c_int = 0 as libc::c_int;
            while j_1 < 7 as libc::c_int {
                if k2_0[i_1 as usize] == expectedK2_0[j_1 as usize] {
                    if !(k2Dist_0[i_1 as usize]
                        == (if k2_0[i_1 as usize] == polar_0 {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }))
                    {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testGridDisk.c\0" as *const u8
                                as *const libc::c_char,
                            160 as libc::c_int,
                            b"k2Dist[i] == (k2[i] == polar ? 0 : 1)\0" as *const u8
                                as *const libc::c_char,
                            b"distance is as expected\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    inList_1 += 1;
                }
                j_1 += 1;
            }
            if !(inList_1 == 1 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                    164 as libc::c_int,
                    b"inList == 1\0" as *const u8 as *const libc::c_char,
                    b"index found in expected set\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_1 += 1;
    }
    if !(k2present_0 == 6 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int,
            b"k2present == 6\0" as *const u8 as *const libc::c_char,
            b"pentagon has 5 neighbors\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"gridDisk1_PolarPentagon_k3\0" as *const u8 as *const libc::c_char;
    let mut polar_1: H3Index = 0;
    setH3Index(
        &mut polar_1,
        1 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut k2_1: [H3Index; 37] = [
        0 as libc::c_int as H3Index,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut k2Dist_1: [libc::c_int; 37] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut expectedK2_1: [H3Index; 37] = [
        0x81013ffffffffff as libc::c_long as H3Index,
        0x811fbffffffffff as libc::c_long as H3Index,
        0x81193ffffffffff as libc::c_long as H3Index,
        0x81097ffffffffff as libc::c_long as H3Index,
        0x81003ffffffffff as libc::c_long as H3Index,
        0x81183ffffffffff as libc::c_long as H3Index,
        0x8111bffffffffff as libc::c_long as H3Index,
        0x81077ffffffffff as libc::c_long as H3Index,
        0x811f7ffffffffff as libc::c_long as H3Index,
        0x81067ffffffffff as libc::c_long as H3Index,
        0x81093ffffffffff as libc::c_long as H3Index,
        0x811e7ffffffffff as libc::c_long as H3Index,
        0x81083ffffffffff as libc::c_long as H3Index,
        0x81117ffffffffff as libc::c_long as H3Index,
        0x8101bffffffffff as libc::c_long as H3Index,
        0x81107ffffffffff as libc::c_long as H3Index,
        0x81073ffffffffff as libc::c_long as H3Index,
        0x811f3ffffffffff as libc::c_long as H3Index,
        0x81063ffffffffff as libc::c_long as H3Index,
        0x8108fffffffffff as libc::c_long as H3Index,
        0x811e3ffffffffff as libc::c_long as H3Index,
        0x8119bffffffffff as libc::c_long as H3Index,
        0x81113ffffffffff as libc::c_long as H3Index,
        0x81017ffffffffff as libc::c_long as H3Index,
        0x81103ffffffffff as libc::c_long as H3Index,
        0x8109bffffffffff as libc::c_long as H3Index,
        0x81197ffffffffff as libc::c_long as H3Index,
        0x81007ffffffffff as libc::c_long as H3Index,
        0x8108bffffffffff as libc::c_long as H3Index,
        0x81187ffffffffff as libc::c_long as H3Index,
        0x8107bffffffffff as libc::c_long as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut expectedK2Dist: [libc::c_int; 37] = [
        2 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ];
    if gridDiskDistances(
        polar_1,
        3 as libc::c_int,
        k2_1.as_mut_ptr(),
        k2Dist_1.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int,
            b"!(gridDiskDistances(polar, 3, k2, k2Dist))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut k2present_1: libc::c_int = 0 as libc::c_int;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < 37 as libc::c_int {
        if k2_1[i_2 as usize] != 0 as libc::c_int as libc::c_ulonglong {
            k2present_1 += 1;
            let mut inList_2: libc::c_int = 0 as libc::c_int;
            let mut j_2: libc::c_int = 0 as libc::c_int;
            while j_2 < 37 as libc::c_int {
                if k2_1[i_2 as usize] == expectedK2_1[j_2 as usize] {
                    if !(k2Dist_1[i_2 as usize] == expectedK2Dist[j_2 as usize]) {
                        fprintf(
                            __stderrp,
                            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                as *const libc::c_char,
                            currentSuiteName,
                            currentTestName,
                            b"src/apps/testapps/testGridDisk.c\0" as *const u8
                                as *const libc::c_char,
                            225 as libc::c_int,
                            b"k2Dist[i] == expectedK2Dist[j]\0" as *const u8 as *const libc::c_char,
                            b"distance is as expected\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    globalTestCount += 1;
                    printf(b".\0" as *const u8 as *const libc::c_char);
                    inList_2 += 1;
                }
                j_2 += 1;
            }
            if !(inList_2 == 1 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                    229 as libc::c_int,
                    b"inList == 1\0" as *const u8 as *const libc::c_char,
                    b"index found in expected set\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_2 += 1;
    }
    if !(k2present_1 == 31 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            b"k2present == 31\0" as *const u8 as *const libc::c_char,
            b"pentagon has 30 neighbors\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"gridDisk1_Pentagon_k4\0" as *const u8 as *const libc::c_char;
    let mut pent: H3Index = 0;
    setH3Index(&mut pent, 1 as libc::c_int, 14 as libc::c_int, CENTER_DIGIT);
    let mut k2_2: [H3Index; 61] = [
        0 as libc::c_int as H3Index,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut k2Dist_2: [libc::c_int; 61] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut expectedK2_2: [H3Index; 61] = [
        0x811d7ffffffffff as libc::c_long as H3Index,
        0x810c7ffffffffff as libc::c_long as H3Index,
        0x81227ffffffffff as libc::c_long as H3Index,
        0x81293ffffffffff as libc::c_long as H3Index,
        0x81133ffffffffff as libc::c_long as H3Index,
        0x8136bffffffffff as libc::c_long as H3Index,
        0x81167ffffffffff as libc::c_long as H3Index,
        0x811d3ffffffffff as libc::c_long as H3Index,
        0x810c3ffffffffff as libc::c_long as H3Index,
        0x81223ffffffffff as libc::c_long as H3Index,
        0x81477ffffffffff as libc::c_long as H3Index,
        0x8128fffffffffff as libc::c_long as H3Index,
        0x81367ffffffffff as libc::c_long as H3Index,
        0x8112fffffffffff as libc::c_long as H3Index,
        0x811cfffffffffff as libc::c_long as H3Index,
        0x8123bffffffffff as libc::c_long as H3Index,
        0x810dbffffffffff as libc::c_long as H3Index,
        0x8112bffffffffff as libc::c_long as H3Index,
        0x81473ffffffffff as libc::c_long as H3Index,
        0x8128bffffffffff as libc::c_long as H3Index,
        0x81363ffffffffff as libc::c_long as H3Index,
        0x811cbffffffffff as libc::c_long as H3Index,
        0x81237ffffffffff as libc::c_long as H3Index,
        0x810d7ffffffffff as libc::c_long as H3Index,
        0x81127ffffffffff as libc::c_long as H3Index,
        0x8137bffffffffff as libc::c_long as H3Index,
        0x81287ffffffffff as libc::c_long as H3Index,
        0x8126bffffffffff as libc::c_long as H3Index,
        0x81177ffffffffff as libc::c_long as H3Index,
        0x810d3ffffffffff as libc::c_long as H3Index,
        0x81233ffffffffff as libc::c_long as H3Index,
        0x8150fffffffffff as libc::c_long as H3Index,
        0x81123ffffffffff as libc::c_long as H3Index,
        0x81377ffffffffff as libc::c_long as H3Index,
        0x81283ffffffffff as libc::c_long as H3Index,
        0x8102fffffffffff as libc::c_long as H3Index,
        0x811c3ffffffffff as libc::c_long as H3Index,
        0x810cfffffffffff as libc::c_long as H3Index,
        0x8122fffffffffff as libc::c_long as H3Index,
        0x8113bffffffffff as libc::c_long as H3Index,
        0x81373ffffffffff as libc::c_long as H3Index,
        0x8129bffffffffff as libc::c_long as H3Index,
        0x8102bffffffffff as libc::c_long as H3Index,
        0x811dbffffffffff as libc::c_long as H3Index,
        0x810cbffffffffff as libc::c_long as H3Index,
        0x8122bffffffffff as libc::c_long as H3Index,
        0x81297ffffffffff as libc::c_long as H3Index,
        0x81507ffffffffff as libc::c_long as H3Index,
        0x8136fffffffffff as libc::c_long as H3Index,
        0x8127bffffffffff as libc::c_long as H3Index,
        0x81137ffffffffff as libc::c_long as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if gridDiskDistances(
        pent,
        4 as libc::c_int,
        k2_2.as_mut_ptr(),
        k2Dist_2.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int,
            b"!(gridDiskDistances(pent, 4, k2, k2Dist))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut k2present_2: libc::c_int = 0 as libc::c_int;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < 61 as libc::c_int {
        if k2_2[i_3 as usize] != 0 as libc::c_int as libc::c_ulonglong {
            k2present_2 += 1;
            let mut inList_3: libc::c_int = 0 as libc::c_int;
            let mut j_3: libc::c_int = 0 as libc::c_int;
            while j_3 < 61 as libc::c_int {
                if k2_2[i_3 as usize] == expectedK2_2[j_3 as usize] {
                    inList_3 += 1;
                }
                j_3 += 1;
            }
            if !(inList_3 == 1 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                    305 as libc::c_int,
                    b"inList == 1\0" as *const u8 as *const libc::c_char,
                    b"index found in expected set\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_3 += 1;
    }
    if !(k2present_2 == 51 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int,
            b"k2present == 51\0" as *const u8 as *const libc::c_char,
            b"pentagon has 50 neighbors\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName =
        b"gridDisk_equals_gridDiskDistancesSafe\0" as *const u8 as *const libc::c_char;
    let mut res: libc::c_int = 0 as libc::c_int;
    while res < 2 as libc::c_int {
        iterateAllIndexesAtRes(
            res,
            Some(
                gridDisk_equals_gridDiskDistancesSafe_assertions
                    as unsafe extern "C" fn(H3Index) -> (),
            ),
        );
        res += 1;
    }
    currentTestName = b"h3NeighborRotations_identity\0" as *const u8 as *const libc::c_char;
    let mut origin: H3Index = 0x811d7ffffffffff as libc::c_long as H3Index;
    let mut rotations: libc::c_int = 0 as libc::c_int;
    let mut out: H3Index = 0;
    if h3NeighborRotations(origin, CENTER_DIGIT, &mut rotations, &mut out) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            328 as libc::c_int,
            b"!(h3NeighborRotations(origin, CENTER_DIGIT, &rotations, &out))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(out == origin) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int,
            b"out == origin\0" as *const u8 as *const libc::c_char,
            b"Moving to self goes to self\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(rotations == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            b"rotations == 0\0" as *const u8 as *const libc::c_char,
            b"Expected rotations\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName =
        b"h3NeighborRotations_rotationsOverflow\0" as *const u8 as *const libc::c_char;
    let mut origin_0: H3Index = 0;
    setH3Index(
        &mut origin_0,
        0 as libc::c_int,
        0 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut rotations_0: libc::c_int = 2147483646 as libc::c_int;
    let mut out_0: H3Index = 0;
    if h3NeighborRotations(origin_0, K_AXES_DIGIT, &mut rotations_0, &mut out_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            342 as libc::c_int,
            b"!(h3NeighborRotations(origin, K_AXES_DIGIT, &rotations, &out))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut expected: H3Index = 0;
    setH3Index(
        &mut expected,
        0 as libc::c_int,
        1 as libc::c_int,
        CENTER_DIGIT,
    );
    if !(out_0 == expected) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            b"out == expected\0" as *const u8 as *const libc::c_char,
            b"Expected neighbor\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(rotations_0 == 5 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            b"rotations == 5\0" as *const u8 as *const libc::c_char,
            b"Expected rotations value\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName =
        b"h3NeighborRotations_rotationsOverflow2\0" as *const u8 as *const libc::c_char;
    let mut origin_1: H3Index = 0;
    setH3Index(
        &mut origin_1,
        0 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut rotations_1: libc::c_int = 2147483647 as libc::c_int;
    let mut out_1: H3Index = 0;
    if h3NeighborRotations(origin_1, JK_AXES_DIGIT, &mut rotations_1, &mut out_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            b"!(h3NeighborRotations(origin, JK_AXES_DIGIT, &rotations, &out))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut expected_0: H3Index = 0;
    setH3Index(
        &mut expected_0,
        0 as libc::c_int,
        0 as libc::c_int,
        CENTER_DIGIT,
    );
    if !(out_1 == expected_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            364 as libc::c_int,
            b"out == expected\0" as *const u8 as *const libc::c_char,
            b"Expected neighbor\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(rotations_1 == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            367 as libc::c_int,
            b"rotations == 0\0" as *const u8 as *const libc::c_char,
            b"Expected rotations value\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"h3NeighborRotations_invalid\0" as *const u8 as *const libc::c_char;
    let mut origin_2: H3Index = 0x811d7ffffffffff as libc::c_long as H3Index;
    let mut rotations_2: libc::c_int = 0 as libc::c_int;
    let mut out_2: H3Index = 0;
    if !(h3NeighborRotations(
        origin_2,
        4294967295 as Direction,
        &mut rotations_2,
        &mut out_2,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            375 as libc::c_int,
            b"h3NeighborRotations(origin, -1, &rotations, &out) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"Invalid direction fails (-1)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(h3NeighborRotations(origin_2, INVALID_DIGIT, &mut rotations_2, &mut out_2)
        == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            377 as libc::c_int,
            b"h3NeighborRotations(origin, 7, &rotations, &out) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"Invalid direction fails (7)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(h3NeighborRotations(origin_2, 100 as Direction, &mut rotations_2, &mut out_2)
        == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            379 as libc::c_int,
            b"h3NeighborRotations(origin, 100, &rotations, &out) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"Invalid direction fails (100)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cwOffsetPent\0" as *const u8 as *const libc::c_char;
    let mut pentagon: libc::c_int = 0 as libc::c_int;
    while pentagon < 122 as libc::c_int {
        if !(_isBaseCellPentagon(pentagon) == 0) {
            let mut neighbor: libc::c_int = 0 as libc::c_int;
            while neighbor < 122 as libc::c_int {
                let mut homeFaceIjk: FaceIJK = FaceIJK {
                    face: 0,
                    coord: CoordIJK { i: 0, j: 0, k: 0 },
                };
                _baseCellToFaceIjk(neighbor, &mut homeFaceIjk);
                let mut neighborFace: libc::c_int = homeFaceIjk.face;
                if !(_getBaseCellNeighbor(neighbor, J_AXES_DIGIT) != pentagon
                    || _baseCellIsCwOffset(pentagon, neighborFace) as libc::c_int != 0)
                {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testGridDisk.c\0"
                            as *const u8 as *const libc::c_char,
                        407 as libc::c_int,
                        b"_getBaseCellNeighbor(neighbor, J_AXES_DIGIT) != pentagon || _baseCellIsCwOffset(pentagon, neighborFace)\0"
                            as *const u8 as *const libc::c_char,
                        b"cwOffsetPent is reachable\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                neighbor += 1;
            }
        }
        pentagon += 1;
    }
    currentTestName = b"gridDiskInvalid\0" as *const u8 as *const libc::c_char;
    let mut k: libc::c_int = 1000 as libc::c_int;
    let mut kSz: int64_t = 0;
    if maxGridDiskSize(k, &mut kSz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            415 as libc::c_int,
            b"!(maxGridDiskSize(k, &kSz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut neighbors: *mut H3Index = calloc(
        kSz as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if !(gridDisk(0x7fffffffffffffff as libc::c_long as H3Index, k, neighbors)
        == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            419 as libc::c_int,
            b"H3_EXPORT(gridDisk)(0x7fffffffffffffff, k, neighbors) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"gridDisk returns error for invalid input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(neighbors as *mut libc::c_void);
    currentTestName = b"gridDiskInvalidDigit\0" as *const u8 as *const libc::c_char;
    let mut k_0: libc::c_int = 2 as libc::c_int;
    let mut kSz_0: int64_t = 0;
    if maxGridDiskSize(k_0, &mut kSz_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            426 as libc::c_int,
            b"!(maxGridDiskSize(k, &kSz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut neighbors_0: *mut H3Index = calloc(
        kSz_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if !(gridDisk(
        0x4d4b00fe5c5c3030 as libc::c_long as H3Index,
        k_0,
        neighbors_0,
    ) == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            b"H3_EXPORT(gridDisk)(0x4d4b00fe5c5c3030, k, neighbors) == E_CELL_INVALID\0"
                as *const u8 as *const libc::c_char,
            b"gridDisk returns error for invalid input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(neighbors_0 as *mut libc::c_void);
    currentTestName = b"gridDiskDistances_invalidK\0" as *const u8 as *const libc::c_char;
    let mut index: H3Index = 0x811d7ffffffffff as libc::c_long as H3Index;
    if !(gridDiskDistances(
        index,
        -(1 as libc::c_int),
        0 as *mut H3Index,
        0 as *mut libc::c_int,
    ) == E_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
            b"H3_EXPORT(gridDiskDistances)(index, -1, NULL, NULL) == E_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"gridDiskDistances invalid k\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(gridDiskDistancesUnsafe(
        index,
        -(1 as libc::c_int),
        0 as *mut H3Index,
        0 as *mut libc::c_int,
    ) == E_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            441 as libc::c_int,
            b"H3_EXPORT(gridDiskDistancesUnsafe)(index, -1, NULL, NULL) == E_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"gridDiskDistancesUnsafe invalid k\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(gridDiskDistancesSafe(
        index,
        -(1 as libc::c_int),
        0 as *mut H3Index,
        0 as *mut libc::c_int,
    ) == E_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            444 as libc::c_int,
            b"H3_EXPORT(gridDiskDistancesSafe)(index, -1, NULL, NULL) == E_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"gridDiskDistancesSafe invalid k\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"maxGridDiskSize_invalid\0" as *const u8 as *const libc::c_char;
    let mut sz: int64_t = 0;
    if !(maxGridDiskSize(-(1 as libc::c_int), &mut sz) == E_DOMAIN as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            450 as libc::c_int,
            b"H3_EXPORT(maxGridDiskSize)(-1, &sz) == E_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"negative k is invalid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"maxGridDiskSize_large\0" as *const u8 as *const libc::c_char;
    let mut sz_0: int64_t = 0;
    if maxGridDiskSize(26755 as libc::c_int, &mut sz_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            455 as libc::c_int,
            b"!(maxGridDiskSize(26755, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(sz_0 == 2147570341 as libc::c_long as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            456 as libc::c_int,
            b"sz == 2147570341\0" as *const u8 as *const libc::c_char,
            b"large (> 32 bit signed int) k works\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"maxGridDiskSize_numCells\0" as *const u8 as *const libc::c_char;
    let mut sz_1: int64_t = 0;
    let mut prev: int64_t = 0 as libc::c_int as int64_t;
    let mut max: int64_t = 0;
    if getNumCells(15 as libc::c_int, &mut max) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            463 as libc::c_int,
            b"!(getNumCells(15, &max))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut k_1: libc::c_int = 13780510 as libc::c_int - 100 as libc::c_int;
    while k_1 < 13780510 as libc::c_int + 100 as libc::c_int {
        if maxGridDiskSize(k_1, &mut sz_1) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                466 as libc::c_int,
                b"!(maxGridDiskSize(k, &sz))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(sz_1 <= max) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                469 as libc::c_int,
                b"sz <= max\0" as *const u8 as *const libc::c_char,
                b"maxGridDiskSize does not produce estimates above the number of grid cells\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(prev <= sz_1) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
                470 as libc::c_int,
                b"prev <= sz\0" as *const u8 as *const libc::c_char,
                b"maxGridDiskSize is monotonically increasing\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        prev = sz_1;
        k_1 += 1;
    }
    if maxGridDiskSize(2147483647 as libc::c_int, &mut sz_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            474 as libc::c_int,
            b"!(maxGridDiskSize(2147483647, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(sz_1 == max) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDisk.c\0" as *const u8 as *const libc::c_char,
            476 as libc::c_int,
            b"sz == max\0" as *const u8 as *const libc::c_char,
            b"maxGridDiskSize of INT32_MAX produces valid result\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"gridDisk\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"gridDisk\0" as *const u8 as *const libc::c_char,
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
