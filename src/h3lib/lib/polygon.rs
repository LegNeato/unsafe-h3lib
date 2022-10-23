use ::libc;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn bboxContains(bbox: *const BBox, point: *const LatLng) -> bool;
    fn bboxIsTransmeridian(bbox: *const BBox) -> bool;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
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
pub struct BBox {
    pub north: libc::c_double,
    pub south: libc::c_double,
    pub east: libc::c_double,
    pub west: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn pointInsideGeoLoop(
    mut loop_0: *const GeoLoop,
    mut bbox: *const BBox,
    mut coord: *const LatLng,
) -> bool {
    if !bboxContains(bbox, coord) {
        return 0 as libc::c_int != 0;
    }
    let mut isTransmeridian: bool = bboxIsTransmeridian(bbox);
    let mut contains: bool = 0 as libc::c_int != 0;
    let mut lat: libc::c_double = (*coord).lat;
    let mut lng: libc::c_double = if isTransmeridian as libc::c_int != 0
        && (*coord).lng < 0 as libc::c_int as libc::c_double
    {
        (*coord).lng
            + (6.28318530717958647692528676655900576839433)
                .to_f64()
                .unwrap()
    } else {
        (*coord).lng
    };
    let mut a: LatLng = LatLng { lat: 0., lng: 0. };
    let mut b: LatLng = LatLng { lat: 0., lng: 0. };
    let mut loopIndex: libc::c_int = -(1 as libc::c_int);
    loop {
        loopIndex += 1;
        if loopIndex >= (*loop_0).numVerts {
            break;
        }
        a = *((*loop_0).verts).offset(loopIndex as isize);
        b = *((*loop_0).verts)
            .offset(((loopIndex + 1 as libc::c_int) % (*loop_0).numVerts) as isize);
        if a.lat > b.lat {
            let mut tmp: LatLng = a;
            a = b;
            b = tmp;
        }
        if lat == a.lat || lat == b.lat {
            lat += 2.2204460492503131e-16f64;
        }
        if lat < a.lat || lat > b.lat {
            continue;
        }
        let mut aLng: libc::c_double =
            if isTransmeridian as libc::c_int != 0 && a.lng < 0 as libc::c_int as libc::c_double {
                a.lng
                    + (6.28318530717958647692528676655900576839433)
                        .to_f64()
                        .unwrap()
            } else {
                a.lng
            };
        let mut bLng: libc::c_double =
            if isTransmeridian as libc::c_int != 0 && b.lng < 0 as libc::c_int as libc::c_double {
                b.lng
                    + (6.28318530717958647692528676655900576839433)
                        .to_f64()
                        .unwrap()
            } else {
                b.lng
            };
        if aLng == lng || bLng == lng {
            lng -= 2.2204460492503131e-16f64;
        }
        let mut ratio: libc::c_double = (lat - a.lat) / (b.lat - a.lat);
        let mut testLng: libc::c_double = if isTransmeridian as libc::c_int != 0
            && aLng + (bLng - aLng) * ratio < 0 as libc::c_int as libc::c_double
        {
            aLng + (bLng - aLng) * ratio
                + (6.28318530717958647692528676655900576839433)
                    .to_f64()
                    .unwrap()
        } else {
            aLng + (bLng - aLng) * ratio
        };
        if testLng > lng {
            contains = !contains;
        }
    }
    return contains;
}
#[no_mangle]
pub unsafe extern "C" fn bboxFromGeoLoop(mut loop_0: *const GeoLoop, mut bbox: *mut BBox) {
    if (*loop_0).numVerts == 0 as libc::c_int {
        *bbox = {
            let mut init = BBox {
                north: 0 as libc::c_int as libc::c_double,
                south: 0.,
                east: 0.,
                west: 0.,
            };
            init
        };
        return;
    }
    (*bbox).south = 1.7976931348623157e+308f64;
    (*bbox).west = 1.7976931348623157e+308f64;
    (*bbox).north = -1.7976931348623157e+308f64;
    (*bbox).east = -1.7976931348623157e+308f64;
    let mut minPosLng: libc::c_double = 1.7976931348623157e+308f64;
    let mut maxNegLng: libc::c_double = -1.7976931348623157e+308f64;
    let mut isTransmeridian: bool = 0 as libc::c_int != 0;
    let mut lat: libc::c_double = 0.;
    let mut lng: libc::c_double = 0.;
    let mut coord: LatLng = LatLng { lat: 0., lng: 0. };
    let mut next: LatLng = LatLng { lat: 0., lng: 0. };
    let mut loopIndex: libc::c_int = -(1 as libc::c_int);
    loop {
        loopIndex += 1;
        if loopIndex >= (*loop_0).numVerts {
            break;
        }
        coord = *((*loop_0).verts).offset(loopIndex as isize);
        next = *((*loop_0).verts)
            .offset(((loopIndex + 1 as libc::c_int) % (*loop_0).numVerts) as isize);
        lat = coord.lat;
        lng = coord.lng;
        if lat < (*bbox).south {
            (*bbox).south = lat;
        }
        if lng < (*bbox).west {
            (*bbox).west = lng;
        }
        if lat > (*bbox).north {
            (*bbox).north = lat;
        }
        if lng > (*bbox).east {
            (*bbox).east = lng;
        }
        if lng > 0 as libc::c_int as libc::c_double && lng < minPosLng {
            minPosLng = lng;
        }
        if lng < 0 as libc::c_int as libc::c_double && lng > maxNegLng {
            maxNegLng = lng;
        }
        if fabs(lng - next.lng) > 3.14159265358979323846264338327950288f64 {
            isTransmeridian = 1 as libc::c_int != 0;
        }
    }
    if isTransmeridian {
        (*bbox).east = maxNegLng;
        (*bbox).west = minPosLng;
    }
}
unsafe extern "C" fn isClockwiseNormalizedGeoLoop(
    mut loop_0: *const GeoLoop,
    mut isTransmeridian: bool,
) -> bool {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut a: LatLng = LatLng { lat: 0., lng: 0. };
    let mut b: LatLng = LatLng { lat: 0., lng: 0. };
    let mut loopIndex: libc::c_int = -(1 as libc::c_int);
    loop {
        loopIndex += 1;
        if loopIndex >= (*loop_0).numVerts {
            break;
        }
        a = *((*loop_0).verts).offset(loopIndex as isize);
        b = *((*loop_0).verts)
            .offset(((loopIndex + 1 as libc::c_int) % (*loop_0).numVerts) as isize);
        if !isTransmeridian && fabs(a.lng - b.lng) > 3.14159265358979323846264338327950288f64 {
            return isClockwiseNormalizedGeoLoop(loop_0, 1 as libc::c_int != 0);
        }
        sum += ((if isTransmeridian as libc::c_int != 0
            && b.lng < 0 as libc::c_int as libc::c_double
        {
            b.lng
                + (6.28318530717958647692528676655900576839433)
                    .to_f64()
                    .unwrap()
        } else {
            b.lng
        }) - (if isTransmeridian as libc::c_int != 0
            && a.lng < 0 as libc::c_int as libc::c_double
        {
            a.lng
                + (6.28318530717958647692528676655900576839433)
                    .to_f64()
                    .unwrap()
        } else {
            a.lng
        })) * (b.lat + a.lat);
    }
    return sum > 0 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn isClockwiseGeoLoop(mut loop_0: *const GeoLoop) -> bool {
    return isClockwiseNormalizedGeoLoop(loop_0, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn bboxesFromGeoPolygon(
    mut polygon: *const GeoPolygon,
    mut bboxes: *mut BBox,
) {
    bboxFromGeoLoop(
        &(*polygon).geoloop,
        &mut *bboxes.offset(0 as libc::c_int as isize),
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*polygon).numHoles {
        bboxFromGeoLoop(
            &mut *((*polygon).holes).offset(i as isize),
            &mut *bboxes.offset((i + 1 as libc::c_int) as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pointInsidePolygon(
    mut geoPolygon: *const GeoPolygon,
    mut bboxes: *const BBox,
    mut coord: *const LatLng,
) -> bool {
    let mut contains: bool = pointInsideGeoLoop(
        &(*geoPolygon).geoloop,
        &*bboxes.offset(0 as libc::c_int as isize),
        coord,
    );
    if contains as libc::c_int != 0 && (*geoPolygon).numHoles > 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*geoPolygon).numHoles {
            if pointInsideGeoLoop(
                &mut *((*geoPolygon).holes).offset(i as isize),
                &*bboxes.offset((i + 1 as libc::c_int) as isize),
                coord,
            ) {
                return 0 as libc::c_int != 0;
            }
            i += 1;
        }
    }
    return contains;
}
