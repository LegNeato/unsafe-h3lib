extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn lineHexEstimate(
        origin: *const LatLng,
        destination: *const LatLng,
        res: libc::c_int,
        out: *mut int64_t,
    ) -> H3Error;
    fn bboxHexEstimate(bbox: *const BBox, res: libc::c_int, out: *mut int64_t) -> H3Error;
    fn bboxEquals(b1: *const BBox, b2: *const BBox) -> bool;
    fn bboxContains(bbox: *const BBox, point: *const LatLng) -> bool;
    fn bboxCenter(bbox: *const BBox, center: *mut LatLng);
    fn bboxIsTransmeridian(bbox: *const BBox) -> bool;
    fn geoAlmostEqual(p1: *const LatLng, p2: *const LatLng) -> bool;
    fn bboxFromGeoLoop(loop_0: *const GeoLoop, bbox: *mut BBox);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type int64_t = libc::c_longlong;
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
pub struct BBox {
    pub north: libc::c_double,
    pub south: libc::c_double,
    pub east: libc::c_double,
    pub west: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn assertBBox(
    mut geoloop: *const GeoLoop,
    mut expected: *const BBox,
    mut inside: *const LatLng,
    mut outside: *const LatLng,
) {
    let mut result: BBox = BBox {
        north: 0.,
        south: 0.,
        east: 0.,
        west: 0.,
    };
    bboxFromGeoLoop(geoloop, &mut result);
    if !bboxEquals(&mut result, expected) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            b"bboxEquals(&result, expected)\0" as *const u8 as *const libc::c_char,
            b"Got expected bbox\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !bboxContains(&mut result, inside) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            b"bboxContains(&result, inside)\0" as *const u8 as *const libc::c_char,
            b"Contains expected inside point\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if bboxContains(&mut result, outside) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            b"!bboxContains(&result, outside)\0" as *const u8 as *const libc::c_char,
            b"Does not contain expected outside point\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn runTests() {
    currentTestName = b"posLatPosLng\0" as *const u8 as *const libc::c_char;
    let mut verts: [LatLng; 4] = [
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
    let geoloop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts.as_mut_ptr(),
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
    let inside: LatLng = {
        let mut init = LatLng {
            lat: 0.9f64,
            lng: 0.4f64,
        };
        init
    };
    let outside: LatLng = {
        let mut init = LatLng {
            lat: 0.0f64,
            lng: 0.0f64,
        };
        init
    };
    assertBBox(&geoloop, &expected, &inside, &outside);
    currentTestName = b"negLatPosLng\0" as *const u8 as *const libc::c_char;
    let mut verts_0: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: -0.3f64,
                lng: 0.6f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: 0.9f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.2f64,
                lng: 0.8f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: 0.6f64,
            };
            init
        },
    ];
    let geoloop_0: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_0.as_mut_ptr(),
        };
        init
    };
    let expected_0: BBox = {
        let mut init = BBox {
            north: -0.1f64,
            south: -0.4f64,
            east: 0.9f64,
            west: 0.6f64,
        };
        init
    };
    let inside_0: LatLng = {
        let mut init = LatLng {
            lat: -0.3f64,
            lng: 0.8f64,
        };
        init
    };
    let outside_0: LatLng = {
        let mut init = LatLng {
            lat: 0.0f64,
            lng: 0.0f64,
        };
        init
    };
    assertBBox(&geoloop_0, &expected_0, &inside_0, &outside_0);
    currentTestName = b"posLatNegLng\0" as *const u8 as *const libc::c_char;
    let mut verts_1: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.7f64,
                lng: -1.4f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.8f64,
                lng: -0.9f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.0f64,
                lng: -0.8f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.1f64,
                lng: -1.3f64,
            };
            init
        },
    ];
    let geoloop_1: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_1.as_mut_ptr(),
        };
        init
    };
    let expected_1: BBox = {
        let mut init = BBox {
            north: 1.1f64,
            south: 0.7f64,
            east: -0.8f64,
            west: -1.4f64,
        };
        init
    };
    let inside_1: LatLng = {
        let mut init = LatLng {
            lat: 0.9f64,
            lng: -1.0f64,
        };
        init
    };
    let outside_1: LatLng = {
        let mut init = LatLng {
            lat: 0.0f64,
            lng: 0.0f64,
        };
        init
    };
    assertBBox(&geoloop_1, &expected_1, &inside_1, &outside_1);
    currentTestName = b"negLatNegLng\0" as *const u8 as *const libc::c_char;
    let mut verts_2: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: -1.4f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.3f64,
                lng: -1.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: -1.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.2f64,
                lng: -1.4f64,
            };
            init
        },
    ];
    let geoloop_2: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_2.as_mut_ptr(),
        };
        init
    };
    let expected_2: BBox = {
        let mut init = BBox {
            north: -0.1f64,
            south: -0.4f64,
            east: -1.1f64,
            west: -1.4f64,
        };
        init
    };
    let inside_2: LatLng = {
        let mut init = LatLng {
            lat: -0.3f64,
            lng: -1.2f64,
        };
        init
    };
    let outside_2: LatLng = {
        let mut init = LatLng {
            lat: 0.0f64,
            lng: 0.0f64,
        };
        init
    };
    assertBBox(&geoloop_2, &expected_2, &inside_2, &outside_2);
    currentTestName = b"aroundZeroZero\0" as *const u8 as *const libc::c_char;
    let mut verts_3: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: -0.4f64,
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
                lat: -0.4f64,
                lng: 0.4f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: -0.4f64,
            };
            init
        },
    ];
    let geoloop_3: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_3.as_mut_ptr(),
        };
        init
    };
    let expected_3: BBox = {
        let mut init = BBox {
            north: 0.4f64,
            south: -0.4f64,
            east: 0.4f64,
            west: -0.4f64,
        };
        init
    };
    let inside_3: LatLng = {
        let mut init = LatLng {
            lat: -0.1f64,
            lng: -0.1f64,
        };
        init
    };
    let outside_3: LatLng = {
        let mut init = LatLng {
            lat: 1.0f64,
            lng: -1.0f64,
        };
        init
    };
    assertBBox(&geoloop_3, &expected_3, &inside_3, &outside_3);
    currentTestName = b"transmeridian\0" as *const u8 as *const libc::c_char;
    let mut verts_4: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.4f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.4f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.1f64,
            };
            init
        },
    ];
    let geoloop_4: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_4.as_mut_ptr(),
        };
        init
    };
    let expected_4: BBox = {
        let mut init = BBox {
            north: 0.4f64,
            south: -0.4f64,
            east: -3.14159265358979323846264338327950288f64 + 0.1f64,
            west: 3.14159265358979323846264338327950288f64 - 0.1f64,
        };
        init
    };
    let insideOnMeridian: LatLng = {
        let mut init = LatLng {
            lat: -0.1f64,
            lng: 3.14159265358979323846264338327950288f64,
        };
        init
    };
    let outside_4: LatLng = {
        let mut init = LatLng {
            lat: 1.0f64,
            lng: 3.14159265358979323846264338327950288f64 - 0.5f64,
        };
        init
    };
    assertBBox(&geoloop_4, &expected_4, &insideOnMeridian, &outside_4);
    let westInside: LatLng = {
        let mut init = LatLng {
            lat: 0.1f64,
            lng: 3.14159265358979323846264338327950288f64 - 0.05f64,
        };
        init
    };
    if !bboxContains(&expected_4, &westInside) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            b"bboxContains(&expected, &westInside)\0" as *const u8 as *const libc::c_char,
            b"Contains expected west inside point\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let eastInside: LatLng = {
        let mut init = LatLng {
            lat: 0.1f64,
            lng: -3.14159265358979323846264338327950288f64 + 0.05f64,
        };
        init
    };
    if !bboxContains(&expected_4, &eastInside) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
            b"bboxContains(&expected, &eastInside)\0" as *const u8 as *const libc::c_char,
            b"Contains expected east outside point\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let westOutside: LatLng = {
        let mut init = LatLng {
            lat: 0.1f64,
            lng: 3.14159265358979323846264338327950288f64 - 0.5f64,
        };
        init
    };
    if bboxContains(&expected_4, &westOutside) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            b"!bboxContains(&expected, &westOutside)\0" as *const u8 as *const libc::c_char,
            b"Does not contain expected west outside point\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let eastOutside: LatLng = {
        let mut init = LatLng {
            lat: 0.1f64,
            lng: -3.14159265358979323846264338327950288f64 + 0.5f64,
        };
        init
    };
    if bboxContains(&expected_4, &eastOutside) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            b"!bboxContains(&expected, &eastOutside)\0" as *const u8 as *const libc::c_char,
            b"Does not contain expected east outside point\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"edgeOnNorthPole\0" as *const u8 as *const libc::c_char;
    let mut verts_5: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 1.57079632679489661923132169163975144f64 - 0.1f64,
                lng: 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.57079632679489661923132169163975144f64 - 0.1f64,
                lng: 0.8f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.57079632679489661923132169163975144f64,
                lng: 0.8f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.57079632679489661923132169163975144f64,
                lng: 0.1f64,
            };
            init
        },
    ];
    let geoloop_5: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_5.as_mut_ptr(),
        };
        init
    };
    let expected_5: BBox = {
        let mut init = BBox {
            north: 1.57079632679489661923132169163975144f64,
            south: 1.57079632679489661923132169163975144f64 - 0.1f64,
            east: 0.8f64,
            west: 0.1f64,
        };
        init
    };
    let inside_4: LatLng = {
        let mut init = LatLng {
            lat: 1.57079632679489661923132169163975144f64 - 0.01f64,
            lng: 0.4f64,
        };
        init
    };
    let outside_5: LatLng = {
        let mut init = LatLng {
            lat: 1.57079632679489661923132169163975144f64,
            lng: 0.9f64,
        };
        init
    };
    assertBBox(&geoloop_5, &expected_5, &inside_4, &outside_5);
    currentTestName = b"edgeOnSouthPole\0" as *const u8 as *const libc::c_char;
    let mut verts_6: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: -1.57079632679489661923132169163975144f64 + 0.1f64,
                lng: 0.1f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -1.57079632679489661923132169163975144f64 + 0.1f64,
                lng: 0.8f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -1.57079632679489661923132169163975144f64,
                lng: 0.8f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -1.57079632679489661923132169163975144f64,
                lng: 0.1f64,
            };
            init
        },
    ];
    let geoloop_6: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: verts_6.as_mut_ptr(),
        };
        init
    };
    let expected_6: BBox = {
        let mut init = BBox {
            north: -1.57079632679489661923132169163975144f64 + 0.1f64,
            south: -1.57079632679489661923132169163975144f64,
            east: 0.8f64,
            west: 0.1f64,
        };
        init
    };
    let inside_5: LatLng = {
        let mut init = LatLng {
            lat: -1.57079632679489661923132169163975144f64 + 0.01f64,
            lng: 0.4f64,
        };
        init
    };
    let outside_6: LatLng = {
        let mut init = LatLng {
            lat: -1.57079632679489661923132169163975144f64,
            lng: 0.9f64,
        };
        init
    };
    assertBBox(&geoloop_6, &expected_6, &inside_5, &outside_6);
    currentTestName = b"containsEdges\0" as *const u8 as *const libc::c_char;
    let bbox: BBox = {
        let mut init = BBox {
            north: 0.1f64,
            south: -0.1f64,
            east: 0.2f64,
            west: -0.2f64,
        };
        init
    };
    let mut points: [LatLng; 8] = [
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 0.0f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: -0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.0f64,
                lng: 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: 0.0f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: -0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.0f64,
                lng: -0.2f64,
            };
            init
        },
    ];
    let numPoints: libc::c_int = 8 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numPoints {
        if !bboxContains(&bbox, &mut *points.as_mut_ptr().offset(i as isize)) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
                144 as libc::c_int,
                b"bboxContains(&bbox, &points[i])\0" as *const u8 as *const libc::c_char,
                b"Contains edge point\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    currentTestName = b"containsEdgesTransmeridian\0" as *const u8 as *const libc::c_char;
    let bbox_0: BBox = {
        let mut init = BBox {
            north: 0.1f64,
            south: -0.1f64,
            east: -3.14159265358979323846264338327950288f64 + 0.2f64,
            west: 3.14159265358979323846264338327950288f64 - 0.2f64,
        };
        init
    };
    let mut points_0: [LatLng; 8] = [
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 3.14159265358979323846264338327950288f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.0f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: 3.14159265358979323846264338327950288f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.0f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.2f64,
            };
            init
        },
    ];
    let numPoints_0: libc::c_int = 8 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < numPoints_0 {
        if !bboxContains(&bbox_0, &mut *points_0.as_mut_ptr().offset(i_0 as isize)) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
                159 as libc::c_int,
                b"bboxContains(&bbox, &points[i])\0" as *const u8 as *const libc::c_char,
                b"Contains transmeridian edge point\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_0 += 1;
    }
    currentTestName = b"bboxCenterBasicQuandrants\0" as *const u8 as *const libc::c_char;
    let mut center: LatLng = LatLng { lat: 0., lng: 0. };
    let mut bbox1: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.8f64,
            east: 1.0f64,
            west: 0.8f64,
        };
        init
    };
    let mut expected1: LatLng = {
        let mut init = LatLng {
            lat: 0.9f64,
            lng: 0.9f64,
        };
        init
    };
    bboxCenter(&mut bbox1, &mut center);
    if !geoAlmostEqual(&mut center, &mut expected1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            b"geoAlmostEqual(&center, &expected1)\0" as *const u8 as *const libc::c_char,
            b"pos/pos as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut bbox2: BBox = {
        let mut init = BBox {
            north: -0.8f64,
            south: -1.0f64,
            east: 1.0f64,
            west: 0.8f64,
        };
        init
    };
    let mut expected2: LatLng = {
        let mut init = LatLng {
            lat: -0.9f64,
            lng: 0.9f64,
        };
        init
    };
    bboxCenter(&mut bbox2, &mut center);
    if !geoAlmostEqual(&mut center, &mut expected2) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            b"geoAlmostEqual(&center, &expected2)\0" as *const u8 as *const libc::c_char,
            b"neg/pos as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut bbox3: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.8f64,
            east: -0.8f64,
            west: -1.0f64,
        };
        init
    };
    let mut expected3: LatLng = {
        let mut init = LatLng {
            lat: 0.9f64,
            lng: -0.9f64,
        };
        init
    };
    bboxCenter(&mut bbox3, &mut center);
    if !geoAlmostEqual(&mut center, &mut expected3) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            b"geoAlmostEqual(&center, &expected3)\0" as *const u8 as *const libc::c_char,
            b"pos/neg as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut bbox4: BBox = {
        let mut init = BBox {
            north: -0.8f64,
            south: -1.0f64,
            east: -0.8f64,
            west: -1.0f64,
        };
        init
    };
    let mut expected4: LatLng = {
        let mut init = LatLng {
            lat: -0.9f64,
            lng: -0.9f64,
        };
        init
    };
    bboxCenter(&mut bbox4, &mut center);
    if !geoAlmostEqual(&mut center, &mut expected4) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            b"geoAlmostEqual(&center, &expected4)\0" as *const u8 as *const libc::c_char,
            b"neg/neg as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut bbox5: BBox = {
        let mut init = BBox {
            north: 0.8f64,
            south: -0.8f64,
            east: 1.0f64,
            west: -1.0f64,
        };
        init
    };
    let mut expected5: LatLng = {
        let mut init = LatLng {
            lat: 0.0f64,
            lng: 0.0f64,
        };
        init
    };
    bboxCenter(&mut bbox5, &mut center);
    if !geoAlmostEqual(&mut center, &mut expected5) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            b"geoAlmostEqual(&center, &expected5)\0" as *const u8 as *const libc::c_char,
            b"around origin as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"bboxCenterTransmeridian\0" as *const u8 as *const libc::c_char;
    let mut center_0: LatLng = LatLng { lat: 0., lng: 0. };
    let mut bbox1_0: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.8f64,
            east: -3.14159265358979323846264338327950288f64 + 0.3f64,
            west: 3.14159265358979323846264338327950288f64 - 0.1f64,
        };
        init
    };
    let mut expected1_0: LatLng = {
        let mut init = LatLng {
            lat: 0.9f64,
            lng: -3.14159265358979323846264338327950288f64 + 0.1f64,
        };
        init
    };
    bboxCenter(&mut bbox1_0, &mut center_0);
    if !geoAlmostEqual(&mut center_0, &mut expected1_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            b"geoAlmostEqual(&center, &expected1)\0" as *const u8 as *const libc::c_char,
            b"skew east as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut bbox2_0: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.8f64,
            east: -3.14159265358979323846264338327950288f64 + 0.1f64,
            west: 3.14159265358979323846264338327950288f64 - 0.3f64,
        };
        init
    };
    let mut expected2_0: LatLng = {
        let mut init = LatLng {
            lat: 0.9f64,
            lng: 3.14159265358979323846264338327950288f64 - 0.1f64,
        };
        init
    };
    bboxCenter(&mut bbox2_0, &mut center_0);
    if !geoAlmostEqual(&mut center_0, &mut expected2_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int,
            b"geoAlmostEqual(&center, &expected2)\0" as *const u8 as *const libc::c_char,
            b"skew west as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut bbox3_0: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.8f64,
            east: -3.14159265358979323846264338327950288f64 + 0.1f64,
            west: 3.14159265358979323846264338327950288f64 - 0.1f64,
        };
        init
    };
    let mut expected3_0: LatLng = {
        let mut init = LatLng {
            lat: 0.9f64,
            lng: 3.14159265358979323846264338327950288f64,
        };
        init
    };
    bboxCenter(&mut bbox3_0, &mut center_0);
    if !geoAlmostEqual(&mut center_0, &mut expected3_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            b"geoAlmostEqual(&center, &expected3)\0" as *const u8 as *const libc::c_char,
            b"on antimeridian as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"bboxIsTransmeridian\0" as *const u8 as *const libc::c_char;
    let mut bboxNormal: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.8f64,
            east: 1.0f64,
            west: 0.8f64,
        };
        init
    };
    if bboxIsTransmeridian(&mut bboxNormal) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            b"!bboxIsTransmeridian(&bboxNormal)\0" as *const u8 as *const libc::c_char,
            b"Normal bbox not transmeridian\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut bboxTransmeridian: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.8f64,
            east: -3.14159265358979323846264338327950288f64 + 0.3f64,
            west: 3.14159265358979323846264338327950288f64 - 0.1f64,
        };
        init
    };
    if !bboxIsTransmeridian(&mut bboxTransmeridian) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            b"bboxIsTransmeridian(&bboxTransmeridian)\0" as *const u8 as *const libc::c_char,
            b"Transmeridian bbox is transmeridian\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"bboxEquals\0" as *const u8 as *const libc::c_char;
    let mut bbox_1: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.0f64,
            east: 1.0f64,
            west: 0.0f64,
        };
        init
    };
    let mut north: BBox = bbox_1;
    north.north += 0.1f64;
    let mut south: BBox = bbox_1;
    south.south += 0.1f64;
    let mut east: BBox = bbox_1;
    east.east += 0.1f64;
    let mut west: BBox = bbox_1;
    west.west += 0.1f64;
    if !bboxEquals(&mut bbox_1, &mut bbox_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            235 as libc::c_int,
            b"bboxEquals(&bbox, &bbox)\0" as *const u8 as *const libc::c_char,
            b"Equals self\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if bboxEquals(&mut bbox_1, &mut north) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int,
            b"!bboxEquals(&bbox, &north)\0" as *const u8 as *const libc::c_char,
            b"Not equals different north\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if bboxEquals(&mut bbox_1, &mut south) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int,
            b"!bboxEquals(&bbox, &south)\0" as *const u8 as *const libc::c_char,
            b"Not equals different south\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if bboxEquals(&mut bbox_1, &mut east) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            b"!bboxEquals(&bbox, &east)\0" as *const u8 as *const libc::c_char,
            b"Not equals different east\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if bboxEquals(&mut bbox_1, &mut west) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            239 as libc::c_int,
            b"!bboxEquals(&bbox, &west)\0" as *const u8 as *const libc::c_char,
            b"Not equals different west\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"bboxHexEstimate_invalidRes\0" as *const u8 as *const libc::c_char;
    let mut numHexagons: int64_t = 0;
    let mut bbox_2: BBox = {
        let mut init = BBox {
            north: 1.0f64,
            south: 0.0f64,
            east: 1.0f64,
            west: 0.0f64,
        };
        init
    };
    if !(bboxHexEstimate(&mut bbox_2, -(1 as libc::c_int), &mut numHexagons)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int,
            b"bboxHexEstimate(&bbox, -1, &numHexagons) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"bboxHexEstimate of invalid resolution fails\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"lineHexEstimate_invalidRes\0" as *const u8 as *const libc::c_char;
    let mut numHexagons_0: int64_t = 0;
    let mut origin: LatLng = {
        let mut init = LatLng {
            lat: 0.0f64,
            lng: 0.0f64,
        };
        init
    };
    let mut destination: LatLng = {
        let mut init = LatLng {
            lat: 1.0f64,
            lng: 1.0f64,
        };
        init
    };
    if !(lineHexEstimate(
        &mut origin,
        &mut destination,
        -(1 as libc::c_int),
        &mut numHexagons_0,
    ) == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBBox.c\0" as *const u8 as *const libc::c_char,
            255 as libc::c_int,
            b"lineHexEstimate(&origin, &destination, -1, &numHexagons) == E_RES_DOMAIN\0"
                as *const u8 as *const libc::c_char,
            b"lineHexEstimate of invalid resolution fails\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"BBox\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"BBox\0" as *const u8 as *const libc::c_char,
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
