use ::libc;
extern "C" {
    pub type __sFILEX;
    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn gridDiskDistancesSafe(
        origin: H3Index,
        k: libc::c_int,
        out: *mut H3Index,
        distances: *mut libc::c_int,
    ) -> H3Error;
    fn gridRingUnsafe(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn uncompactCellsSize(
        compactedSet: *const H3Index,
        numCompacted: int64_t,
        res: libc::c_int,
        out: *mut int64_t,
    ) -> H3Error;
    fn uncompactCells(
        compactedSet: *const H3Index,
        numCompacted: int64_t,
        outSet: *mut H3Index,
        numOut: int64_t,
        res: libc::c_int,
    ) -> H3Error;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
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
    pub _extra: *mut __sFILEX,
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
unsafe extern "C" fn runTests() {
    let mut sf: LatLng = {
        let mut init = LatLng {
            lat: 0.659966917655f64,
            lng: 2 as libc::c_int as libc::c_double * 3.14159f64 - 2.1364398519396f64,
        };
        init
    };
    let mut sfHex: H3Index = 0;
    if latLngToCell(&mut sf, 9 as libc::c_int, &mut sfHex) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                as *const libc::c_char,
            27 as libc::c_int,
            b"!(latLngToCell(&sf, 9, &sfHex))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"identityGridRing\0" as *const u8 as *const libc::c_char;
    let mut k0: [H3Index; 1] = [0 as libc::c_int as H3Index];
    if gridRingUnsafe(sfHex, 0 as libc::c_int, k0.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                as *const libc::c_char,
            31 as libc::c_int,
            b"!(gridRingUnsafe(sfHex, 0, k0))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(k0[0 as libc::c_int as usize] == sfHex) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                as *const libc::c_char,
            32 as libc::c_int,
            b"k0[0] == sfHex\0" as *const u8 as *const libc::c_char,
            b"generated identity k-ring\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"ring1\0" as *const u8 as *const libc::c_char;
    let mut k1: [H3Index; 6] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut expectedK1: [H3Index; 6] = [
        0x89283080ddbffff as libc::c_long as H3Index,
        0x89283080c37ffff as libc::c_long as H3Index,
        0x89283080c27ffff as libc::c_long as H3Index,
        0x89283080d53ffff as libc::c_long as H3Index,
        0x89283080dcfffff as libc::c_long as H3Index,
        0x89283080dc3ffff as libc::c_long as H3Index,
    ];
    if gridRingUnsafe(sfHex, 1 as libc::c_int, k1.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                as *const libc::c_char,
            40 as libc::c_int,
            b"!(gridRingUnsafe(sfHex, 1, k1))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if !(k1[i as usize] != 0 as libc::c_int as libc::c_ulonglong) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                    as *const libc::c_char,
                43 as libc::c_int,
                b"k1[i] != 0\0" as *const u8 as *const libc::c_char,
                b"index is populated\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut inList: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            if k1[i as usize] == expectedK1[j as usize] {
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
                b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                    as *const libc::c_char,
                50 as libc::c_int,
                b"inList == 1\0" as *const u8 as *const libc::c_char,
                b"index found in expected set\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    currentTestName = b"ring2\0" as *const u8 as *const libc::c_char;
    let mut k2: [H3Index; 12] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut expectedK2: [H3Index; 12] = [
        0x89283080ca7ffff as libc::c_long as H3Index,
        0x89283080cafffff as libc::c_long as H3Index,
        0x89283080c33ffff as libc::c_long as H3Index,
        0x89283080c23ffff as libc::c_long as H3Index,
        0x89283080c2fffff as libc::c_long as H3Index,
        0x89283080d5bffff as libc::c_long as H3Index,
        0x89283080d43ffff as libc::c_long as H3Index,
        0x89283080d57ffff as libc::c_long as H3Index,
        0x89283080d1bffff as libc::c_long as H3Index,
        0x89283080dc7ffff as libc::c_long as H3Index,
        0x89283080dd7ffff as libc::c_long as H3Index,
        0x89283080dd3ffff as libc::c_long as H3Index,
    ];
    if gridRingUnsafe(sfHex, 2 as libc::c_int, k2.as_mut_ptr()) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                as *const libc::c_char,
            61 as libc::c_int,
            b"!(gridRingUnsafe(sfHex, 2, k2))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 12 as libc::c_int {
        if !(k2[i_0 as usize] != 0 as libc::c_int as libc::c_ulonglong) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                    as *const libc::c_char,
                64 as libc::c_int,
                b"k2[i] != 0\0" as *const u8 as *const libc::c_char,
                b"index is populated\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut inList_0: libc::c_int = 0 as libc::c_int;
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < 12 as libc::c_int {
            if k2[i_0 as usize] == expectedK2[j_0 as usize] {
                inList_0 += 1;
            }
            j_0 += 1;
        }
        if !(inList_0 == 1 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                    as *const libc::c_char,
                71 as libc::c_int,
                b"inList == 1\0" as *const u8 as *const libc::c_char,
                b"index found in expected set\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_0 += 1;
    }
    currentTestName = b"nearPentagonRing1\0" as *const u8 as *const libc::c_char;
    let mut nearPentagon: H3Index = 0x837405fffffffff as libc::c_long as H3Index;
    let mut kp1: [H3Index; 6] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    if !(gridRingUnsafe(nearPentagon, 1 as libc::c_int, kp1.as_mut_ptr())
        == E_PENTAGON as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                as *const libc::c_char,
            79 as libc::c_int,
            b"H3_EXPORT(gridRingUnsafe)(nearPentagon, 1, kp1) == E_PENTAGON\0" as *const u8
                as *const libc::c_char,
            b"Should return an error when hitting a pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"nearPentagonRing2\0" as *const u8 as *const libc::c_char;
    let mut nearPentagon_0: H3Index = 0x837405fffffffff as libc::c_long as H3Index;
    let mut kp2: [H3Index; 12] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    if !(gridRingUnsafe(nearPentagon_0, 2 as libc::c_int, kp2.as_mut_ptr())
        == E_PENTAGON as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                as *const libc::c_char,
            86 as libc::c_int,
            b"H3_EXPORT(gridRingUnsafe)(nearPentagon, 2, kp2) == E_PENTAGON\0" as *const u8
                as *const libc::c_char,
            b"Should return an error when hitting a pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"onPentagon\0" as *const u8 as *const libc::c_char;
    let mut nearPentagon_1: H3Index = 0;
    setH3Index(
        &mut nearPentagon_1,
        0 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut kp2_0: [H3Index; 12] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    if !(gridRingUnsafe(nearPentagon_1, 2 as libc::c_int, kp2_0.as_mut_ptr())
        == E_PENTAGON as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                as *const libc::c_char,
            94 as libc::c_int,
            b"H3_EXPORT(gridRingUnsafe)(nearPentagon, 2, kp2) == E_PENTAGON\0" as *const u8
                as *const libc::c_char,
            b"Should return an error when starting at a pentagon\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName =
        b"gridRingUnsafe_matches_gridDiskDistancesSafe\0" as *const u8 as *const libc::c_char;
    let mut res: libc::c_int = 0 as libc::c_int;
    while res < 2 as libc::c_int {
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < 122 as libc::c_int {
            let mut bc: H3Index = 0;
            setH3Index(&mut bc, 0 as libc::c_int, i_1, CENTER_DIGIT);
            let mut childrenSz: int64_t = 0;
            if uncompactCellsSize(&mut bc, 1 as libc::c_int as int64_t, res, &mut childrenSz) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                        as *const libc::c_char,
                    104 as libc::c_int,
                    b"!(uncompactCellsSize(&bc, 1, res, &childrenSz))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut children: *mut H3Index = calloc(
                childrenSz as libc::c_ulong,
                ::core::mem::size_of::<H3Index>() as libc::c_ulong,
            ) as *mut H3Index;
            if uncompactCells(
                &mut bc,
                1 as libc::c_int as int64_t,
                children,
                childrenSz,
                res,
            ) != 0
            {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0" as *const u8
                        as *const libc::c_char,
                    107 as libc::c_int,
                    b"!(uncompactCells(&bc, 1, children, childrenSz, res))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut j_1: libc::c_int = 0 as libc::c_int;
            while (j_1 as libc::c_longlong) < childrenSz {
                if !(*children.offset(j_1 as isize) == 0 as libc::c_int as libc::c_ulonglong) {
                    let mut k: libc::c_int = 0 as libc::c_int;
                    while k < 3 as libc::c_int {
                        let mut ringSz: libc::c_int = if k != 0 as libc::c_int {
                            6 as libc::c_int * k
                        } else {
                            1 as libc::c_int
                        };
                        let mut kSz: int64_t = 0;
                        if maxGridDiskSize(k, &mut kSz) != 0 {
                            fprintf(
                                __stderrp,
                                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                    as *const libc::c_char,
                                currentSuiteName,
                                currentTestName,
                                b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0"
                                    as *const u8
                                    as *const libc::c_char,
                                117 as libc::c_int,
                                b"!(maxGridDiskSize(k, &kSz))\0" as *const u8
                                    as *const libc::c_char,
                                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                            );
                            exit(1 as libc::c_int);
                        }
                        globalTestCount += 1;
                        printf(b".\0" as *const u8 as *const libc::c_char);
                        let mut ring: *mut H3Index = calloc(
                            ringSz as libc::c_ulong,
                            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
                        ) as *mut H3Index;
                        let mut failed: H3Error =
                            gridRingUnsafe(*children.offset(j_1 as isize), k, ring);
                        if failed == 0 {
                            let mut internalNeighbors: *mut H3Index = calloc(
                                kSz as libc::c_ulong,
                                ::core::mem::size_of::<H3Index>() as libc::c_ulong,
                            )
                                as *mut H3Index;
                            let mut internalDistances: *mut libc::c_int = calloc(
                                kSz as libc::c_ulong,
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            )
                                as *mut libc::c_int;
                            if gridDiskDistancesSafe(
                                *children.offset(j_1 as isize),
                                k,
                                internalNeighbors,
                                internalDistances,
                            ) != 0
                            {
                                fprintf(
                                    __stderrp,
                                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    currentSuiteName,
                                    currentTestName,
                                    b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0"
                                        as *const u8 as *const libc::c_char,
                                    129 as libc::c_int,
                                    b"!(gridDiskDistancesSafe( children[j], k, internalNeighbors, internalDistances))\0"
                                        as *const u8 as *const libc::c_char,
                                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                                );
                                exit(1 as libc::c_int);
                            }
                            globalTestCount += 1;
                            printf(b".\0" as *const u8 as *const libc::c_char);
                            let mut found: libc::c_int = 0 as libc::c_int;
                            let mut internalFound: libc::c_int = 0 as libc::c_int;
                            let mut iRing: libc::c_int = 0 as libc::c_int;
                            while iRing < ringSz {
                                if *ring.offset(iRing as isize)
                                    != 0 as libc::c_int as libc::c_ulonglong
                                {
                                    found += 1;
                                    let mut iInternal: int64_t = 0 as libc::c_int as int64_t;
                                    while iInternal < kSz {
                                        if *internalNeighbors.offset(iInternal as isize)
                                            == *ring.offset(iRing as isize)
                                        {
                                            internalFound += 1;
                                            if !(*internalDistances.offset(iInternal as isize) == k)
                                            {
                                                fprintf(
                                                    __stderrp,
                                                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    currentSuiteName,
                                                    currentTestName,
                                                    b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0"
                                                        as *const u8 as *const libc::c_char,
                                                    147 as libc::c_int,
                                                    b"internalDistances[iInternal] == k\0" as *const u8
                                                        as *const libc::c_char,
                                                    b"Ring and internal agree on distance\0" as *const u8
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
                                            b"/Users/legnitto/src/h3/src/apps/testapps/testGridRingUnsafe.c\0"
                                                as *const u8 as *const libc::c_char,
                                            156 as libc::c_int,
                                            b"found == internalFound\0" as *const u8
                                                as *const libc::c_char,
                                            b"Ring and internal implementations produce same output\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        exit(1 as libc::c_int);
                                    }
                                    globalTestCount += 1;
                                    printf(b".\0" as *const u8 as *const libc::c_char);
                                }
                                iRing += 1;
                            }
                            free(internalNeighbors as *mut libc::c_void);
                            free(internalDistances as *mut libc::c_void);
                        }
                        free(ring as *mut libc::c_void);
                        k += 1;
                    }
                }
                j_1 += 1;
            }
            free(children as *mut libc::c_void);
            i_1 += 1;
        }
        res += 1;
    }
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"gridRingUnsafe\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"gridRingUnsafe\0" as *const u8 as *const libc::c_char,
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
