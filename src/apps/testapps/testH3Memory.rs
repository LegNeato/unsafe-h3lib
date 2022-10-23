extern crate unsafe_h3lib_applib;
use ::libc;
extern "C" {

    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn gridDisk(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn maxPolygonToCellsSize(
        geoPolygon: *const GeoPolygon,
        res: libc::c_int,
        flags: uint32_t,
        out: *mut int64_t,
    ) -> H3Error;
    fn polygonToCells(
        geoPolygon: *const GeoPolygon,
        res: libc::c_int,
        flags: uint32_t,
        out: *mut H3Index,
    ) -> H3Error;
    fn compactCells(
        h3Set: *const H3Index,
        compactedSet: *mut H3Index,
        numHexes: int64_t,
    ) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn countNonNullIndexes(indexes: *mut H3Index, numCells: int64_t) -> int64_t;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type int64_t = libc::c_longlong;
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
static mut failAlloc: bool = 0 as libc::c_int != 0;
static mut actualAllocCalls: libc::c_int = 0 as libc::c_int;
static mut actualFreeCalls: libc::c_int = 0 as libc::c_int;
static mut permittedAllocCalls: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn resetMemoryCounters(mut permitted: libc::c_int) {
    failAlloc = 0 as libc::c_int != 0;
    actualAllocCalls = 0 as libc::c_int;
    actualFreeCalls = 0 as libc::c_int;
    permittedAllocCalls = permitted;
}
#[no_mangle]
pub unsafe extern "C" fn test_prefix_malloc(mut size: size_t) -> *mut libc::c_void {
    actualAllocCalls += 1;
    if permittedAllocCalls != 0 && actualAllocCalls > permittedAllocCalls {
        failAlloc = 1 as libc::c_int != 0;
    }
    if failAlloc {
        return 0 as *mut libc::c_void;
    }
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn test_prefix_calloc(
    mut num: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    actualAllocCalls += 1;
    if permittedAllocCalls != 0 && actualAllocCalls > permittedAllocCalls {
        failAlloc = 1 as libc::c_int != 0;
    }
    if failAlloc {
        return 0 as *mut libc::c_void;
    }
    return calloc(num, size);
}
#[no_mangle]
pub unsafe extern "C" fn test_prefix_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    actualAllocCalls += 1;
    if permittedAllocCalls != 0 && actualAllocCalls > permittedAllocCalls {
        failAlloc = 1 as libc::c_int != 0;
    }
    if failAlloc {
        return 0 as *mut libc::c_void;
    }
    return realloc(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn test_prefix_free(mut ptr: *mut libc::c_void) {
    actualFreeCalls += 1;
    return free(ptr);
}
#[no_mangle]
pub static mut sunnyvale: H3Index = 0x89283470c27ffff as libc::c_long as H3Index;
#[no_mangle]
pub static mut pentagon: H3Index = 0x89080000003ffff as libc::c_long as H3Index;
static mut sfVerts: [LatLng; 6] = [
    {
        let mut init = LatLng {
            lat: 0.659966917655f64,
            lng: -2.1364398519396f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595011102219f64,
            lng: -2.1359434279405f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6583348114025f64,
            lng: -2.1354884206045f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6581220034068f64,
            lng: -2.1382437718946f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594479998527f64,
            lng: -2.1384597563896f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6599990002976f64,
            lng: -2.1376771158464f64,
        };
        init
    },
];
static mut sfGeoLoop: GeoLoop = unsafe {
    {
        let mut init = GeoLoop {
            numVerts: 6 as libc::c_int,
            verts: sfVerts.as_ptr() as *mut _,
        };
        init
    }
};
static mut sfGeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
unsafe extern "C" fn runTests() {
    currentTestName = b"gridDisk\0" as *const u8 as *const libc::c_char;
    let mut k: libc::c_int = 2 as libc::c_int;
    let mut hexCount: int64_t = 0;
    if maxGridDiskSize(k, &mut hexCount) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int,
            b"!(maxGridDiskSize(k, &hexCount))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut gridDiskOutput: *mut H3Index = calloc(
        hexCount as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    resetMemoryCounters(0 as libc::c_int);
    if gridDisk(sunnyvale, k, gridDiskOutput) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            b"!(gridDisk(sunnyvale, k, gridDiskOutput))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            b"actualAllocCalls == 0\0" as *const u8 as *const libc::c_char,
            b"gridDisk did not call alloc\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            b"actualFreeCalls == 0\0" as *const u8 as *const libc::c_char,
            b"gridDisk did not call free\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    resetMemoryCounters(0 as libc::c_int);
    if gridDisk(pentagon, k, gridDiskOutput) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            b"!(gridDisk(pentagon, k, gridDiskOutput))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            b"actualAllocCalls == 1\0" as *const u8 as *const libc::c_char,
            b"gridDisk called alloc\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            b"actualFreeCalls == 1\0" as *const u8 as *const libc::c_char,
            b"gridDisk called free\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    resetMemoryCounters(0 as libc::c_int);
    failAlloc = 1 as libc::c_int != 0;
    memset(
        gridDiskOutput as *mut libc::c_void,
        0 as libc::c_int,
        (hexCount as libc::c_ulonglong)
            .wrapping_mul(::core::mem::size_of::<H3Index>() as libc::c_ulong as libc::c_ulonglong)
            as libc::c_ulong,
    );
    if !(gridDisk(pentagon, k, gridDiskOutput) == E_MEMORY_ALLOC as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            b"H3_EXPORT(gridDisk)(pentagon, k, gridDiskOutput) == E_MEMORY_ALLOC\0" as *const u8
                as *const libc::c_char,
            b"gridDisk returns E_MEMORY_ALLOC\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            b"actualAllocCalls == 1\0" as *const u8 as *const libc::c_char,
            b"gridDisk called alloc\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            b"actualFreeCalls == 0\0" as *const u8 as *const libc::c_char,
            b"gridDisk did not call free\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < hexCount {
        if *gridDiskOutput.offset(i as isize) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
                123 as libc::c_int,
                b"!gridDiskOutput[i]\0" as *const u8 as *const libc::c_char,
                b"gridDisk did not produce output without alloc\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    free(gridDiskOutput as *mut libc::c_void);
    currentTestName = b"compactCells\0" as *const u8 as *const libc::c_char;
    let mut k_0: libc::c_int = 9 as libc::c_int;
    let mut hexCount_0: int64_t = 0;
    if maxGridDiskSize(k_0, &mut hexCount_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            b"!(maxGridDiskSize(k, &hexCount))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut expectedCompactCount: int64_t = 73 as libc::c_int as int64_t;
    let mut sunnyvaleExpanded: *mut H3Index = calloc(
        hexCount_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    resetMemoryCounters(0 as libc::c_int);
    if gridDisk(sunnyvale, k_0, sunnyvaleExpanded) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            b"!(gridDisk(sunnyvale, k, sunnyvaleExpanded))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            b"actualAllocCalls == 0\0" as *const u8 as *const libc::c_char,
            b"gridDisk did not call alloc\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            b"actualFreeCalls == 0\0" as *const u8 as *const libc::c_char,
            b"gridDisk did not call free\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut compressed: *mut H3Index = calloc(
        hexCount_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    resetMemoryCounters(0 as libc::c_int);
    failAlloc = 1 as libc::c_int != 0;
    let mut err: H3Error = compactCells(sunnyvaleExpanded, compressed, hexCount_0);
    if !(err == E_MEMORY_ALLOC as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            b"err == E_MEMORY_ALLOC\0" as *const u8 as *const libc::c_char,
            b"malloc failed (1)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            b"actualAllocCalls == 1\0" as *const u8 as *const libc::c_char,
            b"alloc called once\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            b"actualFreeCalls == 0\0" as *const u8 as *const libc::c_char,
            b"free not called\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    resetMemoryCounters(1 as libc::c_int);
    err = compactCells(sunnyvaleExpanded, compressed, hexCount_0);
    if !(err == E_MEMORY_ALLOC as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            b"err == E_MEMORY_ALLOC\0" as *const u8 as *const libc::c_char,
            b"malloc failed (2)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            b"actualAllocCalls == 2\0" as *const u8 as *const libc::c_char,
            b"alloc called twice\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_int,
            b"actualFreeCalls == 1\0" as *const u8 as *const libc::c_char,
            b"free called once\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    resetMemoryCounters(2 as libc::c_int);
    err = compactCells(sunnyvaleExpanded, compressed, hexCount_0);
    if !(err == E_MEMORY_ALLOC as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            b"err == E_MEMORY_ALLOC\0" as *const u8 as *const libc::c_char,
            b"malloc failed (3)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            b"actualAllocCalls == 3\0" as *const u8 as *const libc::c_char,
            b"alloc called three times\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            b"actualFreeCalls == 2\0" as *const u8 as *const libc::c_char,
            b"free called twice\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    resetMemoryCounters(3 as libc::c_int);
    err = compactCells(sunnyvaleExpanded, compressed, hexCount_0);
    if !(err == E_MEMORY_ALLOC as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int,
            b"err == E_MEMORY_ALLOC\0" as *const u8 as *const libc::c_char,
            b"compactCells failed (4)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 4 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int,
            b"actualAllocCalls == 4\0" as *const u8 as *const libc::c_char,
            b"alloc called four times\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            b"actualFreeCalls == 3\0" as *const u8 as *const libc::c_char,
            b"free called three times\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    resetMemoryCounters(4 as libc::c_int);
    err = compactCells(sunnyvaleExpanded, compressed, hexCount_0);
    if !(err == E_SUCCESS as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int,
            b"err == E_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"compact using successful malloc\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 4 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int,
            b"actualAllocCalls == 4\0" as *const u8 as *const libc::c_char,
            b"alloc called four times\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 4 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            b"actualFreeCalls == 4\0" as *const u8 as *const libc::c_char,
            b"free called four times\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut count: int64_t = 0 as libc::c_int as int64_t;
    let mut i_0: int64_t = 0 as libc::c_int as int64_t;
    while i_0 < hexCount_0 {
        if *compressed.offset(i_0 as isize) != 0 as libc::c_int as libc::c_ulonglong {
            count += 1;
        }
        i_0 += 1;
    }
    if !(count == expectedCompactCount) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            b"count == expectedCompactCount\0" as *const u8 as *const libc::c_char,
            b"got expected compact count\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(compressed as *mut libc::c_void);
    free(sunnyvaleExpanded as *mut libc::c_void);
    currentTestName = b"polygonToCells\0" as *const u8 as *const libc::c_char;
    sfGeoPolygon.geoloop = sfGeoLoop;
    sfGeoPolygon.numHoles = 0 as libc::c_int;
    let mut numHexagons: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut sfGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            b"!(maxPolygonToCellsSize(&sfGeoPolygon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons: *mut H3Index = calloc(
        numHexagons as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    resetMemoryCounters(0 as libc::c_int);
    failAlloc = 1 as libc::c_int != 0;
    let mut err_0: H3Error = polygonToCells(
        &mut sfGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons,
    );
    if !(err_0 == E_MEMORY_ALLOC as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            b"err == E_MEMORY_ALLOC\0" as *const u8 as *const libc::c_char,
            b"polygonToCells failed (1)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            b"actualAllocCalls == 1\0" as *const u8 as *const libc::c_char,
            b"alloc called once\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            b"actualFreeCalls == 0\0" as *const u8 as *const libc::c_char,
            b"free not called\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    resetMemoryCounters(1 as libc::c_int);
    err_0 = polygonToCells(
        &mut sfGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons,
    );
    if !(err_0 == E_MEMORY_ALLOC as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            b"err == E_MEMORY_ALLOC\0" as *const u8 as *const libc::c_char,
            b"polygonToCells failed (2)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int,
            b"actualAllocCalls == 2\0" as *const u8 as *const libc::c_char,
            b"alloc called twice\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            b"actualFreeCalls == 1\0" as *const u8 as *const libc::c_char,
            b"free called once\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    resetMemoryCounters(2 as libc::c_int);
    err_0 = polygonToCells(
        &mut sfGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons,
    );
    if !(err_0 == E_MEMORY_ALLOC as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            b"err == E_MEMORY_ALLOC\0" as *const u8 as *const libc::c_char,
            b"polygonToCells failed (3)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            b"actualAllocCalls == 3\0" as *const u8 as *const libc::c_char,
            b"alloc called three times\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            b"actualFreeCalls == 2\0" as *const u8 as *const libc::c_char,
            b"free called twice\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    resetMemoryCounters(3 as libc::c_int);
    err_0 = polygonToCells(
        &mut sfGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons,
    );
    if !(err_0 == E_SUCCESS as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int,
            b"err == E_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"polygonToCells succeeded (4)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualAllocCalls == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int,
            b"actualAllocCalls == 3\0" as *const u8 as *const libc::c_char,
            b"alloc called three times\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(actualFreeCalls == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int,
            b"actualFreeCalls == 3\0" as *const u8 as *const libc::c_char,
            b"free called three times\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes: int64_t = countNonNullIndexes(hexagons, numHexagons);
    if !(actualNumIndexes == 1253 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Memory.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            b"actualNumIndexes == 1253\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"h3Memory\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"h3Memory\0" as *const u8 as *const libc::c_char,
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
