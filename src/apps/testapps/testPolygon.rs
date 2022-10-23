extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn destroyLinkedMultiPolygon(polygon: *mut LinkedGeoPolygon);
    fn bboxEquals(b1: *const BBox, b2: *const BBox) -> bool;
    fn normalizeMultiPolygon(root: *mut LinkedGeoPolygon) -> H3Error;
    fn addNewLinkedPolygon(polygon: *mut LinkedGeoPolygon) -> *mut LinkedGeoPolygon;
    fn addLinkedLoop(
        polygon: *mut LinkedGeoPolygon,
        loop_0: *mut LinkedGeoLoop,
    ) -> *mut LinkedGeoLoop;
    fn addLinkedCoord(loop_0: *mut LinkedGeoLoop, vertex: *const LatLng) -> *mut LinkedLatLng;
    fn countLinkedPolygons(polygon: *mut LinkedGeoPolygon) -> libc::c_int;
    fn countLinkedLoops(polygon: *mut LinkedGeoPolygon) -> libc::c_int;
    fn countLinkedCoords(loop_0: *mut LinkedGeoLoop) -> libc::c_int;
    fn destroyLinkedGeoLoop(loop_0: *mut LinkedGeoLoop);
    fn bboxFromLinkedGeoLoop(loop_0: *const LinkedGeoLoop, bbox: *mut BBox);
    fn pointInsideLinkedGeoLoop(
        loop_0: *const LinkedGeoLoop,
        bbox: *const BBox,
        coord: *const LatLng,
    ) -> bool;
    fn isClockwiseLinkedGeoLoop(loop_0: *const LinkedGeoLoop) -> bool;
    fn bboxesFromGeoPolygon(polygon: *const GeoPolygon, bboxes: *mut BBox);
    fn bboxFromGeoLoop(loop_0: *const GeoLoop, bbox: *mut BBox);
    fn pointInsideGeoLoop(loop_0: *const GeoLoop, bbox: *const BBox, coord: *const LatLng) -> bool;
    fn isClockwiseGeoLoop(geoloop: *const GeoLoop) -> bool;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type uint32_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkedLatLng {
    pub vertex: LatLng,
    pub next: *mut LinkedLatLng,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkedGeoLoop {
    pub first: *mut LinkedLatLng,
    pub last: *mut LinkedLatLng,
    pub next: *mut LinkedGeoLoop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkedGeoPolygon {
    pub first: *mut LinkedGeoLoop,
    pub last: *mut LinkedGeoLoop,
    pub next: *mut LinkedGeoPolygon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BBox {
    pub north: libc::c_double,
    pub south: libc::c_double,
    pub east: libc::c_double,
    pub west: libc::c_double,
}
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
unsafe extern "C" fn createLinkedLoop(
    mut loop_0: *mut LinkedGeoLoop,
    mut verts: *mut LatLng,
    mut numVerts: libc::c_int,
) {
    *loop_0 = {
        let mut init = LinkedGeoLoop {
            first: 0 as *mut LinkedLatLng,
            last: 0 as *mut LinkedLatLng,
            next: 0 as *mut LinkedGeoLoop,
        };
        init
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numVerts {
        let fresh0 = verts;
        verts = verts.offset(1);
        addLinkedCoord(loop_0, fresh0);
        i += 1;
    }
}
unsafe extern "C" fn runTests() {
    currentTestName = b"pointInsideGeoLoop\0" as *const u8 as *const libc::c_char;
    let mut geoloop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 6 as libc::c_int,
            verts: sfVerts.as_mut_ptr(),
        };
        init
    };
    let mut inside: LatLng = {
        let mut init = LatLng {
            lat: 0.659f64,
            lng: -2.136f64,
        };
        init
    };
    let mut somewhere: LatLng = {
        let mut init = LatLng {
            lat: 1 as libc::c_int as libc::c_double,
            lng: 2 as libc::c_int as libc::c_double,
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
    if pointInsideGeoLoop(
        &mut geoloop,
        &mut bbox,
        &mut *sfVerts.as_mut_ptr().offset(0 as libc::c_int as isize),
    ) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            b"!pointInsideGeoLoop(&geoloop, &bbox, &sfVerts[0])\0" as *const u8
                as *const libc::c_char,
            b"does not contain exact vertex 0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !pointInsideGeoLoop(
        &mut geoloop,
        &mut bbox,
        &mut *sfVerts.as_mut_ptr().offset(3 as libc::c_int as isize),
    ) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            b"pointInsideGeoLoop(&geoloop, &bbox, &sfVerts[3])\0" as *const u8
                as *const libc::c_char,
            b"contains exact vertex 3\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !pointInsideGeoLoop(&mut geoloop, &mut bbox, &mut inside) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            b"pointInsideGeoLoop(&geoloop, &bbox, &inside)\0" as *const u8 as *const libc::c_char,
            b"contains point inside\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if pointInsideGeoLoop(&mut geoloop, &mut bbox, &mut somewhere) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            b"!pointInsideGeoLoop(&geoloop, &bbox, &somewhere)\0" as *const u8
                as *const libc::c_char,
            b"contains somewhere else\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"pointInsideGeoLoopCornerCases\0" as *const u8 as *const libc::c_char;
    let mut verts: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut geoloop_0: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts.as_mut_ptr(),
        };
        init
    };
    let mut bbox_0: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromGeoLoop(&mut geoloop_0, &mut bbox_0);
    let mut point: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    if pointInsideGeoLoop(&mut geoloop_0, &mut bbox_0, &mut point) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            b"!pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8 as *const libc::c_char,
            b"does not contain sw corner\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    point.lat = 1 as libc::c_int as libc::c_double;
    if pointInsideGeoLoop(&mut geoloop_0, &mut bbox_0, &mut point) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int,
            b"!pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8 as *const libc::c_char,
            b"does not contain nw corner \0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    point.lng = 1 as libc::c_int as libc::c_double;
    if pointInsideGeoLoop(&mut geoloop_0, &mut bbox_0, &mut point) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            b"!pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8 as *const libc::c_char,
            b"does not contain ne corner \0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    point.lat = 0 as libc::c_int as libc::c_double;
    if !pointInsideGeoLoop(&mut geoloop_0, &mut bbox_0, &mut point) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            b"pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8 as *const libc::c_char,
            b"contains se corner \0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"pointInsideGeoLoopEdgeCases\0" as *const u8 as *const libc::c_char;
    let mut verts_0: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut geoloop_1: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_0.as_mut_ptr(),
        };
        init
    };
    let mut bbox_1: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromGeoLoop(&mut geoloop_1, &mut bbox_1);
    let mut point_0: LatLng = LatLng { lat: 0., lng: 0. };
    point_0.lat = 0.5f64;
    point_0.lng = 0 as libc::c_int as libc::c_double;
    if pointInsideGeoLoop(&mut geoloop_1, &mut bbox_1, &mut point_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            b"!pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8 as *const libc::c_char,
            b"does not contain point on west edge\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    point_0.lat = 1 as libc::c_int as libc::c_double;
    point_0.lng = 0.5f64;
    if pointInsideGeoLoop(&mut geoloop_1, &mut bbox_1, &mut point_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            b"!pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8 as *const libc::c_char,
            b"does not contain point on north edge\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    point_0.lat = 0.5f64;
    point_0.lng = 1 as libc::c_int as libc::c_double;
    if !pointInsideGeoLoop(&mut geoloop_1, &mut bbox_1, &mut point_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            b"pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8 as *const libc::c_char,
            b"contains point on east edge\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    point_0.lat = 0 as libc::c_int as libc::c_double;
    point_0.lng = 0.5f64;
    if !pointInsideGeoLoop(&mut geoloop_1, &mut bbox_1, &mut point_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
            b"pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8 as *const libc::c_char,
            b"contains point on south edge\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"pointInsideGeoLoopExtraEdgeCase\0" as *const u8 as *const libc::c_char;
    let mut verts_1: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 0.5f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut geoloop_2: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 3 as libc::c_int,
            verts: verts_1.as_mut_ptr(),
        };
        init
    };
    let mut bbox_2: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromGeoLoop(&mut geoloop_2, &mut bbox_2);
    let mut point_1: LatLng = {
        let mut init = LatLng {
            lat: 0.5f64,
            lng: 0.5f64,
        };
        init
    };
    if !pointInsideGeoLoop(&mut geoloop_2, &mut bbox_2, &mut point_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            b"pointInsideGeoLoop(&geoloop, &bbox, &point)\0" as *const u8 as *const libc::c_char,
            b"contains inside point matching longitude of a vertex\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"pointInsideGeoLoopTransmeridian\0" as *const u8 as *const libc::c_char;
    let mut verts_2: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.01f64,
                lng: -3.14159265358979323846f64 + 0.01f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.01f64,
                lng: 3.14159265358979323846f64 - 0.01f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.01f64,
                lng: 3.14159265358979323846f64 - 0.01f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.01f64,
                lng: -3.14159265358979323846f64 + 0.01f64,
            };
            init
        },
    ];
    let mut transMeridianGeoLoop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_2.as_mut_ptr(),
        };
        init
    };
    let mut eastPoint: LatLng = {
        let mut init = LatLng {
            lat: 0.001f64,
            lng: -3.14159265358979323846f64 + 0.001f64,
        };
        init
    };
    let mut eastPointOutside: LatLng = {
        let mut init = LatLng {
            lat: 0.001f64,
            lng: -3.14159265358979323846f64 + 0.1f64,
        };
        init
    };
    let mut westPoint: LatLng = {
        let mut init = LatLng {
            lat: 0.001f64,
            lng: 3.14159265358979323846f64 - 0.001f64,
        };
        init
    };
    let mut westPointOutside: LatLng = {
        let mut init = LatLng {
            lat: 0.001f64,
            lng: 3.14159265358979323846f64 - 0.1f64,
        };
        init
    };
    let mut bbox_3: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromGeoLoop(&mut transMeridianGeoLoop, &mut bbox_3);
    if !pointInsideGeoLoop(&mut transMeridianGeoLoop, &mut bbox_3, &mut westPoint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            b"pointInsideGeoLoop(&transMeridianGeoLoop, &bbox, &westPoint)\0" as *const u8
                as *const libc::c_char,
            b"contains point to the west of the antimeridian\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !pointInsideGeoLoop(&mut transMeridianGeoLoop, &mut bbox_3, &mut eastPoint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            b"pointInsideGeoLoop(&transMeridianGeoLoop, &bbox, &eastPoint)\0" as *const u8
                as *const libc::c_char,
            b"contains point to the east of the antimeridian\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if pointInsideGeoLoop(
        &mut transMeridianGeoLoop,
        &mut bbox_3,
        &mut westPointOutside,
    ) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            b"!pointInsideGeoLoop(&transMeridianGeoLoop, &bbox, &westPointOutside)\0" as *const u8
                as *const libc::c_char,
            b"does not contain outside point to the west of the antimeridian\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if pointInsideGeoLoop(
        &mut transMeridianGeoLoop,
        &mut bbox_3,
        &mut eastPointOutside,
    ) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            b"!pointInsideGeoLoop(&transMeridianGeoLoop, &bbox, &eastPointOutside)\0" as *const u8
                as *const libc::c_char,
            b"does not contain outside point to the east of the antimeridian\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"pointInsideLinkedGeoLoop\0" as *const u8 as *const libc::c_char;
    let mut somewhere_0: LatLng = {
        let mut init = LatLng {
            lat: 1 as libc::c_int as libc::c_double,
            lng: 2 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut inside_0: LatLng = {
        let mut init = LatLng {
            lat: 0.659f64,
            lng: -2.136f64,
        };
        init
    };
    let mut loop_0: LinkedGeoLoop = LinkedGeoLoop {
        first: 0 as *mut LinkedLatLng,
        last: 0 as *mut LinkedLatLng,
        next: 0 as *mut LinkedGeoLoop,
    };
    createLinkedLoop(
        &mut loop_0,
        &mut *sfVerts.as_mut_ptr().offset(0 as libc::c_int as isize),
        6 as libc::c_int,
    );
    let mut bbox_4: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromLinkedGeoLoop(&mut loop_0, &mut bbox_4);
    if !pointInsideLinkedGeoLoop(&mut loop_0, &mut bbox_4, &mut inside_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            b"pointInsideLinkedGeoLoop(&loop, &bbox, &inside)\0" as *const u8
                as *const libc::c_char,
            b"contains exact4\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if pointInsideLinkedGeoLoop(&mut loop_0, &mut bbox_4, &mut somewhere_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            171 as libc::c_int,
            b"!pointInsideLinkedGeoLoop(&loop, &bbox, &somewhere)\0" as *const u8
                as *const libc::c_char,
            b"contains somewhere else\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedGeoLoop(&mut loop_0);
    currentTestName = b"bboxFromGeoLoop\0" as *const u8 as *const libc::c_char;
    let mut verts_3: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.8f64,
                lng: 0.3f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.7f64,
                lng: 0.6f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.1f64,
                lng: 0.7f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.0f64,
                lng: 0.2f64,
            };
            init
        },
    ];
    let mut geoloop_3: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_3.as_mut_ptr(),
        };
        init
    };
    let expected: BBox = {
        let mut init = BBox {
            north: 1.1f64,
            south: 0.7f64,
            east: 0.7f64,
            west: 0.2f64,
        };
        init
    };
    let mut result: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromGeoLoop(&mut geoloop_3, &mut result);
    if !bboxEquals(&mut result, &expected) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            b"bboxEquals(&result, &expected)\0" as *const u8 as *const libc::c_char,
            b"Got expected bbox\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"bboxFromGeoLoopTransmeridian\0" as *const u8 as *const libc::c_char;
    let mut verts_4: [LatLng; 6] = [
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: -3.14159265358979323846f64 + 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 3.14159265358979323846f64 - 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.05f64,
                lng: 3.14159265358979323846f64 - 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: 3.14159265358979323846f64 - 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: -3.14159265358979323846f64 + 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.05f64,
                lng: -3.14159265358979323846f64 + 0.2f64,
            };
            init
        },
    ];
    let mut geoloop_4: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 6 as libc::c_int,
            verts: verts_4.as_mut_ptr(),
        };
        init
    };
    let expected_0: BBox = {
        let mut init = BBox {
            north: 0.1f64,
            south: -0.1f64,
            east: -3.14159265358979323846f64 + 0.2f64,
            west: 3.14159265358979323846f64 - 0.2f64,
        };
        init
    };
    let mut result_0: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromGeoLoop(&mut geoloop_4, &mut result_0);
    if !bboxEquals(&mut result_0, &expected_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
            b"bboxEquals(&result, &expected)\0" as *const u8 as *const libc::c_char,
            b"Got expected transmeridian bbox\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"bboxFromGeoLoopNoVertices\0" as *const u8 as *const libc::c_char;
    let mut geoloop_5: GeoLoop = GeoLoop {
        numVerts: 0,
        verts: 0 as *mut LatLng,
    };
    geoloop_5.verts = 0 as *mut LatLng;
    geoloop_5.numVerts = 0 as libc::c_int;
    let expected_1: BBox = {
        let mut init = BBox {
            north: 0.0f64,
            south: 0.0f64,
            east: 0.0f64,
            west: 0.0f64,
        };
        init
    };
    let mut result_1: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromGeoLoop(&mut geoloop_5, &mut result_1);
    if !bboxEquals(&mut result_1, &expected_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            b"bboxEquals(&result, &expected)\0" as *const u8 as *const libc::c_char,
            b"Got expected bbox\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"bboxesFromGeoPolygon\0" as *const u8 as *const libc::c_char;
    let mut verts_5: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.8f64,
                lng: 0.3f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.7f64,
                lng: 0.6f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.1f64,
                lng: 0.7f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.0f64,
                lng: 0.2f64,
            };
            init
        },
    ];
    let mut geoloop_6: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_5.as_mut_ptr(),
        };
        init
    };
    let mut polygon: GeoPolygon = {
        let mut init = GeoPolygon {
            geoloop: geoloop_6,
            numHoles: 0 as libc::c_int,
            holes: 0 as *mut GeoLoop,
        };
        init
    };
    let expected_2: BBox = {
        let mut init = BBox {
            north: 1.1f64,
            south: 0.7f64,
            east: 0.7f64,
            west: 0.2f64,
        };
        init
    };
    let mut result_2: *mut BBox = calloc(
        ::core::mem::size_of::<BBox>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut BBox;
    bboxesFromGeoPolygon(&mut polygon, result_2);
    if !bboxEquals(
        &mut *result_2.offset(0 as libc::c_int as isize),
        &expected_2,
    ) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            b"bboxEquals(&result[0], &expected)\0" as *const u8 as *const libc::c_char,
            b"Got expected bbox\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(result_2 as *mut libc::c_void);
    currentTestName = b"bboxesFromGeoPolygonHole\0" as *const u8 as *const libc::c_char;
    let mut verts_6: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.8f64,
                lng: 0.3f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.7f64,
                lng: 0.6f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.1f64,
                lng: 0.7f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.0f64,
                lng: 0.2f64,
            };
            init
        },
    ];
    let mut geoloop_7: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_6.as_mut_ptr(),
        };
        init
    };
    let mut holeVerts: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.9f64,
                lng: 0.3f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.9f64,
                lng: 0.5f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.0f64,
                lng: 0.7f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.9f64,
                lng: 0.3f64,
            };
            init
        },
    ];
    let mut holeGeoLoop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: holeVerts.as_mut_ptr(),
        };
        init
    };
    let mut polygon_0: GeoPolygon = {
        let mut init = GeoPolygon {
            geoloop: geoloop_7,
            numHoles: 1 as libc::c_int,
            holes: &mut holeGeoLoop,
        };
        init
    };
    let expected_3: BBox = {
        let mut init = BBox {
            north: 1.1f64,
            south: 0.7f64,
            east: 0.7f64,
            west: 0.2f64,
        };
        init
    };
    let expectedHole: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.9f64,
            east: 0.7f64,
            west: 0.3f64,
        };
        init
    };
    let mut result_3: *mut BBox = calloc(
        ::core::mem::size_of::<BBox>() as libc::c_ulong,
        2 as libc::c_int as libc::c_ulong,
    ) as *mut BBox;
    bboxesFromGeoPolygon(&mut polygon_0, result_3);
    if !bboxEquals(
        &mut *result_3.offset(0 as libc::c_int as isize),
        &expected_3,
    ) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int,
            b"bboxEquals(&result[0], &expected)\0" as *const u8 as *const libc::c_char,
            b"Got expected bbox\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !bboxEquals(
        &mut *result_3.offset(1 as libc::c_int as isize),
        &expectedHole,
    ) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int,
            b"bboxEquals(&result[1], &expectedHole)\0" as *const u8 as *const libc::c_char,
            b"Got expected hole bbox\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(result_3 as *mut libc::c_void);
    currentTestName = b"bboxFromLinkedGeoLoop\0" as *const u8 as *const libc::c_char;
    let mut verts_7: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.8f64,
                lng: 0.3f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.7f64,
                lng: 0.6f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.1f64,
                lng: 0.7f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.0f64,
                lng: 0.2f64,
            };
            init
        },
    ];
    let mut loop_1: LinkedGeoLoop = LinkedGeoLoop {
        first: 0 as *mut LinkedLatLng,
        last: 0 as *mut LinkedLatLng,
        next: 0 as *mut LinkedGeoLoop,
    };
    createLinkedLoop(
        &mut loop_1,
        &mut *verts_7.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let expected_4: BBox = {
        let mut init = BBox {
            north: 1.1f64,
            south: 0.7f64,
            east: 0.7f64,
            west: 0.2f64,
        };
        init
    };
    let mut result_4: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromLinkedGeoLoop(&mut loop_1, &mut result_4);
    if !bboxEquals(&mut result_4, &expected_4) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            261 as libc::c_int,
            b"bboxEquals(&result, &expected)\0" as *const u8 as *const libc::c_char,
            b"Got expected bbox\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedGeoLoop(&mut loop_1);
    currentTestName = b"bboxFromLinkedGeoLoopNoVertices\0" as *const u8 as *const libc::c_char;
    let mut loop_2: LinkedGeoLoop = {
        let mut init = LinkedGeoLoop {
            first: 0 as *mut LinkedLatLng,
            last: 0 as *mut LinkedLatLng,
            next: 0 as *mut LinkedGeoLoop,
        };
        init
    };
    let expected_5: BBox = {
        let mut init = BBox {
            north: 0.0f64,
            south: 0.0f64,
            east: 0.0f64,
            west: 0.0f64,
        };
        init
    };
    let mut result_5: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromLinkedGeoLoop(&mut loop_2, &mut result_5);
    if !bboxEquals(&mut result_5, &expected_5) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
            b"bboxEquals(&result, &expected)\0" as *const u8 as *const libc::c_char,
            b"Got expected bbox\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedGeoLoop(&mut loop_2);
    currentTestName = b"isClockwiseGeoLoop\0" as *const u8 as *const libc::c_char;
    let mut verts_8: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0.1f64,
            };
            init
        },
    ];
    let mut geoloop_8: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 3 as libc::c_int,
            verts: verts_8.as_mut_ptr(),
        };
        init
    };
    if !isClockwiseGeoLoop(&mut geoloop_8) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            284 as libc::c_int,
            b"isClockwiseGeoLoop(&geoloop)\0" as *const u8 as *const libc::c_char,
            b"Got true for clockwise geoloop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"isClockwiseLinkedGeoLoop\0" as *const u8 as *const libc::c_char;
    let mut verts_9: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.2f64,
                lng: 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 0.2f64,
            };
            init
        },
    ];
    let mut loop_3: LinkedGeoLoop = LinkedGeoLoop {
        first: 0 as *mut LinkedLatLng,
        last: 0 as *mut LinkedLatLng,
        next: 0 as *mut LinkedGeoLoop,
    };
    createLinkedLoop(
        &mut loop_3,
        &mut *verts_9.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    if !isClockwiseLinkedGeoLoop(&mut loop_3) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int,
            b"isClockwiseLinkedGeoLoop(&loop)\0" as *const u8 as *const libc::c_char,
            b"Got true for clockwise loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedGeoLoop(&mut loop_3);
    currentTestName = b"isNotClockwiseLinkedGeoLoop\0" as *const u8 as *const libc::c_char;
    let mut verts_10: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0.4f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: 0.4f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut loop_4: LinkedGeoLoop = LinkedGeoLoop {
        first: 0 as *mut LinkedLatLng,
        last: 0 as *mut LinkedLatLng,
        next: 0 as *mut LinkedGeoLoop,
    };
    createLinkedLoop(
        &mut loop_4,
        &mut *verts_10.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    if isClockwiseLinkedGeoLoop(&mut loop_4) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            b"!isClockwiseLinkedGeoLoop(&loop)\0" as *const u8 as *const libc::c_char,
            b"Got false for counter-clockwise loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedGeoLoop(&mut loop_4);
    currentTestName = b"isClockwiseGeoLoopTransmeridian\0" as *const u8 as *const libc::c_char;
    let mut verts_11: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: 3.14159265358979323846f64 - 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: -3.14159265358979323846f64 + 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: -3.14159265358979323846f64 + 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: 3.14159265358979323846f64 - 0.1f64,
            };
            init
        },
    ];
    let mut geoloop_9: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_11.as_mut_ptr(),
        };
        init
    };
    if !isClockwiseGeoLoop(&mut geoloop_9) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            b"isClockwiseGeoLoop(&geoloop)\0" as *const u8 as *const libc::c_char,
            b"Got true for clockwise geoloop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName =
        b"isClockwiseLinkedGeoLoopTransmeridian\0" as *const u8 as *const libc::c_char;
    let mut verts_12: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: 3.14159265358979323846f64 - 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: -3.14159265358979323846f64 + 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: -3.14159265358979323846f64 + 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: 3.14159265358979323846f64 - 0.1f64,
            };
            init
        },
    ];
    let mut loop_5: LinkedGeoLoop = LinkedGeoLoop {
        first: 0 as *mut LinkedLatLng,
        last: 0 as *mut LinkedLatLng,
        next: 0 as *mut LinkedGeoLoop,
    };
    createLinkedLoop(
        &mut loop_5,
        &mut *verts_12.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    if !isClockwiseLinkedGeoLoop(&mut loop_5) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int,
            b"isClockwiseLinkedGeoLoop(&loop)\0" as *const u8 as *const libc::c_char,
            b"Got true for clockwise transmeridian loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedGeoLoop(&mut loop_5);
    currentTestName =
        b"isNotClockwiseLinkedGeoLoopTransmeridian\0" as *const u8 as *const libc::c_char;
    let mut verts_13: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: 3.14159265358979323846f64 - 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: 3.14159265358979323846f64 - 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: -3.14159265358979323846f64 + 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: -3.14159265358979323846f64 + 0.1f64,
            };
            init
        },
    ];
    let mut loop_6: LinkedGeoLoop = LinkedGeoLoop {
        first: 0 as *mut LinkedLatLng,
        last: 0 as *mut LinkedLatLng,
        next: 0 as *mut LinkedGeoLoop,
    };
    createLinkedLoop(
        &mut loop_6,
        &mut *verts_13.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    if isClockwiseLinkedGeoLoop(&mut loop_6) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            343 as libc::c_int,
            b"!isClockwiseLinkedGeoLoop(&loop)\0" as *const u8 as *const libc::c_char,
            b"Got false for counter-clockwise transmeridian loop\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedGeoLoop(&mut loop_6);
    currentTestName = b"normalizeMultiPolygonSingle\0" as *const u8 as *const libc::c_char;
    let mut verts_14: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            352 as libc::c_int,
            b"outer != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer,
        &mut *verts_14.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut polygon_1: LinkedGeoPolygon = {
        let mut init = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        init
    };
    addLinkedLoop(&mut polygon_1, outer);
    if normalizeMultiPolygon(&mut polygon_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            358 as libc::c_int,
            b"!(normalizeMultiPolygon(&polygon))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_1) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_1) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            361 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"Loop count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(polygon_1.first == outer) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            362 as libc::c_int,
            b"polygon.first == outer\0" as *const u8 as *const libc::c_char,
            b"Got expected loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_1);
    currentTestName = b"normalizeMultiPolygonTwoOuterLoops\0" as *const u8 as *const libc::c_char;
    let mut verts1: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer1: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer1.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            371 as libc::c_int,
            b"outer1 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer1,
        &mut *verts1.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut verts2: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 3 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer2: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer2.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            377 as libc::c_int,
            b"outer2 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer2,
        &mut *verts2.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut polygon_2: LinkedGeoPolygon = {
        let mut init = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        init
    };
    addLinkedLoop(&mut polygon_2, outer1);
    addLinkedLoop(&mut polygon_2, outer2);
    if normalizeMultiPolygon(&mut polygon_2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            b"!(normalizeMultiPolygon(&polygon))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_2) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            386 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_2) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            388 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"Loop count on first polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(polygon_2.next) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            390 as libc::c_int,
            b"countLinkedLoops(polygon.next) == 1\0" as *const u8 as *const libc::c_char,
            b"Loop count on second polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_2);
    currentTestName = b"normalizeMultiPolygonOneHole\0" as *const u8 as *const libc::c_char;
    let mut verts_15: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 3 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 3 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer_0: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer_0.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            399 as libc::c_int,
            b"outer != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer_0,
        &mut *verts_15.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let mut verts2_0: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 2 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut inner: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if inner.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            405 as libc::c_int,
            b"inner != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        inner,
        &mut *verts2_0.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut polygon_3: LinkedGeoPolygon = {
        let mut init = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        init
    };
    addLinkedLoop(&mut polygon_3, inner);
    addLinkedLoop(&mut polygon_3, outer_0);
    if normalizeMultiPolygon(&mut polygon_3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            412 as libc::c_int,
            b"!(normalizeMultiPolygon(&polygon))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_3) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            414 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_3) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            416 as libc::c_int,
            b"countLinkedLoops(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count on first polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(polygon_3.first == outer_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int,
            b"polygon.first == outer\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !((*polygon_3.first).next == inner) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            418 as libc::c_int,
            b"polygon.first->next == inner\0" as *const u8 as *const libc::c_char,
            b"Got expected inner loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_3);
    currentTestName = b"normalizeMultiPolygonTwoHoles\0" as *const u8 as *const libc::c_char;
    let mut verts_16: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0.4f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: 0.4f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer_1: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer_1.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            427 as libc::c_int,
            b"outer != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer_1,
        &mut *verts_16.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let mut verts2_1: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.2f64,
                lng: 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 0.2f64,
            };
            init
        },
    ];
    let mut inner1: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if inner1.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            433 as libc::c_int,
            b"inner1 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        inner1,
        &mut *verts2_1.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut verts3: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 0.2f64,
                lng: 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.3f64,
                lng: 0.3f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.2f64,
                lng: 0.3f64,
            };
            init
        },
    ];
    let mut inner2: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if inner2.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int,
            b"inner2 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        inner2,
        &mut *verts3.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut polygon_4: LinkedGeoPolygon = {
        let mut init = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        init
    };
    addLinkedLoop(&mut polygon_4, inner2);
    addLinkedLoop(&mut polygon_4, outer_1);
    addLinkedLoop(&mut polygon_4, inner1);
    if normalizeMultiPolygon(&mut polygon_4) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            447 as libc::c_int,
            b"!(normalizeMultiPolygon(&polygon))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_4) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            450 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct for 2 holes\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(polygon_4.first == outer_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            451 as libc::c_int,
            b"polygon.first == outer\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_4) == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            453 as libc::c_int,
            b"countLinkedLoops(&polygon) == 3\0" as *const u8 as *const libc::c_char,
            b"Loop count on first polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_4);
    currentTestName = b"normalizeMultiPolygonTwoDonuts\0" as *const u8 as *const libc::c_char;
    let mut verts_17: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 3 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 3 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer_2: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer_2.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int,
            b"outer != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer_2,
        &mut *verts_17.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let mut verts2_2: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 2 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut inner_0: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if inner_0.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            466 as libc::c_int,
            b"inner != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        inner_0,
        &mut *verts2_2.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut verts3_0: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: -(3 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -(3 as libc::c_int) as libc::c_double,
                lng: -(3 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -(3 as libc::c_int) as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer2_0: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer2_0.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            471 as libc::c_int,
            b"outer2 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer2_0,
        &mut *verts3_0.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let mut verts4: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: -(1 as libc::c_int) as libc::c_double,
                lng: -(1 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -(2 as libc::c_int) as libc::c_double,
                lng: -(2 as libc::c_int) as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -(1 as libc::c_int) as libc::c_double,
                lng: -(2 as libc::c_int) as libc::c_double,
            };
            init
        },
    ];
    let mut inner2_0: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if inner2_0.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            476 as libc::c_int,
            b"inner2 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        inner2_0,
        &mut *verts4.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut polygon_5: LinkedGeoPolygon = {
        let mut init = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        init
    };
    addLinkedLoop(&mut polygon_5, inner2_0);
    addLinkedLoop(&mut polygon_5, inner_0);
    addLinkedLoop(&mut polygon_5, outer_2);
    addLinkedLoop(&mut polygon_5, outer2_0);
    if normalizeMultiPolygon(&mut polygon_5) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            485 as libc::c_int,
            b"!(normalizeMultiPolygon(&polygon))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_5) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            487 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_5) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int,
            b"countLinkedLoops(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count on first polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords(polygon_5.first) == 4 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            491 as libc::c_int,
            b"countLinkedCoords(polygon.first) == 4\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*polygon_5.first).next) == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            493 as libc::c_int,
            b"countLinkedCoords(polygon.first->next) == 3\0" as *const u8 as *const libc::c_char,
            b"Got expected inner loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(polygon_5.next) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            495 as libc::c_int,
            b"countLinkedLoops(polygon.next) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count on second polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*polygon_5.next).first) == 4 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            497 as libc::c_int,
            b"countLinkedCoords(polygon.next->first) == 4\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedCoords((*(*polygon_5.next).first).next) == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            499 as libc::c_int,
            b"countLinkedCoords(polygon.next->first->next) == 3\0" as *const u8
                as *const libc::c_char,
            b"Got expected inner loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_5);
    currentTestName = b"normalizeMultiPolygonNestedDonuts\0" as *const u8 as *const libc::c_char;
    let mut verts_18: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.2f64,
                lng: 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.2f64,
                lng: -0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.2f64,
                lng: -0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.2f64,
                lng: 0.2f64,
            };
            init
        },
    ];
    let mut outer_3: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer_3.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            507 as libc::c_int,
            b"outer != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer_3,
        &mut *verts_18.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let mut verts2_3: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: -0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: -0.1f64,
            };
            init
        },
    ];
    let mut inner_1: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if inner_1.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            512 as libc::c_int,
            b"inner != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        inner_1,
        &mut *verts2_3.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let mut verts3_1: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.6f64,
                lng: 0.6f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.6f64,
                lng: -0.6f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.6f64,
                lng: -0.6f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.6f64,
                lng: 0.6f64,
            };
            init
        },
    ];
    let mut outerBig: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outerBig.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            517 as libc::c_int,
            b"outerBig != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outerBig,
        &mut *verts3_1.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let mut verts4_0: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.5f64,
                lng: 0.5f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.5f64,
                lng: 0.5f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.5f64,
                lng: -0.5f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.5f64,
                lng: -0.5f64,
            };
            init
        },
    ];
    let mut innerBig: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if innerBig.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            522 as libc::c_int,
            b"innerBig != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        innerBig,
        &mut *verts4_0.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let mut polygon_6: LinkedGeoPolygon = {
        let mut init = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        init
    };
    addLinkedLoop(&mut polygon_6, inner_1);
    addLinkedLoop(&mut polygon_6, outerBig);
    addLinkedLoop(&mut polygon_6, innerBig);
    addLinkedLoop(&mut polygon_6, outer_3);
    if normalizeMultiPolygon(&mut polygon_6) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            531 as libc::c_int,
            b"!(normalizeMultiPolygon(&polygon))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_6) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            533 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_6) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            535 as libc::c_int,
            b"countLinkedLoops(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count on first polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(polygon_6.first == outerBig) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            536 as libc::c_int,
            b"polygon.first == outerBig\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !((*polygon_6.first).next == innerBig) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            537 as libc::c_int,
            b"polygon.first->next == innerBig\0" as *const u8 as *const libc::c_char,
            b"Got expected inner loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(polygon_6.next) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            539 as libc::c_int,
            b"countLinkedLoops(polygon.next) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count on second polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !((*polygon_6.next).first == outer_3) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            540 as libc::c_int,
            b"polygon.next->first == outer\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !((*(*polygon_6.next).first).next == inner_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            541 as libc::c_int,
            b"polygon.next->first->next == inner\0" as *const u8 as *const libc::c_char,
            b"Got expected inner loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_6);
    currentTestName = b"normalizeMultiPolygonNoOuterLoops\0" as *const u8 as *const libc::c_char;
    let mut verts1_0: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer1_0: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer1_0.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int,
            b"outer1 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer1_0,
        &mut *verts1_0.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut verts2_4: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 3 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer2_1: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer2_1.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            556 as libc::c_int,
            b"outer2 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer2_1,
        &mut *verts2_4.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut polygon_7: LinkedGeoPolygon = {
        let mut init = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        init
    };
    addLinkedLoop(&mut polygon_7, outer1_0);
    addLinkedLoop(&mut polygon_7, outer2_1);
    if !(normalizeMultiPolygon(&mut polygon_7) == E_FAILED as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            564 as libc::c_int,
            b"normalizeMultiPolygon(&polygon) == E_FAILED\0" as *const u8 as *const libc::c_char,
            b"Expected error code returned\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_7) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            566 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_7) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            568 as libc::c_int,
            b"countLinkedLoops(&polygon) == 0\0" as *const u8 as *const libc::c_char,
            b"Loop count as expected with invalid input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_7);
    currentTestName =
        b"normalizeMultiPolygonAlreadyNormalized\0" as *const u8 as *const libc::c_char;
    let mut verts1_1: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer1_1: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer1_1.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            577 as libc::c_int,
            b"outer1 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer1_1,
        &mut *verts1_1.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut verts2_5: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 3 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer2_2: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer2_2.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            583 as libc::c_int,
            b"outer2 != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer2_2,
        &mut *verts2_5.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut polygon_8: LinkedGeoPolygon = {
        let mut init = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        init
    };
    addLinkedLoop(&mut polygon_8, outer1_1);
    let mut next: *mut LinkedGeoPolygon = addNewLinkedPolygon(&mut polygon_8);
    addLinkedLoop(next, outer2_2);
    if !(normalizeMultiPolygon(&mut polygon_8) == E_FAILED as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            593 as libc::c_int,
            b"normalizeMultiPolygon(&polygon) == E_FAILED\0" as *const u8 as *const libc::c_char,
            b"Expected error code returned\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedPolygons(&mut polygon_8) == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            595 as libc::c_int,
            b"countLinkedPolygons(&polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(&mut polygon_8) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            597 as libc::c_int,
            b"countLinkedLoops(&polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"Loop count on first polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(polygon_8.first == outer1_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            598 as libc::c_int,
            b"polygon.first == outer1\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(countLinkedLoops(polygon_8.next) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            600 as libc::c_int,
            b"countLinkedLoops(polygon.next) == 1\0" as *const u8 as *const libc::c_char,
            b"Loop count on second polygon correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !((*polygon_8.next).first == outer2_2) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            601 as libc::c_int,
            b"polygon.next->first == outer2\0" as *const u8 as *const libc::c_char,
            b"Got expected outer loop\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_8);
    currentTestName = b"normalizeMultiPolygon_unassignedHole\0" as *const u8 as *const libc::c_char;
    let mut verts_19: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 1 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut outer_4: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if outer_4.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            610 as libc::c_int,
            b"outer != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        outer_4,
        &mut *verts_19.as_mut_ptr().offset(0 as libc::c_int as isize),
        4 as libc::c_int,
    );
    let mut verts2_6: [LatLng; 3] = [
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 2 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 3 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 2 as libc::c_int as libc::c_double,
                lng: 3 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut inner_2: *mut LinkedGeoLoop =
        malloc(::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong) as *mut LinkedGeoLoop;
    if inner_2.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygon.c\0" as *const u8 as *const libc::c_char,
            616 as libc::c_int,
            b"inner != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    createLinkedLoop(
        inner_2,
        &mut *verts2_6.as_mut_ptr().offset(0 as libc::c_int as isize),
        3 as libc::c_int,
    );
    let mut polygon_9: LinkedGeoPolygon = {
        let mut init = LinkedGeoPolygon {
            first: 0 as *mut LinkedGeoLoop,
            last: 0 as *mut LinkedGeoLoop,
            next: 0 as *mut LinkedGeoPolygon,
        };
        init
    };
    addLinkedLoop(&mut polygon_9, inner_2);
    addLinkedLoop(&mut polygon_9, outer_4);
    if !(normalizeMultiPolygon(&mut polygon_9) == E_FAILED as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygon.c\0" as *const u8 as *const libc::c_char,
            624 as libc::c_int,
            b"normalizeMultiPolygon(&polygon) == E_FAILED\0" as *const u8 as *const libc::c_char,
            b"Expected error code returned\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(&mut polygon_9);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"polygon\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"polygon\0" as *const u8 as *const libc::c_char,
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
