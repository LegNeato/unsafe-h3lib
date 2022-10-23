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
    fn gridDisksUnsafe(
        h3Set: *mut H3Index,
        length: libc::c_int,
        k: libc::c_int,
        out: *mut H3Index,
    ) -> H3Error;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                as *const libc::c_char,
            25 as libc::c_int,
            b"!(latLngToCell(&sf, 9, &sfHex))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut sfHexPtr: *mut H3Index = &mut sfHex;
    let mut k1: [H3Index; 6] = [
        0x89283080ddbffff as libc::c_long as H3Index,
        0x89283080c37ffff as libc::c_long as H3Index,
        0x89283080c27ffff as libc::c_long as H3Index,
        0x89283080d53ffff as libc::c_long as H3Index,
        0x89283080dcfffff as libc::c_long as H3Index,
        0x89283080dc3ffff as libc::c_long as H3Index,
    ];
    let mut withPentagon: [H3Index; 2] = [
        0x8029fffffffffff as libc::c_long as H3Index,
        0x801dfffffffffff as libc::c_long as H3Index,
    ];
    currentTestName = b"identityGridDiskCells\0" as *const u8 as *const libc::c_char;
    let mut k0: [H3Index; 1] = [0 as libc::c_int as H3Index];
    if gridDisksUnsafe(
        sfHexPtr,
        1 as libc::c_int,
        0 as libc::c_int,
        k0.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                as *const libc::c_char,
            34 as libc::c_int,
            b"!(gridDisksUnsafe(sfHexPtr, 1, 0, k0))\0" as *const u8 as *const libc::c_char,
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
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                as *const libc::c_char,
            36 as libc::c_int,
            b"k0[0] == sfHex\0" as *const u8 as *const libc::c_char,
            b"generated identity k-ring\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"ring1of1\0" as *const u8 as *const libc::c_char;
    let mut allKrings: [H3Index; 42] = [
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
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    if gridDisksUnsafe(
        k1.as_mut_ptr(),
        6 as libc::c_int,
        1 as libc::c_int,
        allKrings.as_mut_ptr(),
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                as *const libc::c_char,
            44 as libc::c_int,
            b"!(gridDisksUnsafe(k1, 6, 1, allKrings))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 42 as libc::c_int {
        if !(allKrings[i as usize] != 0 as libc::c_int as libc::c_ulonglong) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                    as *const libc::c_char,
                47 as libc::c_int,
                b"allKrings[i] != 0\0" as *const u8 as *const libc::c_char,
                b"index is populated\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if i % 7 as libc::c_int == 0 as libc::c_int {
            let mut index: libc::c_int = i / 7 as libc::c_int;
            if !(k1[index as usize] == allKrings[i as usize]) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                        as *const libc::c_char,
                    51 as libc::c_int,
                    b"k1[index] == allKrings[i]\0" as *const u8 as *const libc::c_char,
                    b"The beginning of the segment is the correct hexagon\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
    currentTestName = b"ring2of1\0" as *const u8 as *const libc::c_char;
    let mut allKrings2: *mut H3Index = calloc(
        (6 as libc::c_int * (1 as libc::c_int + 6 as libc::c_int + 12 as libc::c_int))
            as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if gridDisksUnsafe(
        k1.as_mut_ptr(),
        6 as libc::c_int,
        2 as libc::c_int,
        allKrings2,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                as *const libc::c_char,
            58 as libc::c_int,
            b"!(gridDisksUnsafe(k1, 6, 2, allKrings2))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 6 as libc::c_int * (1 as libc::c_int + 6 as libc::c_int + 12 as libc::c_int) {
        if !(*allKrings2.offset(i_0 as isize) != 0 as libc::c_int as libc::c_ulonglong) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                    as *const libc::c_char,
                61 as libc::c_int,
                b"allKrings2[i] != 0\0" as *const u8 as *const libc::c_char,
                b"index is populated\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if i_0 % (1 as libc::c_int + 6 as libc::c_int + 12 as libc::c_int) == 0 as libc::c_int {
            let mut index_0: libc::c_int =
                i_0 / (1 as libc::c_int + 6 as libc::c_int + 12 as libc::c_int);
            if !(k1[index_0 as usize] == *allKrings2.offset(i_0 as isize)) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                        as *const libc::c_char,
                    66 as libc::c_int,
                    b"k1[index] == allKrings2[i]\0" as *const u8 as *const libc::c_char,
                    b"The beginning of the segment is the correct hexagon\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_0 += 1;
    }
    free(allKrings2 as *mut libc::c_void);
    currentTestName = b"failed\0" as *const u8 as *const libc::c_char;
    let mut allKrings_0: *mut H3Index = calloc(
        (2 as libc::c_int * (1 as libc::c_int + 6 as libc::c_int)) as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if !(gridDisksUnsafe(
        withPentagon.as_mut_ptr(),
        2 as libc::c_int,
        1 as libc::c_int,
        allKrings_0,
    ) == E_PENTAGON as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                as *const libc::c_char,
            76 as libc::c_int,
            b"H3_EXPORT(gridDisksUnsafe)(withPentagon, 2, 1, allKrings) == E_PENTAGON\0"
                as *const u8 as *const libc::c_char,
            b"Expected error on gridDisksUnsafe\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(allKrings_0 as *mut libc::c_void);
    currentTestName = b"invalid_k\0" as *const u8 as *const libc::c_char;
    if !(gridDisksUnsafe(
        k1.as_mut_ptr(),
        6 as libc::c_int,
        -(1 as libc::c_int),
        0 as *mut H3Index,
    ) == E_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridDisksUnsafe.c\0" as *const u8
                as *const libc::c_char,
            82 as libc::c_int,
            b"H3_EXPORT(gridDisksUnsafe)(k1, 6, -1, NULL) == E_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"gridDisksUnsafe invalid k\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"gridDisksUnsafe\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"gridDisksUnsafe\0" as *const u8 as *const libc::c_char,
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
