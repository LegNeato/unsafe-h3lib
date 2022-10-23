use ::libc;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
    fn greatCircleDistanceKm(a: *const LatLng, b: *const LatLng) -> libc::c_double;
    fn constrainLng(lng: libc::c_double) -> libc::c_double;
    fn getPentagons(res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BBox {
    pub north: libc::c_double,
    pub south: libc::c_double,
    pub east: libc::c_double,
    pub west: libc::c_double,
}
#[inline(always)]
unsafe extern "C" fn __inline_isfinitef(mut __x: libc::c_float) -> libc::c_int {
    (__x.abs() != ::core::f32::INFINITY) as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn __inline_isfinited(mut __x: libc::c_double) -> libc::c_int {
    (__x.abs() != ::core::f64::INFINITY) as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn __inline_isfinitel(mut __x: f64) -> libc::c_int {
    (__x.abs() != ::core::f64::INFINITY) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn bboxIsTransmeridian(mut bbox: *const BBox) -> bool {
    (*bbox).east < (*bbox).west
}
#[no_mangle]
pub unsafe extern "C" fn bboxCenter(mut bbox: *const BBox, mut center: *mut LatLng) {
    (*center).lat = ((*bbox).north + (*bbox).south) / 2.0f64;
    let mut east: libc::c_double = if bboxIsTransmeridian(bbox) as libc::c_int != 0 {
        (*bbox).east + std::f64::consts::TAU
    } else {
        (*bbox).east
    }
    .to_f64()
    .unwrap();
    (*center).lng = constrainLng((east + (*bbox).west) / 2.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn bboxContains(mut bbox: *const BBox, mut point: *const LatLng) -> bool {
    (*point).lat >= (*bbox).south
        && (*point).lat <= (*bbox).north
        && (if bboxIsTransmeridian(bbox) as libc::c_int != 0 {
            ((*point).lng >= (*bbox).west || (*point).lng <= (*bbox).east) as libc::c_int
        } else {
            ((*point).lng >= (*bbox).west && (*point).lng <= (*bbox).east) as libc::c_int
        }) != 0
}
#[no_mangle]
pub unsafe extern "C" fn bboxEquals(mut b1: *const BBox, mut b2: *const BBox) -> bool {
    (*b1).north == (*b2).north
        && (*b1).south == (*b2).south
        && (*b1).east == (*b2).east
        && (*b1).west == (*b2).west
}
#[no_mangle]
pub unsafe extern "C" fn _hexRadiusKm(mut h3Index: H3Index) -> libc::c_double {
    let mut h3Center: LatLng = LatLng { lat: 0., lng: 0. };
    let mut h3Boundary: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    cellToLatLng(h3Index, &mut h3Center);
    cellToBoundary(h3Index, &mut h3Boundary);
    greatCircleDistanceKm(&h3Center, (h3Boundary.verts).as_mut_ptr())
}
#[no_mangle]
pub unsafe extern "C" fn bboxHexEstimate(
    mut bbox: *const BBox,
    mut res: libc::c_int,
    mut out: *mut int64_t,
) -> H3Error {
    let mut pentagons: [H3Index; 12] =
        [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut pentagonsErr: H3Error = getPentagons(res, pentagons.as_mut_ptr());
    if pentagonsErr != 0 {
        return pentagonsErr;
    }
    let mut pentagonRadiusKm: libc::c_double = _hexRadiusKm(pentagons[0 as libc::c_int as usize]);
    let mut pentagonAreaKm2: libc::c_double =
        0.8f64 * (2.59807621135f64 * pentagonRadiusKm * pentagonRadiusKm);
    let mut p1: LatLng = LatLng { lat: 0., lng: 0. };
    let mut p2: LatLng = LatLng { lat: 0., lng: 0. };
    p1.lat = (*bbox).north;
    p1.lng = (*bbox).east;
    p2.lat = (*bbox).south;
    p2.lng = (*bbox).west;
    let mut d: libc::c_double = greatCircleDistanceKm(&p1, &p2);
    let mut lngDiff: libc::c_double = p1.lng - p2.lng;
    let mut latDiff: libc::c_double = p1.lat - p2.lat;
    if lngDiff == 0 as libc::c_int as libc::c_double
        || latDiff == 0 as libc::c_int as libc::c_double
    {
        return E_FAILED as libc::c_int as H3Error;
    }
    let mut a: libc::c_double = d * d / fmin(3.0f64, fabs(lngDiff / latDiff));
    let mut estimateDouble: libc::c_double = ceil(a / pentagonAreaKm2);
    if if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
    {
        __inline_isfinitef(estimateDouble as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
    {
        __inline_isfinited(estimateDouble)
    } else {
        __inline_isfinitel(estimateDouble)
    } == 0
    {
        return E_FAILED as libc::c_int as H3Error;
    }
    let mut estimate: int64_t = estimateDouble as int64_t;
    if estimate == 0 as libc::c_int as libc::c_longlong {
        estimate = 1 as libc::c_int as int64_t;
    }
    *out = estimate;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn lineHexEstimate(
    mut origin: *const LatLng,
    mut destination: *const LatLng,
    mut res: libc::c_int,
    mut out: *mut int64_t,
) -> H3Error {
    let mut pentagons: [H3Index; 12] =
        [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut pentagonsErr: H3Error = getPentagons(res, pentagons.as_mut_ptr());
    if pentagonsErr != 0 {
        return pentagonsErr;
    }
    let mut pentagonRadiusKm: libc::c_double = _hexRadiusKm(pentagons[0 as libc::c_int as usize]);
    let mut dist: libc::c_double = greatCircleDistanceKm(origin, destination);
    let mut distCeil: libc::c_double =
        ceil(dist / (2 as libc::c_int as libc::c_double * pentagonRadiusKm));
    if if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
    {
        __inline_isfinitef(distCeil as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
    {
        __inline_isfinited(distCeil)
    } else {
        __inline_isfinitel(distCeil)
    } == 0
    {
        return E_FAILED as libc::c_int as H3Error;
    }
    let mut estimate: int64_t = distCeil as int64_t;
    if estimate == 0 as libc::c_int as libc::c_longlong {
        estimate = 1 as libc::c_int as int64_t;
    }
    *out = estimate;
    E_SUCCESS as libc::c_int as H3Error
}
