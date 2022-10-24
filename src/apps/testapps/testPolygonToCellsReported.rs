#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
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
    fn degsToRads(degrees: libc::c_double) -> libc::c_double;
    fn getNumCells(res: libc::c_int, out: *mut int64_t) -> H3Error;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn countNonNullIndexes(indexes: *mut H3Index, numCells: int64_t) -> int64_t;
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
    pub _extra: *mut libc::c_void,
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
unsafe extern "C" fn runTests() {
    currentTestName = b"entireWorld\0" as *const u8 as *const libc::c_char;
    let mut worldVerts: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: -1.5707963267948966f64,
                lng: -3.14159265358979323846f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.5707963267948966f64,
                lng: -3.14159265358979323846f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.5707963267948966f64,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -1.5707963267948966f64,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
    ];
    let mut worldGeoLoop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: worldVerts.as_mut_ptr(),
        };
        init
    };
    let mut worldGeoPolygon: GeoPolygon = {
        let mut init = GeoPolygon {
            geoloop: worldGeoLoop,
            numHoles: 0 as libc::c_int,
            holes: 0 as *mut GeoLoop,
        };
        init
    };
    let mut worldVerts2: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: -1.5707963267948966f64,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.5707963267948966f64,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 1.5707963267948966f64,
                lng: 3.14159265358979323846f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -1.5707963267948966f64,
                lng: 3.14159265358979323846f64,
            };
            init
        },
    ];
    let mut worldGeoLoop2: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: worldVerts2.as_mut_ptr(),
        };
        init
    };
    let mut worldGeoPolygon2: GeoPolygon = {
        let mut init = GeoPolygon {
            geoloop: worldGeoLoop2,
            numHoles: 0 as libc::c_int,
            holes: 0 as *mut GeoLoop,
        };
        init
    };
    let mut res: libc::c_int = 0 as libc::c_int;
    while res < 3 as libc::c_int {
        let mut polygonToCellsSize: int64_t = 0;
        if maxPolygonToCellsSize(
            &mut worldGeoPolygon,
            res,
            0 as libc::c_int as uint32_t,
            &mut polygonToCellsSize,
        ) != 0
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8
                    as *const libc::c_char,
                44 as libc::c_int,
                b"!(maxPolygonToCellsSize( &worldGeoPolygon, res, 0, &polygonToCellsSize))\0"
                    as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut polygonToCellsOut: *mut H3Index = calloc(
            polygonToCellsSize as libc::c_ulong,
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
        ) as *mut H3Index;
        if polygonToCells(
            &mut worldGeoPolygon,
            res,
            0 as libc::c_int as uint32_t,
            polygonToCellsOut,
        ) != 0
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8
                    as *const libc::c_char,
                49 as libc::c_int,
                b"!(polygonToCells(&worldGeoPolygon, res, 0, polygonToCellsOut))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut actualNumIndexes: int64_t =
            countNonNullIndexes(polygonToCellsOut, polygonToCellsSize);
        let mut polygonToCellsSize2: int64_t = 0;
        if maxPolygonToCellsSize(
            &mut worldGeoPolygon2,
            res,
            0 as libc::c_int as uint32_t,
            &mut polygonToCellsSize2,
        ) != 0
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8
                    as *const libc::c_char,
                55 as libc::c_int,
                b"!(maxPolygonToCellsSize( &worldGeoPolygon2, res, 0, &polygonToCellsSize2))\0"
                    as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut polygonToCellsOut2: *mut H3Index = calloc(
            polygonToCellsSize2 as libc::c_ulong,
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
        ) as *mut H3Index;
        if polygonToCells(
            &mut worldGeoPolygon2,
            res,
            0 as libc::c_int as uint32_t,
            polygonToCellsOut2,
        ) != 0
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8
                    as *const libc::c_char,
                60 as libc::c_int,
                b"!(polygonToCells(&worldGeoPolygon2, res, 0, polygonToCellsOut2))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut actualNumIndexes2: int64_t =
            countNonNullIndexes(polygonToCellsOut2, polygonToCellsSize2);
        let mut expectedTotalWorld: int64_t = 0;
        if getNumCells(res, &mut expectedTotalWorld) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8
                    as *const libc::c_char,
                65 as libc::c_int,
                b"!(getNumCells(res, &expectedTotalWorld))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(actualNumIndexes + actualNumIndexes2 == expectedTotalWorld) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8
                    as *const libc::c_char,
                67 as libc::c_int,
                b"actualNumIndexes + actualNumIndexes2 == expectedTotalWorld\0" as *const u8
                    as *const libc::c_char,
                b"got expected polygonToCells size (entire world)\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut i: libc::c_int = 0 as libc::c_int;
        while (i as libc::c_longlong) < polygonToCellsSize {
            if !(*polygonToCellsOut.offset(i as isize) == 0 as libc::c_int as libc::c_ulonglong) {
                let mut found: bool = 0 as libc::c_int != 0;
                let mut j: libc::c_int = 0 as libc::c_int;
                while (j as libc::c_longlong) < polygonToCellsSize2 {
                    if *polygonToCellsOut.offset(i as isize)
                        == *polygonToCellsOut2.offset(j as isize)
                    {
                        found = 1 as libc::c_int != 0;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if found {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8
                            as *const libc::c_char,
                        83 as libc::c_int,
                        b"!found\0" as *const u8 as *const libc::c_char,
                        b"Index found more than once when polygonToCellsing the entire world\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
        }
        free(polygonToCellsOut as *mut libc::c_void);
        free(polygonToCellsOut2 as *mut libc::c_void);
        res += 1;
    }
    currentTestName = b"h3js_67\0" as *const u8 as *const libc::c_char;
    let mut east: libc::c_double = degsToRads(-56.25f64);
    let mut north: libc::c_double = degsToRads(-33.13755119234615f64);
    let mut south: libc::c_double = degsToRads(-34.30714385628804f64);
    let mut west: libc::c_double = degsToRads(-57.65625f64);
    let mut testVerts: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: north,
                lng: east,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: south,
                lng: east,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: south,
                lng: west,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: north,
                lng: west,
            };
            init
        },
    ];
    let mut testGeoLoop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: testVerts.as_mut_ptr(),
        };
        init
    };
    let mut testPolygon: GeoPolygon = GeoPolygon {
        geoloop: GeoLoop {
            numVerts: 0,
            verts: 0 as *mut LatLng,
        },
        numHoles: 0,
        holes: 0 as *mut GeoLoop,
    };
    testPolygon.geoloop = testGeoLoop;
    testPolygon.numHoles = 0 as libc::c_int;
    let mut res_0: libc::c_int = 7 as libc::c_int;
    let mut numHexagons: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut testPolygon,
        res_0,
        0 as libc::c_int as uint32_t,
        &mut numHexagons,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            b"!(maxPolygonToCellsSize(&testPolygon, res, 0, &numHexagons))\0" as *const u8
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
    if polygonToCells(
        &mut testPolygon,
        res_0,
        0 as libc::c_int as uint32_t,
        hexagons,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
            b"!(polygonToCells(&testPolygon, res, 0, hexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes_0: int64_t = countNonNullIndexes(hexagons, numHexagons);
    if !(actualNumIndexes_0 == 4499 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            b"actualNumIndexes == 4499\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size (h3-js#67)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons as *mut libc::c_void);
    currentTestName = b"h3js_67_2nd\0" as *const u8 as *const libc::c_char;
    let mut east_0: libc::c_double = degsToRads(-57.65625f64);
    let mut north_0: libc::c_double = degsToRads(-34.30714385628804f64);
    let mut south_0: libc::c_double = degsToRads(-35.4606699514953f64);
    let mut west_0: libc::c_double = degsToRads(-59.0625f64);
    let mut testVerts_0: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: north_0,
                lng: east_0,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: south_0,
                lng: east_0,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: south_0,
                lng: west_0,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: north_0,
                lng: west_0,
            };
            init
        },
    ];
    let mut testGeoLoop_0: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: testVerts_0.as_mut_ptr(),
        };
        init
    };
    let mut testPolygon_0: GeoPolygon = GeoPolygon {
        geoloop: GeoLoop {
            numVerts: 0,
            verts: 0 as *mut LatLng,
        },
        numHoles: 0,
        holes: 0 as *mut GeoLoop,
    };
    testPolygon_0.geoloop = testGeoLoop_0;
    testPolygon_0.numHoles = 0 as libc::c_int;
    let mut res_1: libc::c_int = 7 as libc::c_int;
    let mut numHexagons_0: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut testPolygon_0,
        res_1,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_0,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            b"!(maxPolygonToCellsSize(&testPolygon, res, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_0: *mut H3Index = calloc(
        numHexagons_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut testPolygon_0,
        res_1,
        0 as libc::c_int as uint32_t,
        hexagons_0,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            b"!(polygonToCells(&testPolygon, res, 0, hexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes_1: int64_t = countNonNullIndexes(hexagons_0, numHexagons_0);
    if !(actualNumIndexes_1 == 4609 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            b"actualNumIndexes == 4609\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size (h3-js#67, 2nd case)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_0 as *mut libc::c_void);
    currentTestName = b"h3_136\0" as *const u8 as *const libc::c_char;
    let mut testVerts_1: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.10068990369902957f64,
                lng: 0.8920772174196191f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.10032914690616246f64,
                lng: 0.8915914753447348f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.10033349237998787f64,
                lng: 0.8915860128746426f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.10069496685903621f64,
                lng: 0.8920742194546231f64,
            };
            init
        },
    ];
    let mut testGeoLoop_1: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: testVerts_1.as_mut_ptr(),
        };
        init
    };
    let mut testPolygon_1: GeoPolygon = GeoPolygon {
        geoloop: GeoLoop {
            numVerts: 0,
            verts: 0 as *mut LatLng,
        },
        numHoles: 0,
        holes: 0 as *mut GeoLoop,
    };
    testPolygon_1.geoloop = testGeoLoop_1;
    testPolygon_1.numHoles = 0 as libc::c_int;
    let mut res_2: libc::c_int = 13 as libc::c_int;
    let mut numHexagons_1: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut testPolygon_1,
        res_2,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_1,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            b"!(maxPolygonToCellsSize(&testPolygon, res, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_1: *mut H3Index = calloc(
        numHexagons_1 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut testPolygon_1,
        res_2,
        0 as libc::c_int as uint32_t,
        hexagons_1,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int,
            b"!(polygonToCells(&testPolygon, res, 0, hexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes_2: int64_t = countNonNullIndexes(hexagons_1, numHexagons_1);
    if !(actualNumIndexes_2 == 4353 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            b"actualNumIndexes == 4353\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_1 as *mut libc::c_void);
    currentTestName = b"h3_136\0" as *const u8 as *const libc::c_char;
    let mut center: H3Index = 0x85283473fffffff as libc::c_long as H3Index;
    let mut centerLatLng: LatLng = LatLng { lat: 0., lng: 0. };
    cellToLatLng(center, &mut centerLatLng);
    let mut testVerts_2: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: centerLatLng.lat,
                lng: -2.121207808248113f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.6565301558937859f64,
                lng: -2.1281107217935986f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.6515463604919347f64,
                lng: -2.1345342663428695f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.6466583305904194f64,
                lng: -2.1276313527973842f64,
            };
            init
        },
    ];
    let mut testGeoLoop_2: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: testVerts_2.as_mut_ptr(),
        };
        init
    };
    let mut testPolygon_2: GeoPolygon = GeoPolygon {
        geoloop: GeoLoop {
            numVerts: 0,
            verts: 0 as *mut LatLng,
        },
        numHoles: 0,
        holes: 0 as *mut GeoLoop,
    };
    testPolygon_2.geoloop = testGeoLoop_2;
    testPolygon_2.numHoles = 0 as libc::c_int;
    let mut res_3: libc::c_int = 5 as libc::c_int;
    let mut numHexagons_2: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut testPolygon_2,
        res_3,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_2,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            b"!(maxPolygonToCellsSize(&testPolygon, res, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_2: *mut H3Index = calloc(
        numHexagons_2 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut testPolygon_2,
        res_3,
        0 as libc::c_int as uint32_t,
        hexagons_2,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            b"!(polygonToCells(&testPolygon, res, 0, hexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes_3: int64_t = countNonNullIndexes(hexagons_2, numHexagons_2);
    if !(actualNumIndexes_3 == 8 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCellsReported.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
            b"actualNumIndexes == 8\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_2 as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"polygonToCells_reported\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"polygonToCells_reported\0" as *const u8 as *const libc::c_char,
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
