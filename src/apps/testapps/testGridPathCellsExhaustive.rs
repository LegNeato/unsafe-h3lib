use ::libc;
extern "C" {
    pub type __sFILEX;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn gridDisk(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn areNeighborCells(origin: H3Index, destination: H3Index, out: *mut libc::c_int) -> H3Error;
    fn gridDistance(origin: H3Index, h3: H3Index, distance: *mut int64_t) -> H3Error;
    fn gridPathCellsSize(start: H3Index, end: H3Index, size: *mut int64_t) -> H3Error;
    fn gridPathCells(start: H3Index, end: H3Index, out: *mut H3Index) -> H3Error;
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
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type int64_t = libc::c_longlong;
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
static mut MAX_DISTANCES: [libc::c_int; 6] = [
    1 as libc::c_int,
    2 as libc::c_int,
    5 as libc::c_int,
    12 as libc::c_int,
    19 as libc::c_int,
    26 as libc::c_int,
];
unsafe extern "C" fn gridPathCells_assertions(mut start: H3Index, mut end: H3Index) {
    let mut sz: int64_t = 0;
    if gridPathCellsSize(start, end, &mut sz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            40 as libc::c_int,
            b"!(gridPathCellsSize(start, end, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(sz > 0 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            41 as libc::c_int,
            b"sz > 0\0" as *const u8 as *const libc::c_char,
            b"got valid size\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut line: *mut H3Index = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if gridPathCells(start, end, line) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            44 as libc::c_int,
            b"!(gridPathCells(start, end, line))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(*line.offset(0 as libc::c_int as isize) == start) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            46 as libc::c_int,
            b"line[0] == start\0" as *const u8 as *const libc::c_char,
            b"line starts with start index\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(*line.offset((sz - 1 as libc::c_int as libc::c_longlong) as isize) == end) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            47 as libc::c_int,
            b"line[sz - 1] == end\0" as *const u8 as *const libc::c_char,
            b"line ends with end index\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 1 as libc::c_int;
    while (i as libc::c_longlong) < sz {
        if isValidCell(*line.offset(i as isize)) == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0"
                    as *const u8 as *const libc::c_char,
                50 as libc::c_int,
                b"H3_EXPORT(isValidCell)(line[i])\0" as *const u8 as *const libc::c_char,
                b"index is valid\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut isNeighbor: libc::c_int = 0;
        if areNeighborCells(
            *line.offset(i as isize),
            *line.offset((i - 1 as libc::c_int) as isize),
            &mut isNeighbor,
        ) != 0
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0"
                    as *const u8 as *const libc::c_char,
                53 as libc::c_int,
                b"!(areNeighborCells(line[i], line[i - 1], &isNeighbor))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if isNeighbor == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0"
                    as *const u8 as *const libc::c_char,
                54 as libc::c_int,
                b"isNeighbor\0" as *const u8 as *const libc::c_char,
                b"index is a neighbor of the previous index\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if i > 1 as libc::c_int {
            if areNeighborCells(
                *line.offset(i as isize),
                *line.offset((i - 2 as libc::c_int) as isize),
                &mut isNeighbor,
            ) != 0
            {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    57 as libc::c_int,
                    b"!(areNeighborCells(line[i], line[i - 2], &isNeighbor))\0" as *const u8
                        as *const libc::c_char,
                    b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if isNeighbor != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0"
                        as *const u8 as *const libc::c_char,
                    60 as libc::c_int,
                    b"!isNeighbor\0" as *const u8 as *const libc::c_char,
                    b"index is not a neighbor of the index before the previous\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
    free(line as *mut libc::c_void);
}
unsafe extern "C" fn gridPathCells_invalid_assertions(mut start: H3Index, mut end: H3Index) {
    let mut sz: int64_t = 0;
    if !(gridPathCellsSize(start, end, &mut sz) != E_SUCCESS as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            73 as libc::c_int,
            b"H3_EXPORT(gridPathCellsSize)(start, end, &sz) != E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"line size marked as invalid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut line: *mut H3Index = 0 as *mut H3Index;
    let mut err: H3Error = gridPathCells(start, end, line);
    if !(err != E_SUCCESS as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            77 as libc::c_int,
            b"err != E_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"line marked as invalid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn gridPathCells_gridDisk_assertions(mut h3: H3Index) {
    let mut r: libc::c_int =
        ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if !(r <= 5 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            85 as libc::c_int,
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
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            89 as libc::c_int,
            b"!(maxGridDiskSize(maxK, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if isPentagon(h3) != 0 {
        return;
    }
    let mut neighbors: *mut H3Index = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if gridDisk(h3, maxK, neighbors) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testGridPathCellsExhaustive.c\0" as *const u8
                as *const libc::c_char,
            96 as libc::c_int,
            b"!(gridDisk(h3, maxK, neighbors))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_longlong) < sz {
        if !(*neighbors.offset(i as isize) == 0 as libc::c_int as libc::c_ulonglong) {
            let mut distance: int64_t = 0;
            let mut distanceError: H3Error =
                gridDistance(h3, *neighbors.offset(i as isize), &mut distance);
            if distanceError == E_SUCCESS as libc::c_int as libc::c_uint {
                gridPathCells_assertions(h3, *neighbors.offset(i as isize));
            } else {
                gridPathCells_invalid_assertions(h3, *neighbors.offset(i as isize));
            }
        }
        i += 1;
    }
    free(neighbors as *mut libc::c_void);
}
unsafe extern "C" fn runTests() {
    currentTestName = b"gridPathCells_gridDisk\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(gridPathCells_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(gridPathCells_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(gridPathCells_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtResPartial(
        3 as libc::c_int,
        Some(gridPathCells_gridDisk_assertions as unsafe extern "C" fn(H3Index) -> ()),
        6 as libc::c_int,
    );
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"gridPathCells\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"gridPathCells\0" as *const u8 as *const libc::c_char,
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
