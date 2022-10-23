use ::libc;
extern "C" {
    pub type __sFILEX;
    fn exit(_: libc::c_int) -> !;
    fn rand() -> libc::c_int;
    fn srand(_: libc::c_uint);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fflush(_: *mut FILE) -> libc::c_int;
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    fn degsToRads(degrees: libc::c_double) -> libc::c_double;
    fn radsToDegs(radians: libc::c_double) -> libc::c_double;
    fn originToDirectedEdges(origin: H3Index, edges: *mut H3Index) -> H3Error;
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    fn time(_: *mut time_t) -> time_t;
    fn iterInitBaseCellNum(baseCellNum: libc::c_int, childRes: libc::c_int) -> IterCellsChildren;
    fn iterStepChild(iter: *mut IterCellsChildren);
    fn iterInitRes(res: libc::c_int) -> IterCellsResolution;
    fn iterStepRes(iter: *mut IterCellsResolution);
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_time_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellBoundary {
    pub numVerts: libc::c_int,
    pub verts: [LatLng; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoordIJK {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub k: libc::c_int,
}
pub type time_t = __darwin_time_t;
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
#[no_mangle]
pub unsafe extern "C" fn error(mut msg: *const libc::c_char) {
    fflush(__stdoutp);
    fflush(__stderrp);
    fprintf(
        __stderrp,
        b"ERROR: %s.\n\0" as *const u8 as *const libc::c_char,
        msg,
    );
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn h3Print(mut h: H3Index) {
    printf(b"%llx\0" as *const u8 as *const libc::c_char, h);
}
#[no_mangle]
pub unsafe extern "C" fn h3Println(mut h: H3Index) {
    printf(b"%llx\n\0" as *const u8 as *const libc::c_char, h);
}
#[no_mangle]
pub unsafe extern "C" fn coordIjkPrint(mut c: *const CoordIJK) {
    printf(
        b"[%d, %d, %d]\0" as *const u8 as *const libc::c_char,
        (*c).i,
        (*c).j,
        (*c).k,
    );
}
#[no_mangle]
pub unsafe extern "C" fn geoToStringRads(mut p: *const LatLng, mut str: *mut libc::c_char) {
    sprintf(
        str,
        b"(%.4lf, %.4lf)\0" as *const u8 as *const libc::c_char,
        (*p).lat,
        (*p).lng,
    );
}
#[no_mangle]
pub unsafe extern "C" fn geoToStringDegs(mut p: *const LatLng, mut str: *mut libc::c_char) {
    sprintf(
        str,
        b"(%.9lf, %.9lf)\0" as *const u8 as *const libc::c_char,
        radsToDegs((*p).lat),
        radsToDegs((*p).lng),
    );
}
#[no_mangle]
pub unsafe extern "C" fn geoToStringDegsNoFmt(mut p: *const LatLng, mut str: *mut libc::c_char) {
    sprintf(
        str,
        b"%.9lf %.9lf\0" as *const u8 as *const libc::c_char,
        radsToDegs((*p).lat),
        radsToDegs((*p).lng),
    );
}
#[no_mangle]
pub unsafe extern "C" fn geoPrint(mut p: *const LatLng) {
    let mut buff: [libc::c_char; 256] = [0; 256];
    geoToStringDegs(p, buff.as_mut_ptr());
    printf(
        b"%s\0" as *const u8 as *const libc::c_char,
        buff.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn geoPrintln(mut p: *const LatLng) {
    geoPrint(p);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn geoPrintNoFmt(mut p: *const LatLng) {
    let mut buff: [libc::c_char; 256] = [0; 256];
    geoToStringDegsNoFmt(p, buff.as_mut_ptr());
    printf(
        b"%s\0" as *const u8 as *const libc::c_char,
        buff.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn geoPrintlnNoFmt(mut p: *const LatLng) {
    geoPrintNoFmt(p);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn cellBoundaryPrint(mut b: *const CellBoundary) {
    let mut buff: [libc::c_char; 256] = [0; 256];
    printf(b"{\0" as *const u8 as *const libc::c_char);
    let mut v: libc::c_int = 0 as libc::c_int;
    while v < (*b).numVerts {
        geoToStringDegsNoFmt(
            &*((*b).verts).as_ptr().offset(v as isize),
            buff.as_mut_ptr(),
        );
        printf(
            b"%s \0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
        );
        v += 1;
    }
    printf(b"}\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn cellBoundaryPrintln(mut b: *const CellBoundary) {
    let mut buff: [libc::c_char; 256] = [0; 256];
    printf(b"{\n\0" as *const u8 as *const libc::c_char);
    let mut v: libc::c_int = 0 as libc::c_int;
    while v < (*b).numVerts {
        geoToStringDegsNoFmt(
            &*((*b).verts).as_ptr().offset(v as isize),
            buff.as_mut_ptr(),
        );
        printf(
            b"   %s\n\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
        );
        v += 1;
    }
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn iterateAllDirectedEdgesAtRes(
    mut res: libc::c_int,
    mut callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
) {
    let mut iter: IterCellsResolution = iterInitRes(res);
    while iter.h != 0 {
        let mut edges: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
        originToDirectedEdges(iter.h, edges.as_mut_ptr());
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            if edges[i as usize] != 0 as libc::c_int as libc::c_ulonglong {
                (Some(callback.expect("non-null function pointer")))
                    .expect("non-null function pointer")(edges[i as usize]);
            }
            i += 1;
        }
        iterStepRes(&mut iter);
    }
}
#[no_mangle]
pub unsafe extern "C" fn iterateAllIndexesAtRes(
    mut res: libc::c_int,
    mut callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
) {
    iterateAllIndexesAtResPartial(res, callback, 122 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn iterateAllIndexesAtResPartial(
    mut res: libc::c_int,
    mut callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    mut baseCells: libc::c_int,
) {
    if !(baseCells <= 122 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"iterateAllIndexesAtResPartial\0",
            ))
            .as_ptr(),
            b"utility.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            b"baseCells <= NUM_BASE_CELLS\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < baseCells {
        iterateBaseCellIndexesAtRes(res, callback, i);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn iterateBaseCellIndexesAtRes(
    mut res: libc::c_int,
    mut callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    mut baseCell: libc::c_int,
) {
    let mut iter: IterCellsChildren = iterInitBaseCellNum(baseCell, res);
    while iter.h != 0 {
        (Some(callback.expect("non-null function pointer"))).expect("non-null function pointer")(
            iter.h,
        );
        iterStepChild(&mut iter);
    }
}
#[no_mangle]
pub unsafe extern "C" fn randomGeo(mut g: *mut LatLng) {
    static mut init: libc::c_int = 0 as libc::c_int;
    if init == 0 {
        srand(time(0 as *mut time_t) as libc::c_uint);
        init = 1 as libc::c_int;
    }
    (*g).lat = degsToRads(
        (rand() as libc::c_float / 0x7fffffff as libc::c_int as libc::c_float) as libc::c_double
            * 180.0f64
            - 90.0f64,
    );
    (*g).lng = degsToRads(
        (rand() as libc::c_float / 0x7fffffff as libc::c_int as libc::c_float) as libc::c_double,
    ) * 360.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn countNonNullIndexes(
    mut indexes: *mut H3Index,
    mut numCells: int64_t,
) -> int64_t {
    let mut nonNullIndexes: int64_t = 0 as libc::c_int as int64_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_longlong) < numCells {
        if *indexes.offset(i as isize) != 0 as libc::c_int as libc::c_ulonglong {
            nonNullIndexes += 1;
        }
        i += 1;
    }
    return nonNullIndexes;
}
