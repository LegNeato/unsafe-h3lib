use ::libc;
extern "C" {
    pub type __sFILEX;
    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn cellToChildrenSize(h: H3Index, childRes: libc::c_int, out: *mut int64_t) -> H3Error;
    fn cellToChildren(h: H3Index, childRes: libc::c_int, children: *mut H3Index) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
unsafe extern "C" fn assertNoDuplicates(mut cells: *mut H3Index, mut n: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        if !(*cells.offset(i as isize) == 0 as libc::c_int as libc::c_ulonglong) {
            if isValidCell(*cells.offset(i as isize)) == 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToChildren.c\0" as *const u8
                        as *const libc::c_char,
                    30 as libc::c_int,
                    b"H3_EXPORT(isValidCell)(cells[i])\0" as *const u8 as *const libc::c_char,
                    b"must be valid H3 cell\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            let mut j: libc::c_int = i + 1 as libc::c_int;
            while j < n {
                if !(*cells.offset(i as isize) != *cells.offset(j as isize)) {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testCellToChildren.c\0"
                            as *const u8 as *const libc::c_char,
                        32 as libc::c_int,
                        b"cells[i] != cells[j]\0" as *const u8 as *const libc::c_char,
                        b"can't have duplicate cells in set\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                j += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn assertSubset(
    mut set1: *mut H3Index,
    mut len1: libc::c_int,
    mut set2: *mut H3Index,
    mut len2: libc::c_int,
) {
    assertNoDuplicates(set1, len1);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len1 {
        if !(*set1.offset(i as isize) == 0 as libc::c_int as libc::c_ulonglong) {
            let mut present: bool = 0 as libc::c_int != 0;
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < len2 {
                if *set1.offset(i as isize) == *set2.offset(j as isize) {
                    present = 1 as libc::c_int != 0;
                    break;
                } else {
                    j += 1;
                }
            }
            if !present {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCellToChildren.c\0" as *const u8
                        as *const libc::c_char,
                    51 as libc::c_int,
                    b"present\0" as *const u8 as *const libc::c_char,
                    b"children must match\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
}
unsafe extern "C" fn assertSetsEqual(
    mut set1: *mut H3Index,
    mut len1: libc::c_int,
    mut set2: *mut H3Index,
    mut len2: libc::c_int,
) {
    assertSubset(set1, len1, set2, len2);
    assertSubset(set2, len2, set1, len1);
}
unsafe extern "C" fn checkChildren(
    mut h: H3Index,
    mut res: libc::c_int,
    mut expectedError: H3Error,
    mut expected: *mut H3Index,
    mut numExpected: libc::c_int,
) {
    let mut numChildren: int64_t = 0;
    let mut numChildrenError: H3Error = cellToChildrenSize(h, res, &mut numChildren);
    if !(numChildrenError == expectedError) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildren.c\0" as *const u8
                as *const libc::c_char,
            71 as libc::c_int,
            b"numChildrenError == expectedError\0" as *const u8 as *const libc::c_char,
            b"Expected error code\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if expectedError != E_SUCCESS as libc::c_int as libc::c_uint {
        return;
    }
    let mut children: *mut H3Index = calloc(
        numChildren as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if cellToChildren(h, res, children) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildren.c\0" as *const u8
                as *const libc::c_char,
            76 as libc::c_int,
            b"!(cellToChildren(h, res, children))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    assertSetsEqual(children, numChildren as libc::c_int, expected, numExpected);
    free(children as *mut libc::c_void);
}
unsafe extern "C" fn runTests() {
    currentTestName = b"oneResStep\0" as *const u8 as *const libc::c_char;
    let mut h: H3Index = 0x88283080ddfffff as libc::c_long as H3Index;
    let mut res: libc::c_int = 9 as libc::c_int;
    let mut expected: [H3Index; 7] = [
        0x89283080dc3ffff as libc::c_long as H3Index,
        0x89283080dc7ffff as libc::c_long as H3Index,
        0x89283080dcbffff as libc::c_long as H3Index,
        0x89283080dcfffff as libc::c_long as H3Index,
        0x89283080dd3ffff as libc::c_long as H3Index,
        0x89283080dd7ffff as libc::c_long as H3Index,
        0x89283080ddbffff as libc::c_long as H3Index,
    ];
    checkChildren(
        h,
        res,
        E_SUCCESS as libc::c_int as H3Error,
        expected.as_mut_ptr(),
        (::core::mem::size_of::<[H3Index; 7]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
            as libc::c_int,
    );
    currentTestName = b"multipleResSteps\0" as *const u8 as *const libc::c_char;
    let mut h_0: H3Index = 0x88283080ddfffff as libc::c_long as H3Index;
    let mut res_0: libc::c_int = 10 as libc::c_int;
    let mut expected_0: [H3Index; 49] = [
        0x8a283080dd27fff as libc::c_long as H3Index,
        0x8a283080dd37fff as libc::c_long as H3Index,
        0x8a283080dc47fff as libc::c_long as H3Index,
        0x8a283080dcdffff as libc::c_long as H3Index,
        0x8a283080dc5ffff as libc::c_long as H3Index,
        0x8a283080dc27fff as libc::c_long as H3Index,
        0x8a283080ddb7fff as libc::c_long as H3Index,
        0x8a283080dc07fff as libc::c_long as H3Index,
        0x8a283080dd8ffff as libc::c_long as H3Index,
        0x8a283080dd5ffff as libc::c_long as H3Index,
        0x8a283080dc4ffff as libc::c_long as H3Index,
        0x8a283080dd47fff as libc::c_long as H3Index,
        0x8a283080dce7fff as libc::c_long as H3Index,
        0x8a283080dd1ffff as libc::c_long as H3Index,
        0x8a283080dceffff as libc::c_long as H3Index,
        0x8a283080dc6ffff as libc::c_long as H3Index,
        0x8a283080dc87fff as libc::c_long as H3Index,
        0x8a283080dcaffff as libc::c_long as H3Index,
        0x8a283080dd2ffff as libc::c_long as H3Index,
        0x8a283080dcd7fff as libc::c_long as H3Index,
        0x8a283080dd9ffff as libc::c_long as H3Index,
        0x8a283080dd6ffff as libc::c_long as H3Index,
        0x8a283080dcc7fff as libc::c_long as H3Index,
        0x8a283080dca7fff as libc::c_long as H3Index,
        0x8a283080dccffff as libc::c_long as H3Index,
        0x8a283080dd77fff as libc::c_long as H3Index,
        0x8a283080dc97fff as libc::c_long as H3Index,
        0x8a283080dd4ffff as libc::c_long as H3Index,
        0x8a283080dd97fff as libc::c_long as H3Index,
        0x8a283080dc37fff as libc::c_long as H3Index,
        0x8a283080dc8ffff as libc::c_long as H3Index,
        0x8a283080dcb7fff as libc::c_long as H3Index,
        0x8a283080dcf7fff as libc::c_long as H3Index,
        0x8a283080dd87fff as libc::c_long as H3Index,
        0x8a283080dda7fff as libc::c_long as H3Index,
        0x8a283080dc9ffff as libc::c_long as H3Index,
        0x8a283080dc77fff as libc::c_long as H3Index,
        0x8a283080dc67fff as libc::c_long as H3Index,
        0x8a283080dc57fff as libc::c_long as H3Index,
        0x8a283080ddaffff as libc::c_long as H3Index,
        0x8a283080dd17fff as libc::c_long as H3Index,
        0x8a283080dc17fff as libc::c_long as H3Index,
        0x8a283080dd57fff as libc::c_long as H3Index,
        0x8a283080dc0ffff as libc::c_long as H3Index,
        0x8a283080dd07fff as libc::c_long as H3Index,
        0x8a283080dc1ffff as libc::c_long as H3Index,
        0x8a283080dd0ffff as libc::c_long as H3Index,
        0x8a283080dc2ffff as libc::c_long as H3Index,
        0x8a283080dd67fff as libc::c_long as H3Index,
    ];
    checkChildren(
        h_0,
        res_0,
        E_SUCCESS as libc::c_int as H3Error,
        expected_0.as_mut_ptr(),
        (::core::mem::size_of::<[H3Index; 49]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
            as libc::c_int,
    );
    currentTestName = b"sameRes\0" as *const u8 as *const libc::c_char;
    let mut h_1: H3Index = 0x88283080ddfffff as libc::c_long as H3Index;
    let mut res_1: libc::c_int = 8 as libc::c_int;
    let mut expected_1: [H3Index; 1] = [h_1];
    checkChildren(
        h_1,
        res_1,
        E_SUCCESS as libc::c_int as H3Error,
        expected_1.as_mut_ptr(),
        (::core::mem::size_of::<[H3Index; 1]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
            as libc::c_int,
    );
    currentTestName = b"childResTooCoarse\0" as *const u8 as *const libc::c_char;
    let mut h_2: H3Index = 0x88283080ddfffff as libc::c_long as H3Index;
    let mut res_2: libc::c_int = 7 as libc::c_int;
    let mut expected_2: [H3Index; 1] = [0 as libc::c_int as H3Index];
    checkChildren(
        h_2,
        res_2,
        E_RES_DOMAIN as libc::c_int as H3Error,
        expected_2.as_mut_ptr(),
        (::core::mem::size_of::<[H3Index; 1]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
            as libc::c_int,
    );
    currentTestName = b"childResTooFine\0" as *const u8 as *const libc::c_char;
    let mut h_3: H3Index = 0x8f283080dcb0ae2 as libc::c_long as H3Index;
    let mut res_3: libc::c_int = 15 as libc::c_int + 1 as libc::c_int;
    let mut expected_3: [H3Index; 1] = [0 as libc::c_int as H3Index];
    checkChildren(
        h_3,
        res_3,
        E_RES_DOMAIN as libc::c_int as H3Error,
        expected_3.as_mut_ptr(),
        (::core::mem::size_of::<[H3Index; 1]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
            as libc::c_int,
    );
    currentTestName = b"pentagonChildren\0" as *const u8 as *const libc::c_char;
    let mut h_4: H3Index = 0x81083ffffffffff as libc::c_long as H3Index;
    let mut res_4: libc::c_int = 3 as libc::c_int;
    let mut expected_4: [H3Index; 41] = [
        0x830800fffffffff as libc::c_long as H3Index,
        0x830802fffffffff as libc::c_long as H3Index,
        0x830803fffffffff as libc::c_long as H3Index,
        0x830804fffffffff as libc::c_long as H3Index,
        0x830805fffffffff as libc::c_long as H3Index,
        0x830806fffffffff as libc::c_long as H3Index,
        0x830810fffffffff as libc::c_long as H3Index,
        0x830811fffffffff as libc::c_long as H3Index,
        0x830812fffffffff as libc::c_long as H3Index,
        0x830813fffffffff as libc::c_long as H3Index,
        0x830814fffffffff as libc::c_long as H3Index,
        0x830815fffffffff as libc::c_long as H3Index,
        0x830816fffffffff as libc::c_long as H3Index,
        0x830818fffffffff as libc::c_long as H3Index,
        0x830819fffffffff as libc::c_long as H3Index,
        0x83081afffffffff as libc::c_long as H3Index,
        0x83081bfffffffff as libc::c_long as H3Index,
        0x83081cfffffffff as libc::c_long as H3Index,
        0x83081dfffffffff as libc::c_long as H3Index,
        0x83081efffffffff as libc::c_long as H3Index,
        0x830820fffffffff as libc::c_long as H3Index,
        0x830821fffffffff as libc::c_long as H3Index,
        0x830822fffffffff as libc::c_long as H3Index,
        0x830823fffffffff as libc::c_long as H3Index,
        0x830824fffffffff as libc::c_long as H3Index,
        0x830825fffffffff as libc::c_long as H3Index,
        0x830826fffffffff as libc::c_long as H3Index,
        0x830828fffffffff as libc::c_long as H3Index,
        0x830829fffffffff as libc::c_long as H3Index,
        0x83082afffffffff as libc::c_long as H3Index,
        0x83082bfffffffff as libc::c_long as H3Index,
        0x83082cfffffffff as libc::c_long as H3Index,
        0x83082dfffffffff as libc::c_long as H3Index,
        0x83082efffffffff as libc::c_long as H3Index,
        0x830830fffffffff as libc::c_long as H3Index,
        0x830831fffffffff as libc::c_long as H3Index,
        0x830832fffffffff as libc::c_long as H3Index,
        0x830833fffffffff as libc::c_long as H3Index,
        0x830834fffffffff as libc::c_long as H3Index,
        0x830835fffffffff as libc::c_long as H3Index,
        0x830836fffffffff as libc::c_long as H3Index,
    ];
    checkChildren(
        h_4,
        res_4,
        E_SUCCESS as libc::c_int as H3Error,
        expected_4.as_mut_ptr(),
        (::core::mem::size_of::<[H3Index; 41]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<H3Index>() as libc::c_ulong)
            as libc::c_int,
    );
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"cellToChildren_new\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"cellToChildren_new\0" as *const u8 as *const libc::c_char,
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
