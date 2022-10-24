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

    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
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
    fn getResolution(h: H3Index) -> libc::c_int;
    fn cellToChildrenSize(h: H3Index, childRes: libc::c_int, out: *mut int64_t) -> H3Error;
    fn cellToChildren(h: H3Index, childRes: libc::c_int, children: *mut H3Index) -> H3Error;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn _getEdgeHexagons(
        geoloop: *const GeoLoop,
        numHexagons: int64_t,
        res: libc::c_int,
        numSearchHexes: *mut int64_t,
        search: *mut H3Index,
        found: *mut H3Index,
    ) -> H3Error;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn iterateAllIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    );
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
pub struct GeoPolygon {
    pub geoloop: GeoLoop,
    pub numHoles: libc::c_int,
    pub holes: *mut GeoLoop,
}
pub type Direction = libc::c_uint;
pub const PENTAGON_SKIPPED_DIGIT: Direction = 1;
pub const NUM_DIGITS: Direction = 7;
pub const INVALID_DIGIT: Direction = 7;
pub const IJ_AXES_DIGIT: Direction = 6;
pub const IK_AXES_DIGIT: Direction = 5;
pub const I_AXES_DIGIT: Direction = 4;
pub const JK_AXES_DIGIT: Direction = 3;
pub const J_AXES_DIGIT: Direction = 2;
pub const K_AXES_DIGIT: Direction = 1;
pub const CENTER_DIGIT: Direction = 0;
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
static mut holeVerts: [LatLng; 3] = [
    {
        let mut init = LatLng {
            lat: 0.6595072188743f64,
            lng: -2.1371053983433f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591482046471f64,
            lng: -2.1373141048153f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592295020837f64,
            lng: -2.1365222838402f64,
        };
        init
    },
];
static mut holeGeoLoop: GeoLoop = unsafe {
    {
        let mut init = GeoLoop {
            numVerts: 3 as libc::c_int,
            verts: holeVerts.as_ptr() as *mut _,
        };
        init
    }
};
static mut holeGeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
static mut emptyVerts: [LatLng; 3] = [
    {
        let mut init = LatLng {
            lat: 0.659966917655f64,
            lng: -2.1364398519394f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659966917656f64,
            lng: -2.1364398519395f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659966917657f64,
            lng: -2.1364398519396f64,
        };
        init
    },
];
static mut emptyGeoLoop: GeoLoop = unsafe {
    {
        let mut init = GeoLoop {
            numVerts: 3 as libc::c_int,
            verts: emptyVerts.as_ptr() as *mut _,
        };
        init
    }
};
static mut emptyGeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
static mut invalidVerts: [LatLng; 2] = [
    {
        let mut init = LatLng {
            lat: ::core::f32::INFINITY as libc::c_double,
            lng: ::core::f32::INFINITY as libc::c_double,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -::core::f32::INFINITY as libc::c_double,
            lng: -::core::f32::INFINITY as libc::c_double,
        };
        init
    },
];
static mut invalidGeoLoop: GeoLoop = unsafe {
    {
        let mut init = GeoLoop {
            numVerts: 2 as libc::c_int,
            verts: invalidVerts.as_ptr() as *mut _,
        };
        init
    }
};
static mut invalidGeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
static mut invalid2Verts: [LatLng; 2] = [
    {
        let mut init = LatLng {
            lat: ::core::f32::NAN as libc::c_double,
            lng: ::core::f32::NAN as libc::c_double,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -::core::f32::NAN as libc::c_double,
            lng: -::core::f32::NAN as libc::c_double,
        };
        init
    },
];
static mut invalid2GeoLoop: GeoLoop = unsafe {
    {
        let mut init = GeoLoop {
            numVerts: 2 as libc::c_int,
            verts: invalid2Verts.as_ptr() as *mut _,
        };
        init
    }
};
static mut invalid2GeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
static mut pointVerts: [LatLng; 1] = [{
    let mut init = LatLng {
        lat: 0 as libc::c_int as libc::c_double,
        lng: 0 as libc::c_int as libc::c_double,
    };
    init
}];
static mut pointGeoLoop: GeoLoop = unsafe {
    {
        let mut init = GeoLoop {
            numVerts: 1 as libc::c_int,
            verts: pointVerts.as_ptr() as *mut _,
        };
        init
    }
};
static mut pointGeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
static mut lineVerts: [LatLng; 2] = [
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
];
static mut lineGeoLoop: GeoLoop = unsafe {
    {
        let mut init = GeoLoop {
            numVerts: 2 as libc::c_int,
            verts: lineVerts.as_ptr() as *mut _,
        };
        init
    }
};
static mut lineGeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
unsafe extern "C" fn isTransmeridianCell(mut h: H3Index) -> bool {
    let mut bndry: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    cellToBoundary(h, &mut bndry);
    let mut minLng: libc::c_double = 3.14159265358979323846264338327950288f64;
    let mut maxLng: libc::c_double = -3.14159265358979323846264338327950288f64;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < bndry.numVerts {
        if bndry.verts[i as usize].lng < minLng {
            minLng = bndry.verts[i as usize].lng;
        }
        if bndry.verts[i as usize].lng > maxLng {
            maxLng = bndry.verts[i as usize].lng;
        }
        i += 1;
    }
    return maxLng - minLng
        > 3.14159265358979323846264338327950288f64
            - 3.14159265358979323846264338327950288f64 / 4 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn fillIndex_assertions(mut h: H3Index) {
    if isTransmeridianCell(h) {
        return;
    }
    let mut currentRes: libc::c_int = getResolution(h);
    let mut nextRes: libc::c_int = currentRes;
    while nextRes <= currentRes + 1 as libc::c_int {
        let mut bndry: CellBoundary = CellBoundary {
            numVerts: 0,
            verts: [LatLng { lat: 0., lng: 0. }; 10],
        };
        cellToBoundary(h, &mut bndry);
        let mut polygon: GeoPolygon = {
            let mut init = GeoPolygon {
                geoloop: {
                    let mut init = GeoLoop {
                        numVerts: bndry.numVerts,
                        verts: (bndry.verts).as_mut_ptr(),
                    };
                    init
                },
                numHoles: 0 as libc::c_int,
                holes: 0 as *mut GeoLoop,
            };
            init
        };
        let mut polygonToCellsSize: int64_t = 0;
        if maxPolygonToCellsSize(
            &mut polygon,
            nextRes,
            0 as libc::c_int as uint32_t,
            &mut polygonToCellsSize,
        ) != 0
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int,
                b"!(maxPolygonToCellsSize(&polygon, nextRes, 0, &polygonToCellsSize))\0"
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
            &mut polygon,
            nextRes,
            0 as libc::c_int as uint32_t,
            polygonToCellsOut,
        ) != 0
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
                b"!(polygonToCells(&polygon, nextRes, 0, polygonToCellsOut))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut polygonToCellsCount: int64_t =
            countNonNullIndexes(polygonToCellsOut, polygonToCellsSize);
        let mut childrenSize: int64_t = 0;
        if cellToChildrenSize(h, nextRes, &mut childrenSize) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int,
                b"!(cellToChildrenSize(h, nextRes, &childrenSize))\0" as *const u8
                    as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut children: *mut H3Index = calloc(
            childrenSize as libc::c_ulong,
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
        ) as *mut H3Index;
        if cellToChildren(h, nextRes, children) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
                111 as libc::c_int,
                b"!(cellToChildren(h, nextRes, children))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut cellToChildrenCount: int64_t = countNonNullIndexes(children, childrenSize);
        if !(polygonToCellsCount == cellToChildrenCount) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int,
                b"polygonToCellsCount == cellToChildrenCount\0" as *const u8 as *const libc::c_char,
                b"PolygonToCells count matches cellToChildren count\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut i: libc::c_int = 0 as libc::c_int;
        while (i as libc::c_longlong) < childrenSize {
            let mut found: bool = 0 as libc::c_int != 0;
            if !(*children.offset(i as isize) == 0 as libc::c_int as libc::c_ulonglong) {
                let mut j: libc::c_int = 0 as libc::c_int;
                while (j as libc::c_longlong) < polygonToCellsSize {
                    if *polygonToCellsOut.offset(j as isize) == *children.offset(i as isize) {
                        found = 1 as libc::c_int != 0;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if !found {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"src/apps/testapps/testPolygonToCells.c\0" as *const u8
                            as *const libc::c_char,
                        130 as libc::c_int,
                        b"found\0" as *const u8 as *const libc::c_char,
                        b"All indexes match between polygonToCells and cellToChildren\0"
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
        free(children as *mut libc::c_void);
        nextRes += 1;
    }
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"polygonToCells\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"polygonToCells\0" as *const u8 as *const libc::c_char,
    );
    runTests();
    printf(
        b"\nDONE: %d assertions\n\0" as *const u8 as *const libc::c_char,
        globalTestCount,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn runTests() {
    sfGeoPolygon.geoloop = sfGeoLoop;
    sfGeoPolygon.numHoles = 0 as libc::c_int;
    holeGeoPolygon.geoloop = sfGeoLoop;
    holeGeoPolygon.numHoles = 1 as libc::c_int;
    holeGeoPolygon.holes = &mut holeGeoLoop;
    emptyGeoPolygon.geoloop = emptyGeoLoop;
    emptyGeoPolygon.numHoles = 0 as libc::c_int;
    invalidGeoPolygon.geoloop = invalidGeoLoop;
    invalidGeoPolygon.numHoles = 0 as libc::c_int;
    invalid2GeoPolygon.geoloop = invalid2GeoLoop;
    invalid2GeoPolygon.numHoles = 0 as libc::c_int;
    pointGeoPolygon.geoloop = pointGeoLoop;
    pointGeoPolygon.numHoles = 0 as libc::c_int;
    lineGeoPolygon.geoloop = lineGeoLoop;
    lineGeoPolygon.numHoles = 0 as libc::c_int;
    currentTestName = b"maxPolygonToCellsSize\0" as *const u8 as *const libc::c_char;
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
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            b"!(maxPolygonToCellsSize(&sfGeoPolygon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(numHexagons == 5613 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            b"numHexagons == 5613\0" as *const u8 as *const libc::c_char,
            b"got expected max polygonToCells size\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if maxPolygonToCellsSize(
        &mut holeGeoPolygon,
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
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            b"!(maxPolygonToCellsSize(&holeGeoPolygon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(numHexagons == 5613 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            b"numHexagons == 5613\0" as *const u8 as *const libc::c_char,
            b"got expected max polygonToCells size (hole)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if maxPolygonToCellsSize(
        &mut emptyGeoPolygon,
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
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int,
            b"!(maxPolygonToCellsSize(&emptyGeoPolygon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(numHexagons == 15 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            b"numHexagons == 15\0" as *const u8 as *const libc::c_char,
            b"got expected max polygonToCells size (empty)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"polygonToCells\0" as *const u8 as *const libc::c_char;
    let mut numHexagons_0: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut sfGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_0,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int,
            b"!(maxPolygonToCellsSize(&sfGeoPolygon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons: *mut H3Index = calloc(
        numHexagons_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut sfGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            b"!(polygonToCells(&sfGeoPolygon, 9, 0, hexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes: int64_t = countNonNullIndexes(hexagons, numHexagons_0);
    if !(actualNumIndexes == 1253 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            b"actualNumIndexes == 1253\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons as *mut libc::c_void);
    currentTestName = b"polygonToCellsHole\0" as *const u8 as *const libc::c_char;
    let mut numHexagons_1: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut holeGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_1,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            b"!(maxPolygonToCellsSize(&holeGeoPolygon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_0: *mut H3Index = calloc(
        numHexagons_1 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut holeGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons_0,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            b"!(polygonToCells(&holeGeoPolygon, 9, 0, hexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes_0: int64_t = countNonNullIndexes(hexagons_0, numHexagons_1);
    if !(actualNumIndexes_0 == 1214 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
            b"actualNumIndexes == 1214\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size (hole)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_0 as *mut libc::c_void);
    currentTestName = b"polygonToCellsEmpty\0" as *const u8 as *const libc::c_char;
    let mut numHexagons_2: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut emptyGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_2,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
            b"!(maxPolygonToCellsSize(&emptyGeoPolygon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_1: *mut H3Index = calloc(
        numHexagons_2 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut emptyGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons_1,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            b"!(polygonToCells(&emptyGeoPolygon, 9, 0, hexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes_1: int64_t = countNonNullIndexes(hexagons_1, numHexagons_2);
    if !(actualNumIndexes_1 == 0 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int,
            b"actualNumIndexes == 0\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size (empty)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_1 as *mut libc::c_void);
    currentTestName = b"polygonToCellsExact\0" as *const u8 as *const libc::c_char;
    let mut somewhere: LatLng = {
        let mut init = LatLng {
            lat: 1 as libc::c_int as libc::c_double,
            lng: 2 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut origin: H3Index = 0;
    if latLngToCell(&mut somewhere, 9 as libc::c_int, &mut origin) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
            b"!(latLngToCell(&somewhere, 9, &origin))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut boundary: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    cellToBoundary(origin, &mut boundary);
    let mut verts: *mut LatLng = calloc(
        (boundary.numVerts + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<LatLng>() as libc::c_ulong,
    ) as *mut LatLng;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < boundary.numVerts {
        *verts.offset(i as isize) = boundary.verts[i as usize];
        i += 1;
    }
    *verts.offset(boundary.numVerts as isize) = boundary.verts[0 as libc::c_int as usize];
    let mut someGeoLoop: GeoLoop = GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    };
    someGeoLoop.numVerts = boundary.numVerts + 1 as libc::c_int;
    someGeoLoop.verts = verts;
    let mut someHexagon: GeoPolygon = GeoPolygon {
        geoloop: GeoLoop {
            numVerts: 0,
            verts: std::ptr::null::<LatLng>() as *mut LatLng,
        },
        numHoles: 0,
        holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
    };
    someHexagon.geoloop = someGeoLoop;
    someHexagon.numHoles = 0 as libc::c_int;
    let mut numHexagons_3: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut someHexagon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_3,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int,
            b"!(maxPolygonToCellsSize(&someHexagon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_2: *mut H3Index = calloc(
        numHexagons_3 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut someHexagon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons_2,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int,
            b"!(polygonToCells(&someHexagon, 9, 0, hexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes_2: int64_t = countNonNullIndexes(hexagons_2, numHexagons_3);
    if !(actualNumIndexes_2 == 1 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int,
            b"actualNumIndexes == 1\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size (1)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_2 as *mut libc::c_void);
    free(verts as *mut libc::c_void);
    currentTestName = b"polygonToCellsTransmeridian\0" as *const u8 as *const libc::c_char;
    let mut primeMeridianVerts: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.01f64,
                lng: 0.01f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.01f64,
                lng: -0.01f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.01f64,
                lng: -0.01f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.01f64,
                lng: 0.01f64,
            };
            init
        },
    ];
    let mut primeMeridianGeoLoop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: primeMeridianVerts.as_mut_ptr(),
        };
        init
    };
    let mut primeMeridianGeoPolygon: GeoPolygon = {
        let mut init = GeoPolygon {
            geoloop: primeMeridianGeoLoop,
            numHoles: 0 as libc::c_int,
            holes: 0 as *mut GeoLoop,
        };
        init
    };
    let mut transMeridianVerts: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.01f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.01f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.01f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.01f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.01f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.01f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.01f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.01f64,
            };
            init
        },
    ];
    let mut transMeridianGeoLoop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: transMeridianVerts.as_mut_ptr(),
        };
        init
    };
    let mut transMeridianGeoPolygon: GeoPolygon = {
        let mut init = GeoPolygon {
            geoloop: transMeridianGeoLoop,
            numHoles: 0 as libc::c_int,
            holes: 0 as *mut GeoLoop,
        };
        init
    };
    let mut transMeridianHoleVerts: [LatLng; 4] = [
        {
            let mut init = LatLng {
                lat: 0.005f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.005f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.005f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.005f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.005f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.005f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.005f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.005f64,
            };
            init
        },
    ];
    let mut transMeridianHoleGeoLoop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 4 as libc::c_int,
            verts: transMeridianHoleVerts.as_mut_ptr(),
        };
        init
    };
    let mut transMeridianHoleGeoPolygon: GeoPolygon = {
        let mut init = GeoPolygon {
            geoloop: transMeridianGeoLoop,
            numHoles: 1 as libc::c_int,
            holes: &mut transMeridianHoleGeoLoop,
        };
        init
    };
    let mut transMeridianFilledHoleGeoPolygon: GeoPolygon = {
        let mut init = GeoPolygon {
            geoloop: transMeridianHoleGeoLoop,
            numHoles: 0 as libc::c_int,
            holes: 0 as *mut GeoLoop,
        };
        init
    };
    let mut expectedSize: int64_t = 0;
    expectedSize = 4228 as libc::c_int as int64_t;
    let mut numHexagons_4: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut primeMeridianGeoPolygon,
        7 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_4,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int,
            b"!(maxPolygonToCellsSize( &primeMeridianGeoPolygon, 7, 0, &numHexagons))\0"
                as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_3: *mut H3Index = calloc(
        numHexagons_4 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut primeMeridianGeoPolygon,
        7 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons_3,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int,
            b"!(polygonToCells(&primeMeridianGeoPolygon, 7, 0, hexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes_3: int64_t = countNonNullIndexes(hexagons_3, numHexagons_4);
    if !(actualNumIndexes_3 == expectedSize) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int,
            b"actualNumIndexes == expectedSize\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size (prime meridian)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    expectedSize = 4238 as libc::c_int as int64_t;
    if maxPolygonToCellsSize(
        &mut transMeridianGeoPolygon,
        7 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_4,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            307 as libc::c_int,
            b"!(maxPolygonToCellsSize( &transMeridianGeoPolygon, 7, 0, &numHexagons))\0"
                as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagonsTM: *mut H3Index = calloc(
        numHexagons_4 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut transMeridianGeoPolygon,
        7 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagonsTM,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int,
            b"!(polygonToCells(&transMeridianGeoPolygon, 7, 0, hexagonsTM))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    actualNumIndexes_3 = countNonNullIndexes(hexagonsTM, numHexagons_4);
    if !(actualNumIndexes_3 == expectedSize) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int,
            b"actualNumIndexes == expectedSize\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size (transmeridian)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if maxPolygonToCellsSize(
        &mut transMeridianFilledHoleGeoPolygon,
        7 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_4,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int,
            b"!(maxPolygonToCellsSize( &transMeridianFilledHoleGeoPolygon, 7, 0, &numHexagons))\0"
                as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagonsTMFH: *mut H3Index = calloc(
        numHexagons_4 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut transMeridianFilledHoleGeoPolygon,
        7 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagonsTMFH,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            b"!(polygonToCells( &transMeridianFilledHoleGeoPolygon, 7, 0, hexagonsTMFH))\0"
                as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumHoleIndexes: int64_t = countNonNullIndexes(hexagonsTMFH, numHexagons_4);
    if maxPolygonToCellsSize(
        &mut transMeridianHoleGeoPolygon,
        7 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_4,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            b"!(maxPolygonToCellsSize( &transMeridianHoleGeoPolygon, 7, 0, &numHexagons))\0"
                as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagonsTMH: *mut H3Index = calloc(
        numHexagons_4 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut transMeridianHoleGeoPolygon,
        7 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagonsTMH,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            334 as libc::c_int,
            b"!(polygonToCells(&transMeridianHoleGeoPolygon, 7, 0, hexagonsTMH))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    actualNumIndexes_3 = countNonNullIndexes(hexagonsTMH, numHexagons_4);
    if !(actualNumIndexes_3 == expectedSize - actualNumHoleIndexes) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            338 as libc::c_int,
            b"actualNumIndexes == expectedSize - actualNumHoleIndexes\0" as *const u8
                as *const libc::c_char,
            b"got expected polygonToCells size (transmeridian hole)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_3 as *mut libc::c_void);
    free(hexagonsTM as *mut libc::c_void);
    free(hexagonsTMFH as *mut libc::c_void);
    free(hexagonsTMH as *mut libc::c_void);
    currentTestName = b"polygonToCellsTransmeridianComplex\0" as *const u8 as *const libc::c_char;
    let mut verts_0: [LatLng; 6] = [
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.00001f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.1f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.00001f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: 0.05f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.2f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: 3.14159265358979323846264338327950288f64 - 0.00001f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.1f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.00001f64,
            };
            init
        },
        {
            let mut init = LatLng {
                lat: -0.05f64,
                lng: -3.14159265358979323846264338327950288f64 + 0.2f64,
            };
            init
        },
    ];
    let mut geoloop: GeoLoop = {
        let mut init = GeoLoop {
            numVerts: 6 as libc::c_int,
            verts: verts_0.as_mut_ptr(),
        };
        init
    };
    let mut polygon: GeoPolygon = {
        let mut init = GeoPolygon {
            geoloop: geoloop,
            numHoles: 0 as libc::c_int,
            holes: 0 as *mut GeoLoop,
        };
        init
    };
    let mut numHexagons_5: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut polygon,
        4 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_5,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            358 as libc::c_int,
            b"!(maxPolygonToCellsSize(&polygon, 4, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_4: *mut H3Index = calloc(
        numHexagons_5 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut polygon,
        4 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons_4,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            361 as libc::c_int,
            b"!(polygonToCells(&polygon, 4, 0, hexagons))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut actualNumIndexes_4: int64_t = countNonNullIndexes(hexagons_4, numHexagons_5);
    if !(actualNumIndexes_4 == 1204 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            366 as libc::c_int,
            b"actualNumIndexes == 1204\0" as *const u8 as *const libc::c_char,
            b"got expected polygonToCells size (complex transmeridian)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_4 as *mut libc::c_void);
    currentTestName = b"polygonToCellsPentagon\0" as *const u8 as *const libc::c_char;
    let mut pentagon: H3Index = 0;
    setH3Index(
        &mut pentagon,
        9 as libc::c_int,
        24 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut coord: LatLng = LatLng { lat: 0., lng: 0. };
    cellToLatLng(pentagon, &mut coord);
    let mut edgeLength2: libc::c_double = degsToRads(0.001f64);
    let mut boundingTopRigt: LatLng = coord;
    boundingTopRigt.lat += edgeLength2;
    boundingTopRigt.lng += edgeLength2;
    let mut boundingTopLeft: LatLng = coord;
    boundingTopLeft.lat += edgeLength2;
    boundingTopLeft.lng -= edgeLength2;
    let mut boundingBottomRight: LatLng = coord;
    boundingBottomRight.lat -= edgeLength2;
    boundingBottomRight.lng += edgeLength2;
    let mut boundingBottomLeft: LatLng = coord;
    boundingBottomLeft.lat -= edgeLength2;
    boundingBottomLeft.lng -= edgeLength2;
    let mut verts_1: [LatLng; 4] = [
        boundingBottomLeft,
        boundingTopLeft,
        boundingTopRigt,
        boundingBottomRight,
    ];
    let mut geoloop_0: GeoLoop = GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    };
    geoloop_0.verts = verts_1.as_mut_ptr();
    geoloop_0.numVerts = 4 as libc::c_int;
    let mut polygon_0: GeoPolygon = GeoPolygon {
        geoloop: GeoLoop {
            numVerts: 0,
            verts: std::ptr::null::<LatLng>() as *mut LatLng,
        },
        numHoles: 0,
        holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
    };
    polygon_0.geoloop = geoloop_0;
    polygon_0.numHoles = 0 as libc::c_int;
    let mut numHexagons_6: int64_t = 0;
    if maxPolygonToCellsSize(
        &mut polygon_0,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_6,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            409 as libc::c_int,
            b"!(maxPolygonToCellsSize(&polygon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_5: *mut H3Index = calloc(
        numHexagons_6 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if polygonToCells(
        &mut polygon_0,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons_5,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            412 as libc::c_int,
            b"!(polygonToCells(&polygon, 9, 0, hexagons))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut numPentagons: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while (i_0 as libc::c_longlong) < numHexagons_6 {
        if *hexagons_5.offset(i_0 as isize) != 0 as libc::c_int as libc::c_ulonglong {
            found += 1;
        }
        if isPentagon(*hexagons_5.offset(i_0 as isize)) != 0 {
            numPentagons += 1;
        }
        i_0 += 1;
    }
    if !(found == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            424 as libc::c_int,
            b"found == 1\0" as *const u8 as *const libc::c_char,
            b"one index found\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(numPentagons == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            425 as libc::c_int,
            b"numPentagons == 1\0" as *const u8 as *const libc::c_char,
            b"one pentagon found\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_5 as *mut libc::c_void);
    currentTestName = b"invalidFlags\0" as *const u8 as *const libc::c_char;
    let mut numHexagons_7: int64_t = 0;
    let mut flags: uint32_t = 1 as libc::c_int as uint32_t;
    while flags <= 32 as libc::c_int as libc::c_uint {
        if !(maxPolygonToCellsSize(
            &mut sfGeoPolygon,
            9 as libc::c_int,
            flags,
            &mut numHexagons_7,
        ) == E_OPTION_INVALID as libc::c_int as libc::c_uint)
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                    as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCells.c\0"
                    as *const u8 as *const libc::c_char,
                435 as libc::c_int,
                b"H3_EXPORT(maxPolygonToCellsSize)( &sfGeoPolygon, 9, flags, &numHexagons) == E_OPTION_INVALID\0"
                    as *const u8 as *const libc::c_char,
                b"Flags other than 0 are invalid for maxPolygonToCellsSize\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        flags = flags.wrapping_add(1);
    }
    if maxPolygonToCellsSize(
        &mut sfGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_7,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
            b"!(maxPolygonToCellsSize(&sfGeoPolygon, 9, 0, &numHexagons))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut hexagons_6: *mut H3Index = calloc(
        numHexagons_7 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let mut flags_0: uint32_t = 1 as libc::c_int as uint32_t;
    while flags_0 <= 32 as libc::c_int as libc::c_uint {
        if !(polygonToCells(&mut sfGeoPolygon, 9 as libc::c_int, flags_0, hexagons_6)
            == E_OPTION_INVALID as libc::c_int as libc::c_uint)
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                    as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testPolygonToCells.c\0"
                    as *const u8 as *const libc::c_char,
                443 as libc::c_int,
                b"H3_EXPORT(polygonToCells)(&sfGeoPolygon, 9, flags, hexagons) == E_OPTION_INVALID\0"
                    as *const u8 as *const libc::c_char,
                b"Flags other than 0 are invalid for polygonToCells\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        flags_0 = flags_0.wrapping_add(1);
    }
    free(hexagons_6 as *mut libc::c_void);
    currentTestName = b"fillIndex\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(fillIndex_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(fillIndex_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(fillIndex_assertions as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"getEdgeHexagonsInvalid\0" as *const u8 as *const libc::c_char;
    let mut numHexagons_8: int64_t = 100 as libc::c_int as int64_t;
    let mut search: *mut H3Index = calloc(
        numHexagons_8 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if search.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            457 as libc::c_int,
            b"search != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    let mut found_0: *mut H3Index = calloc(
        numHexagons_8 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if found_0.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"runTests\0")).as_ptr(),
            b"testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            459 as libc::c_int,
            b"found != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut numSearchHexes: int64_t = 0 as libc::c_int as int64_t;
    let mut err: H3Error = _getEdgeHexagons(
        &mut invalidGeoLoop,
        numHexagons_8,
        res,
        &mut numSearchHexes,
        search,
        found_0,
    );
    if !(err != E_SUCCESS as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            466 as libc::c_int,
            b"err != E_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"_getEdgeHexagons returns error for invalid geoloop\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(found_0 as *mut libc::c_void);
    free(search as *mut libc::c_void);
    currentTestName = b"polygonToCellsInvalid\0" as *const u8 as *const libc::c_char;
    let mut numHexagons_9: int64_t = 0;
    if !(maxPolygonToCellsSize(
        &mut invalidGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_9,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0"
                as *const u8 as *const libc::c_char,
            477 as libc::c_int,
            b"H3_EXPORT(maxPolygonToCellsSize)(&invalidGeoPolygon, 9, 0, &numHexagons) == E_FAILED\0"
                as *const u8 as *const libc::c_char,
            b"Cannot determine cell size to invalid geo polygon with Infinity\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(maxPolygonToCellsSize(
        &mut invalid2GeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_9,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0"
                as *const u8 as *const libc::c_char,
            480 as libc::c_int,
            b"H3_EXPORT(maxPolygonToCellsSize)(&invalid2GeoPolygon, 9, 0, &numHexagons) == E_FAILED\0"
                as *const u8 as *const libc::c_char,
            b"Cannot determine cell size to invalid geo polygon with NaNs\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    numHexagons_9 = 0 as libc::c_int as int64_t;
    let mut hexagons_7: *mut H3Index = calloc(
        numHexagons_9 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if !(polygonToCells(
        &mut invalidGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        hexagons_7,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int,
            b"H3_EXPORT(polygonToCells)(&invalidGeoPolygon, 9, 0, hexagons) == E_FAILED\0"
                as *const u8 as *const libc::c_char,
            b"Invalid geo polygon cannot be evaluated\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(hexagons_7 as *mut libc::c_void);
    currentTestName = b"polygonToCellsPoint\0" as *const u8 as *const libc::c_char;
    let mut numHexagons_10: int64_t = 0;
    if !(maxPolygonToCellsSize(
        &mut pointGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_10,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            497 as libc::c_int,
            b"H3_EXPORT(maxPolygonToCellsSize)(&pointGeoPolygon, 9, 0, &numHexagons) == E_FAILED\0"
                as *const u8 as *const libc::c_char,
            b"Cannot estimate for single point\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"polygonToCellsLine\0" as *const u8 as *const libc::c_char;
    let mut numHexagons_11: int64_t = 0;
    if !(maxPolygonToCellsSize(
        &mut lineGeoPolygon,
        9 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut numHexagons_11,
    ) == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testPolygonToCells.c\0" as *const u8 as *const libc::c_char,
            504 as libc::c_int,
            b"H3_EXPORT(maxPolygonToCellsSize)(&lineGeoPolygon, 9, 0, &numHexagons) == E_FAILED\0"
                as *const u8 as *const libc::c_char,
            b"Cannot estimate for straight line\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
