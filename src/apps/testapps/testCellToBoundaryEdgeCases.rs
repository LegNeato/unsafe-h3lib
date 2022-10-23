use ::libc;
extern "C" {
    pub type __sFILEX;
    fn exit(_: libc::c_int) -> !;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
    fn degsToRads(degrees: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut __stderrp: *mut FILE;
    fn bboxFromGeoLoop(loop_0: *const GeoLoop, bbox: *mut BBox);
    fn pointInsideGeoLoop(loop_0: *const GeoLoop, bbox: *const BBox, coord: *const LatLng) -> bool;
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
pub struct GeoLoop {
    pub numVerts: libc::c_int,
    pub verts: *mut LatLng,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BBox {
    pub north: libc::c_double,
    pub south: libc::c_double,
    pub east: libc::c_double,
    pub west: libc::c_double,
}
unsafe extern "C" fn runTests() {
    currentTestName = b"doublePrecisionVertex\0" as *const u8 as *const libc::c_char;
    let mut cell: H3Index = 0x81083ffffffffff as libc::c_long as H3Index;
    let mut point: LatLng = {
        let mut init = LatLng {
            lat: degsToRads(61.890838431f64),
            lng: degsToRads(8.644221328f64),
        };
        init
    };
    let mut boundary: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    if cellToBoundary(cell, &mut boundary) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToBoundaryEdgeCases.c\0" as *const u8
                as *const libc::c_char,
            41 as libc::c_int,
            b"!(cellToBoundary(cell, &boundary))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut verts: *mut LatLng = (boundary.verts).as_mut_ptr();
    let mut geoloop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: boundary.numVerts,
            verts: verts,
        };
        init
    };
    let mut bbox: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromGeoLoop(&mut geoloop, &mut bbox);
    let mut cell2: H3Index = 0;
    if latLngToCell(&mut point, 1 as libc::c_int, &mut cell2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToBoundaryEdgeCases.c\0" as *const u8
                as *const libc::c_char,
            50 as libc::c_int,
            b"!(latLngToCell(&point, 1, &cell2))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cell2 == cell {
        if !pointInsideGeoLoop(&mut geoloop, &mut bbox, &mut point) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellToBoundaryEdgeCases.c\0"
                    as *const u8 as *const libc::c_char,
                54 as libc::c_int,
                b"pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8
                    as *const libc::c_char,
                b"Boundary contains input point\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
    } else {
        if pointInsideGeoLoop(&mut geoloop, &mut bbox, &mut point) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellToBoundaryEdgeCases.c\0"
                    as *const u8 as *const libc::c_char,
                57 as libc::c_int,
                b"!pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8
                    as *const libc::c_char,
                b"Boundary does not contain input point\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
    };
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"cellToBoundaryEdgeCases\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"cellToBoundaryEdgeCases\0" as *const u8 as *const libc::c_char,
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
