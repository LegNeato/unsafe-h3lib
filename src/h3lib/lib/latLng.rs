use ::libc;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
    fn directedEdgeToBoundary(edge: H3Index, gb: *mut CellBoundary) -> H3Error;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn _ipow(base: int64_t, exp: int64_t) -> int64_t;
}
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
pub struct CellBoundary {
    pub numVerts: libc::c_int,
    pub verts: [LatLng; 10],
}
#[no_mangle]
pub unsafe extern "C" fn _posAngleRads(mut rads: libc::c_double) -> libc::c_double {
    let mut tmp: libc::c_double = if rads < 0.0 {
        rads + 6.28318530717958647692528676655900576839433
    } else {
        rads
    }
    .to_f64()
    .unwrap();
    if rads >= 6.28318530717958647692528676655900576839433 {
        tmp = (tmp - 6.28318530717958647692528676655900576839433)
            .to_f64()
            .unwrap();
    }
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn geoAlmostEqualThreshold(
    mut p1: *const LatLng,
    mut p2: *const LatLng,
    mut threshold: libc::c_double,
) -> bool {
    return fabs((*p1).lat - (*p2).lat) < threshold && fabs((*p1).lng - (*p2).lng) < threshold;
}
#[no_mangle]
pub unsafe extern "C" fn geoAlmostEqual(mut p1: *const LatLng, mut p2: *const LatLng) -> bool {
    return geoAlmostEqualThreshold(
        p1,
        p2,
        (0.000000001f64 * 0.0174532925199432957692369076848861271111)
            .to_f64()
            .unwrap(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn setGeoDegs(
    mut p: *mut LatLng,
    mut latDegs: libc::c_double,
    mut lngDegs: libc::c_double,
) {
    _setGeoRads(p, degsToRads(latDegs), degsToRads(lngDegs));
}
#[no_mangle]
pub unsafe extern "C" fn _setGeoRads(
    mut p: *mut LatLng,
    mut latRads: libc::c_double,
    mut lngRads: libc::c_double,
) {
    (*p).lat = latRads;
    (*p).lng = lngRads;
}
#[no_mangle]
pub unsafe extern "C" fn degsToRads(mut degrees: libc::c_double) -> libc::c_double {
    return (degrees * 0.0174532925199432957692369076848861271111)
        .to_f64()
        .unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn radsToDegs(mut radians: libc::c_double) -> libc::c_double {
    return (radians * 57.29577951308232087679815481410517033240547)
        .to_f64()
        .unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn constrainLat(mut lat: libc::c_double) -> libc::c_double {
    while lat > 1.57079632679489661923132169163975144f64 {
        lat = lat - 3.14159265358979323846264338327950288f64;
    }
    return lat;
}
#[no_mangle]
pub unsafe extern "C" fn constrainLng(mut lng: libc::c_double) -> libc::c_double {
    while lng > 3.14159265358979323846264338327950288f64 {
        lng = lng - 2 as libc::c_int as libc::c_double * 3.14159265358979323846264338327950288f64;
    }
    while lng < -3.14159265358979323846264338327950288f64 {
        lng = lng + 2 as libc::c_int as libc::c_double * 3.14159265358979323846264338327950288f64;
    }
    return lng;
}
#[no_mangle]
pub unsafe extern "C" fn greatCircleDistanceRads(
    mut a: *const LatLng,
    mut b: *const LatLng,
) -> libc::c_double {
    let mut sinLat: libc::c_double = sin(((*b).lat - (*a).lat) / 2.0f64);
    let mut sinLng: libc::c_double = sin(((*b).lng - (*a).lng) / 2.0f64);
    let mut A: libc::c_double = sinLat * sinLat + cos((*a).lat) * cos((*b).lat) * sinLng * sinLng;
    return 2 as libc::c_int as libc::c_double
        * atan2(sqrt(A), sqrt(1 as libc::c_int as libc::c_double - A));
}
#[no_mangle]
pub unsafe extern "C" fn greatCircleDistanceKm(
    mut a: *const LatLng,
    mut b: *const LatLng,
) -> libc::c_double {
    return (greatCircleDistanceRads(a, b) * 6371.007180918475)
        .to_f64()
        .unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn greatCircleDistanceM(
    mut a: *const LatLng,
    mut b: *const LatLng,
) -> libc::c_double {
    return greatCircleDistanceKm(a, b) * 1000 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn _geoAzimuthRads(
    mut p1: *const LatLng,
    mut p2: *const LatLng,
) -> libc::c_double {
    return atan2(
        cos((*p2).lat) * sin((*p2).lng - (*p1).lng),
        cos((*p1).lat) * sin((*p2).lat)
            - sin((*p1).lat) * cos((*p2).lat) * cos((*p2).lng - (*p1).lng),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _geoAzDistanceRads(
    mut p1: *const LatLng,
    mut az: libc::c_double,
    mut distance: libc::c_double,
    mut p2: *mut LatLng,
) {
    if distance < 0.0000000000000001 {
        *p2 = *p1;
        return;
    }
    let mut sinlat: libc::c_double = 0.;
    let mut sinlng: libc::c_double = 0.;
    let mut coslng: libc::c_double = 0.;
    az = _posAngleRads(az);
    if az < 0.0000000000000001
        || fabs(az - 3.14159265358979323846264338327950288f64) < 0.0000000000000001
    {
        if az < 0.0000000000000001 {
            (*p2).lat = (*p1).lat + distance;
        } else {
            (*p2).lat = (*p1).lat - distance;
        }
        if fabs((*p2).lat - 1.57079632679489661923132169163975144f64) < 0.0000000000000001 {
            (*p2).lat = 1.57079632679489661923132169163975144f64;
            (*p2).lng = 0.0f64;
        } else if fabs((*p2).lat + 1.57079632679489661923132169163975144f64) < 0.0000000000000001 {
            (*p2).lat = -1.57079632679489661923132169163975144f64;
            (*p2).lng = 0.0f64;
        } else {
            (*p2).lng = constrainLng((*p1).lng);
        }
    } else {
        sinlat = sin((*p1).lat) * cos(distance) + cos((*p1).lat) * sin(distance) * cos(az);
        if sinlat > 1.0f64 {
            sinlat = 1.0f64;
        }
        if sinlat < -1.0f64 {
            sinlat = -1.0f64;
        }
        (*p2).lat = asin(sinlat);
        if fabs((*p2).lat - 1.57079632679489661923132169163975144f64) < 0.0000000000000001 {
            (*p2).lat = 1.57079632679489661923132169163975144f64;
            (*p2).lng = 0.0f64;
        } else if fabs((*p2).lat + 1.57079632679489661923132169163975144f64) < 0.0000000000000001 {
            (*p2).lat = -1.57079632679489661923132169163975144f64;
            (*p2).lng = 0.0f64;
        } else {
            sinlng = sin(az) * sin(distance) / cos((*p2).lat);
            coslng =
                (cos(distance) - sin((*p1).lat) * sin((*p2).lat)) / cos((*p1).lat) / cos((*p2).lat);
            if sinlng > 1.0f64 {
                sinlng = 1.0f64;
            }
            if sinlng < -1.0f64 {
                sinlng = -1.0f64;
            }
            if coslng > 1.0f64 {
                coslng = 1.0f64;
            }
            if coslng < -1.0f64 {
                coslng = -1.0f64;
            }
            (*p2).lng = constrainLng((*p1).lng + atan2(sinlng, coslng));
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn getHexagonAreaAvgKm2(
    mut res: libc::c_int,
    mut out: *mut libc::c_double,
) -> H3Error {
    static mut areas: [libc::c_double; 16] = [
        4.357449416078383e+06f64,
        6.097884417941332e+05f64,
        8.680178039899720e+04f64,
        1.239343465508816e+04f64,
        1.770347654491307e+03f64,
        2.529038581819449e+02f64,
        3.612906216441245e+01f64,
        5.161293359717191e+00f64,
        7.373275975944177e-01f64,
        1.053325134272067e-01f64,
        1.504750190766435e-02f64,
        2.149643129451879e-03f64,
        3.070918756316060e-04f64,
        4.387026794728296e-05f64,
        6.267181135324313e-06f64,
        8.953115907605790e-07f64,
    ];
    if res < 0 as libc::c_int || res > 15 as libc::c_int {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    }
    *out = areas[res as usize];
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn getHexagonAreaAvgM2(
    mut res: libc::c_int,
    mut out: *mut libc::c_double,
) -> H3Error {
    static mut areas: [libc::c_double; 16] = [
        4.357449416078390e+12f64,
        6.097884417941339e+11f64,
        8.680178039899731e+10f64,
        1.239343465508818e+10f64,
        1.770347654491309e+09f64,
        2.529038581819452e+08f64,
        3.612906216441250e+07f64,
        5.161293359717198e+06f64,
        7.373275975944188e+05f64,
        1.053325134272069e+05f64,
        1.504750190766437e+04f64,
        2.149643129451882e+03f64,
        3.070918756316063e+02f64,
        4.387026794728301e+01f64,
        6.267181135324322e+00f64,
        8.953115907605802e-01f64,
    ];
    if res < 0 as libc::c_int || res > 15 as libc::c_int {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    }
    *out = areas[res as usize];
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn getHexagonEdgeLengthAvgKm(
    mut res: libc::c_int,
    mut out: *mut libc::c_double,
) -> H3Error {
    static mut lens: [libc::c_double; 16] = [
        1107.712591f64,
        418.6760055f64,
        158.2446558f64,
        59.81085794f64,
        22.6063794f64,
        8.544408276f64,
        3.229482772f64,
        1.220629759f64,
        0.461354684f64,
        0.174375668f64,
        0.065907807f64,
        0.024910561f64,
        0.009415526f64,
        0.003559893f64,
        0.001348575f64,
        0.000509713f64,
    ];
    if res < 0 as libc::c_int || res > 15 as libc::c_int {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    }
    *out = lens[res as usize];
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn getHexagonEdgeLengthAvgM(
    mut res: libc::c_int,
    mut out: *mut libc::c_double,
) -> H3Error {
    static mut lens: [libc::c_double; 16] = [
        1107712.591f64,
        418676.0055f64,
        158244.6558f64,
        59810.85794f64,
        22606.3794f64,
        8544.408276f64,
        3229.482772f64,
        1220.629759f64,
        461.3546837f64,
        174.3756681f64,
        65.90780749f64,
        24.9105614f64,
        9.415526211f64,
        3.559893033f64,
        1.348574562f64,
        0.509713273f64,
    ];
    if res < 0 as libc::c_int || res > 15 as libc::c_int {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    }
    *out = lens[res as usize];
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn getNumCells(mut res: libc::c_int, mut out: *mut int64_t) -> H3Error {
    if res < 0 as libc::c_int || res > 15 as libc::c_int {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    }
    *out = 2 as libc::c_int as libc::c_longlong
        + 120 as libc::c_int as libc::c_longlong
            * _ipow(7 as libc::c_int as int64_t, res as int64_t);
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn triangleEdgeLengthsToArea(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
) -> libc::c_double {
    let mut s: libc::c_double = (a + b + c) / 2 as libc::c_int as libc::c_double;
    a = (s - a) / 2 as libc::c_int as libc::c_double;
    b = (s - b) / 2 as libc::c_int as libc::c_double;
    c = (s - c) / 2 as libc::c_int as libc::c_double;
    s = s / 2 as libc::c_int as libc::c_double;
    return 4 as libc::c_int as libc::c_double * atan(sqrt(tan(s) * tan(a) * tan(b) * tan(c)));
}
#[no_mangle]
pub unsafe extern "C" fn triangleArea(
    mut a: *const LatLng,
    mut b: *const LatLng,
    mut c: *const LatLng,
) -> libc::c_double {
    return triangleEdgeLengthsToArea(
        greatCircleDistanceRads(a, b),
        greatCircleDistanceRads(b, c),
        greatCircleDistanceRads(c, a),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cellAreaRads2(mut cell: H3Index, mut out: *mut libc::c_double) -> H3Error {
    let mut c: LatLng = LatLng { lat: 0., lng: 0. };
    let mut cb: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut err: H3Error = cellToLatLng(cell, &mut c);
    if err != 0 {
        return err;
    }
    err = cellToBoundary(cell, &mut cb);
    if err != 0 {
        return err;
    }
    let mut area: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < cb.numVerts {
        let mut j: libc::c_int = (i + 1 as libc::c_int) % cb.numVerts;
        area += triangleArea(
            &mut *(cb.verts).as_mut_ptr().offset(i as isize),
            &mut *(cb.verts).as_mut_ptr().offset(j as isize),
            &mut c,
        );
        i += 1;
    }
    *out = area;
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn cellAreaKm2(mut cell: H3Index, mut out: *mut libc::c_double) -> H3Error {
    let mut err: H3Error = cellAreaRads2(cell, out);
    if err == 0 {
        *out = (*out * 6371.007180918475 * 6371.007180918475)
            .to_f64()
            .unwrap();
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn cellAreaM2(mut cell: H3Index, mut out: *mut libc::c_double) -> H3Error {
    let mut err: H3Error = cellAreaKm2(cell, out);
    if err == 0 {
        *out = *out * 1000 as libc::c_int as libc::c_double * 1000 as libc::c_int as libc::c_double;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn edgeLengthRads(
    mut edge: H3Index,
    mut length: *mut libc::c_double,
) -> H3Error {
    let mut cb: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    let mut err: H3Error = directedEdgeToBoundary(edge, &mut cb);
    if err != 0 {
        return err;
    }
    *length = 0.0f64;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < cb.numVerts - 1 as libc::c_int {
        *length += greatCircleDistanceRads(
            &mut *(cb.verts).as_mut_ptr().offset(i as isize),
            &mut *(cb.verts)
                .as_mut_ptr()
                .offset((i + 1 as libc::c_int) as isize),
        );
        i += 1;
    }
    return E_SUCCESS as libc::c_int as H3Error;
}
#[no_mangle]
pub unsafe extern "C" fn edgeLengthKm(
    mut edge: H3Index,
    mut length: *mut libc::c_double,
) -> H3Error {
    let mut err: H3Error = edgeLengthRads(edge, length);
    *length = (*length * 6371.007180918475).to_f64().unwrap();
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn edgeLengthM(
    mut edge: H3Index,
    mut length: *mut libc::c_double,
) -> H3Error {
    let mut err: H3Error = edgeLengthKm(edge, length);
    *length = *length * 1000 as libc::c_int as libc::c_double;
    return err;
}
