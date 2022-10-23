extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;
extern crate unsafe_h3lib_testapps_lib;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    fn getNumCells(res: libc::c_int, out: *mut int64_t) -> H3Error;
    fn getResolution(h: H3Index) -> libc::c_int;
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn iterInitParent(h: H3Index, childRes: libc::c_int) -> IterCellsChildren;
    fn iterInitBaseCellNum(baseCellNum: libc::c_int, childRes: libc::c_int) -> IterCellsChildren;
    fn iterStepChild(iter: *mut IterCellsChildren);
    fn iterInitRes(res: libc::c_int) -> IterCellsResolution;
    fn iterStepRes(iter: *mut IterCellsResolution);
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    static mut globalTestCount: libc::c_int;
}
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type H3Index = uint64_t;
pub type H3Error = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IterCellsChildren {
    pub h: H3Index,
    pub _parentRes: libc::c_int,
    pub _skipDigit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IterCellsResolution {
    pub h: H3Index,
    pub _baseCellNum: libc::c_int,
    pub _res: libc::c_int,
    pub _itC: IterCellsChildren,
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
unsafe extern "C" fn test_number(mut res: libc::c_int) {
    let mut count: int64_t = 0 as libc::c_int as int64_t;
    let mut iter: IterCellsResolution = iterInitRes(res);
    while iter.h != 0 {
        count += 1;
        iterStepRes(&mut iter);
    }
    let mut expected: int64_t = 0;
    if getNumCells(res, &mut expected) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            b"!(getNumCells(res, &expected))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(count == expected) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
            b"count == expected\0" as *const u8 as *const libc::c_char,
            b"expect the correct number of cells from the iterator\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn test_valid(mut res: libc::c_int) {
    let mut iter: IterCellsResolution = iterInitRes(res);
    while iter.h != 0 {
        if isValidCell(iter.h) == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int,
                b"H3_EXPORT(isValidCell)(iter.h)\0" as *const u8 as *const libc::c_char,
                b"iterator cell is valid\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        iterStepRes(&mut iter);
    }
}
unsafe extern "C" fn test_resolution(mut res: libc::c_int) {
    let mut iter: IterCellsResolution = iterInitRes(res);
    while iter.h != 0 {
        if !(getResolution(iter.h) == res) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int,
                b"H3_EXPORT(getResolution)(iter.h) == res\0" as *const u8 as *const libc::c_char,
                b"iterator cell is the correct resolution\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        iterStepRes(&mut iter);
    }
}
unsafe extern "C" fn test_ordered(mut res: libc::c_int) {
    let mut iter: IterCellsResolution = iterInitRes(res);
    let mut prev: H3Index = iter.h;
    iterStepRes(&mut iter);
    while iter.h != 0 {
        if !(prev < iter.h) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                b"prev < iter.h\0" as *const u8 as *const libc::c_char,
                b"cells should be iterated in order without duplicates\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        prev = iter.h;
        iterStepRes(&mut iter);
    }
}
unsafe extern "C" fn assert_is_null_iterator(mut iter: IterCellsChildren) {
    if !(iter.h == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            b"iter.h == H3_NULL\0" as *const u8 as *const libc::c_char,
            b"null iterator cell is H3_NULL\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(iter._parentRes == -(1 as libc::c_int)) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            b"iter._parentRes == -1\0" as *const u8 as *const libc::c_char,
            b"null iterator parentRes is -1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(iter._skipDigit == -(1 as libc::c_int)) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            b"iter._skipDigit == -1\0" as *const u8 as *const libc::c_char,
            b"null iterator skipDigit is -1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn runTests() {
    currentTestName = b"iterator_setup_invalid\0" as *const u8 as *const libc::c_char;
    assert_is_null_iterator(iterInitBaseCellNum(-(1 as libc::c_int), 0 as libc::c_int));
    assert_is_null_iterator(iterInitBaseCellNum(1000 as libc::c_int, 0 as libc::c_int));
    assert_is_null_iterator(iterInitBaseCellNum(0 as libc::c_int, -(1 as libc::c_int)));
    assert_is_null_iterator(iterInitBaseCellNum(0 as libc::c_int, 100 as libc::c_int));
    assert_is_null_iterator(iterInitParent(
        0 as libc::c_int as H3Index,
        0 as libc::c_int,
    ));
    let mut test: H3Index = 0x85283473fffffff as libc::c_long as H3Index;
    assert_is_null_iterator(iterInitParent(test, 0 as libc::c_int));
    assert_is_null_iterator(iterInitParent(test, 100 as libc::c_int));
    currentTestName = b"null_iterator_base_cell\0" as *const u8 as *const libc::c_char;
    let mut iter: IterCellsChildren = iterInitBaseCellNum(-(1 as libc::c_int), 0 as libc::c_int);
    assert_is_null_iterator(iter);
    iterStepChild(&mut iter);
    if !(iter.h == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            b"iter.h == H3_NULL\0" as *const u8 as *const libc::c_char,
            b"null iterator returns null\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"null_iterator_res\0" as *const u8 as *const libc::c_char;
    let mut iter_0: IterCellsResolution = iterInitRes(-(1 as libc::c_int));
    assert_is_null_iterator(iter_0._itC);
    iterStepRes(&mut iter_0);
    if !(iter_0.h == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Iterators.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int,
            b"iter.h == H3_NULL\0" as *const u8 as *const libc::c_char,
            b"null iterator returns null\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"iterator_cell_count\0" as *const u8 as *const libc::c_char;
    test_number(0 as libc::c_int);
    test_number(1 as libc::c_int);
    test_number(2 as libc::c_int);
    test_number(3 as libc::c_int);
    currentTestName = b"iterator_cell_valid\0" as *const u8 as *const libc::c_char;
    test_valid(0 as libc::c_int);
    test_valid(1 as libc::c_int);
    test_valid(2 as libc::c_int);
    test_valid(3 as libc::c_int);
    currentTestName = b"iterator_cell_resolution\0" as *const u8 as *const libc::c_char;
    test_resolution(0 as libc::c_int);
    test_resolution(1 as libc::c_int);
    test_resolution(2 as libc::c_int);
    test_resolution(3 as libc::c_int);
    currentTestName = b"iterator_cell_ordered\0" as *const u8 as *const libc::c_char;
    test_ordered(0 as libc::c_int);
    test_ordered(1 as libc::c_int);
    test_ordered(2 as libc::c_int);
    test_ordered(3 as libc::c_int);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"h3Iterators\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"h3Iterators\0" as *const u8 as *const libc::c_char,
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
