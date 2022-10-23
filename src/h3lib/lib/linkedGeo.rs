use ::libc;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn bboxContains(bbox: *const BBox, point: *const LatLng) -> bool;
    fn bboxIsTransmeridian(bbox: *const BBox) -> bool;
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    fn test_prefix_malloc(size: size_t) -> *mut libc::c_void;
    fn test_prefix_calloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn test_prefix_free(ptr: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type uint32_t = libc::c_uint;
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
#[no_mangle]
pub unsafe extern "C" fn addNewLinkedPolygon(
    mut polygon: *mut LinkedGeoPolygon,
) -> *mut LinkedGeoPolygon {
    if !((*polygon).next).is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"addNewLinkedPolygon\0"))
                .as_ptr(),
            b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            b"polygon->next == NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    let mut next: *mut LinkedGeoPolygon = test_prefix_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<LinkedGeoPolygon>() as libc::c_ulong,
    ) as *mut LinkedGeoPolygon;
    if next.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"addNewLinkedPolygon\0"))
                .as_ptr(),
            b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            b"next != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    (*polygon).next = next;
    next
}
#[no_mangle]
pub unsafe extern "C" fn addNewLinkedLoop(
    mut polygon: *mut LinkedGeoPolygon,
) -> *mut LinkedGeoLoop {
    let mut loop_0: *mut LinkedGeoLoop = test_prefix_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<LinkedGeoLoop>() as libc::c_ulong,
    ) as *mut LinkedGeoLoop;
    if loop_0.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"addNewLinkedLoop\0"))
                .as_ptr(),
            b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            b"loop != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    addLinkedLoop(polygon, loop_0)
}
#[no_mangle]
pub unsafe extern "C" fn addLinkedLoop(
    mut polygon: *mut LinkedGeoPolygon,
    mut loop_0: *mut LinkedGeoLoop,
) -> *mut LinkedGeoLoop {
    let mut last: *mut LinkedGeoLoop = (*polygon).last;
    if last.is_null() {
        if !((*polygon).first).is_null() as libc::c_int as libc::c_long != 0 {
            __assert_rtn(
                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"addLinkedLoop\0"))
                    .as_ptr(),
                b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int,
                b"polygon->first == NULL\0" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        (*polygon).first = loop_0;
    } else {
        (*last).next = loop_0;
    }
    (*polygon).last = loop_0;
    loop_0
}
#[no_mangle]
pub unsafe extern "C" fn addLinkedCoord(
    mut loop_0: *mut LinkedGeoLoop,
    mut vertex: *const LatLng,
) -> *mut LinkedLatLng {
    let mut coord: *mut LinkedLatLng =
        test_prefix_malloc(::core::mem::size_of::<LinkedLatLng>() as libc::c_ulong)
            as *mut LinkedLatLng;
    if coord.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"addLinkedCoord\0"))
                .as_ptr(),
            b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int,
            b"coord != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    *coord = {
        let mut init = LinkedLatLng {
            vertex: *vertex,
            next: std::ptr::null_mut::<LinkedLatLng>(),
        };
        init
    };
    let mut last: *mut LinkedLatLng = (*loop_0).last;
    if last.is_null() {
        if !((*loop_0).first).is_null() as libc::c_int as libc::c_long != 0 {
            __assert_rtn(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"addLinkedCoord\0"))
                    .as_ptr(),
                b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int,
                b"loop->first == NULL\0" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        (*loop_0).first = coord;
    } else {
        (*last).next = coord;
    }
    (*loop_0).last = coord;
    coord
}
#[no_mangle]
pub unsafe extern "C" fn destroyLinkedGeoLoop(mut loop_0: *mut LinkedGeoLoop) {
    let mut nextCoord: *mut LinkedLatLng = std::ptr::null_mut::<LinkedLatLng>();
    let mut currentCoord: *mut LinkedLatLng = (*loop_0).first;
    while !currentCoord.is_null() {
        nextCoord = (*currentCoord).next;
        test_prefix_free(currentCoord as *mut libc::c_void);
        currentCoord = nextCoord;
    }
}
#[no_mangle]
pub unsafe extern "C" fn destroyLinkedMultiPolygon(mut polygon: *mut LinkedGeoPolygon) {
    let mut skip: bool = 1 as libc::c_int != 0;
    let mut nextPolygon: *mut LinkedGeoPolygon = std::ptr::null_mut::<LinkedGeoPolygon>();
    let mut nextLoop: *mut LinkedGeoLoop = std::ptr::null_mut::<LinkedGeoLoop>();
    let mut currentPolygon: *mut LinkedGeoPolygon = polygon;
    while !currentPolygon.is_null() {
        let mut currentLoop: *mut LinkedGeoLoop = (*currentPolygon).first;
        while !currentLoop.is_null() {
            destroyLinkedGeoLoop(currentLoop);
            nextLoop = (*currentLoop).next;
            test_prefix_free(currentLoop as *mut libc::c_void);
            currentLoop = nextLoop;
        }
        nextPolygon = (*currentPolygon).next;
        if skip {
            skip = 0 as libc::c_int != 0;
        } else {
            test_prefix_free(currentPolygon as *mut libc::c_void);
        }
        currentPolygon = nextPolygon;
    }
}
#[no_mangle]
pub unsafe extern "C" fn countLinkedPolygons(mut polygon: *mut LinkedGeoPolygon) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    while !polygon.is_null() {
        count += 1;
        polygon = (*polygon).next;
    }
    count
}
#[no_mangle]
pub unsafe extern "C" fn countLinkedLoops(mut polygon: *mut LinkedGeoPolygon) -> libc::c_int {
    let mut loop_0: *mut LinkedGeoLoop = (*polygon).first;
    let mut count: libc::c_int = 0 as libc::c_int;
    while !loop_0.is_null() {
        count += 1;
        loop_0 = (*loop_0).next;
    }
    count
}
#[no_mangle]
pub unsafe extern "C" fn countLinkedCoords(mut loop_0: *mut LinkedGeoLoop) -> libc::c_int {
    let mut coord: *mut LinkedLatLng = (*loop_0).first;
    let mut count: libc::c_int = 0 as libc::c_int;
    while !coord.is_null() {
        count += 1;
        coord = (*coord).next;
    }
    count
}
unsafe extern "C" fn countContainers(
    mut loop_0: *const LinkedGeoLoop,
    mut polygons: *mut *const LinkedGeoPolygon,
    mut bboxes: *mut *const BBox,
    polygonCount: libc::c_int,
) -> libc::c_int {
    let mut containerCount: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < polygonCount {
        if loop_0 != (**polygons.offset(i as isize)).first as *const LinkedGeoLoop
            && pointInsideLinkedGeoLoop(
                (**polygons.offset(i as isize)).first,
                *bboxes.offset(i as isize),
                &mut (*(*loop_0).first).vertex,
            ) as libc::c_int
                != 0
        {
            containerCount += 1;
        }
        i += 1;
    }
    containerCount
}
unsafe extern "C" fn findDeepestContainer(
    mut polygons: *mut *const LinkedGeoPolygon,
    mut bboxes: *mut *const BBox,
    polygonCount: libc::c_int,
) -> *const LinkedGeoPolygon {
    let mut parent: *const LinkedGeoPolygon = if polygonCount > 0 as libc::c_int {
        *polygons.offset(0 as libc::c_int as isize)
    } else {
        std::ptr::null::<LinkedGeoPolygon>()
    };
    if polygonCount > 1 as libc::c_int {
        let mut max: libc::c_int = -(1 as libc::c_int);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < polygonCount {
            let mut count: libc::c_int = countContainers(
                (**polygons.offset(i as isize)).first,
                polygons,
                bboxes,
                polygonCount,
            );
            if count > max {
                parent = *polygons.offset(i as isize);
                max = count;
            }
            i += 1;
        }
    }
    parent
}
unsafe extern "C" fn findPolygonForHole(
    mut loop_0: *const LinkedGeoLoop,
    mut polygon: *const LinkedGeoPolygon,
    mut bboxes: *const BBox,
    polygonCount: libc::c_int,
) -> *const LinkedGeoPolygon {
    if polygonCount == 0 as libc::c_int {
        return std::ptr::null::<LinkedGeoPolygon>();
    }
    let mut candidates: *mut *const LinkedGeoPolygon = test_prefix_malloc(
        (polygonCount as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut LinkedGeoPolygon>() as libc::c_ulong),
    ) as *mut *const LinkedGeoPolygon;
    if candidates.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"findPolygonForHole\0"))
                .as_ptr(),
            b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
            249 as libc::c_int,
            b"candidates != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    let mut candidateBBoxes: *mut *const BBox = test_prefix_malloc(
        (polygonCount as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut BBox>() as libc::c_ulong),
    ) as *mut *const BBox;
    if candidateBBoxes.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"findPolygonForHole\0"))
                .as_ptr(),
            b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int,
            b"candidateBBoxes != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    let mut candidateCount: libc::c_int = 0 as libc::c_int;
    let mut index: libc::c_int = 0 as libc::c_int;
    while !polygon.is_null() {
        if pointInsideLinkedGeoLoop(
            (*polygon).first,
            &*bboxes.offset(index as isize),
            &mut (*(*loop_0).first).vertex,
        ) {
            let fresh0 = &mut (*candidates.offset(candidateCount as isize));
            *fresh0 = polygon;
            let fresh1 = &mut (*candidateBBoxes.offset(candidateCount as isize));
            *fresh1 = &*bboxes.offset(index as isize) as *const BBox;
            candidateCount += 1;
        }
        polygon = (*polygon).next;
        index += 1;
    }
    let mut parent: *const LinkedGeoPolygon =
        findDeepestContainer(candidates, candidateBBoxes, candidateCount);
    test_prefix_free(candidates as *mut libc::c_void);
    test_prefix_free(candidateBBoxes as *mut libc::c_void);
    parent
}
#[no_mangle]
pub unsafe extern "C" fn normalizeMultiPolygon(mut root: *mut LinkedGeoPolygon) -> H3Error {
    if !((*root).next).is_null() {
        return E_FAILED as libc::c_int as H3Error;
    }
    let mut loopCount: libc::c_int = countLinkedLoops(root);
    if loopCount <= 1 as libc::c_int {
        return E_SUCCESS as libc::c_int as H3Error;
    }
    let mut resultCode: H3Error = E_SUCCESS as libc::c_int as H3Error;
    let mut polygon: *mut LinkedGeoPolygon = std::ptr::null_mut::<LinkedGeoPolygon>();
    let mut next: *mut LinkedGeoLoop = std::ptr::null_mut::<LinkedGeoLoop>();
    let mut innerCount: libc::c_int = 0 as libc::c_int;
    let mut outerCount: libc::c_int = 0 as libc::c_int;
    let mut innerLoops: *mut *mut LinkedGeoLoop = test_prefix_malloc(
        (loopCount as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut LinkedGeoLoop>() as libc::c_ulong),
    ) as *mut *mut LinkedGeoLoop;
    if innerLoops.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"normalizeMultiPolygon\0"))
                .as_ptr(),
            b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            b"innerLoops != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    let mut bboxes: *mut BBox = test_prefix_malloc(
        (loopCount as libc::c_ulong).wrapping_mul(::core::mem::size_of::<BBox>() as libc::c_ulong),
    ) as *mut BBox;
    if bboxes.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn(
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"normalizeMultiPolygon\0"))
                .as_ptr(),
            b"linkedGeo.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int,
            b"bboxes != NULL\0" as *const u8 as *const libc::c_char,
        );
    } else {
    };
    let mut loop_0: *mut LinkedGeoLoop = (*root).first;
    *root = {
        let mut init = LinkedGeoPolygon {
            first: std::ptr::null_mut::<LinkedGeoLoop>(),
            last: std::ptr::null_mut::<LinkedGeoLoop>(),
            next: std::ptr::null_mut::<LinkedGeoPolygon>(),
        };
        init
    };
    while !loop_0.is_null() {
        if isClockwiseLinkedGeoLoop(loop_0) {
            let fresh2 = &mut (*innerLoops.offset(innerCount as isize));
            *fresh2 = loop_0;
            innerCount += 1;
        } else {
            polygon = if polygon.is_null() {
                root
            } else {
                addNewLinkedPolygon(polygon)
            };
            addLinkedLoop(polygon, loop_0);
            bboxFromLinkedGeoLoop(loop_0, &mut *bboxes.offset(outerCount as isize));
            outerCount += 1;
        }
        next = (*loop_0).next;
        (*loop_0).next = std::ptr::null_mut::<LinkedGeoLoop>();
        loop_0 = next;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < innerCount {
        polygon = findPolygonForHole(*innerLoops.offset(i as isize), root, bboxes, outerCount)
            as *mut LinkedGeoPolygon;
        if !polygon.is_null() {
            addLinkedLoop(polygon, *innerLoops.offset(i as isize));
        } else {
            destroyLinkedGeoLoop(*innerLoops.offset(i as isize));
            test_prefix_free(*innerLoops.offset(i as isize) as *mut libc::c_void);
            resultCode = E_FAILED as libc::c_int as H3Error;
        }
        i += 1;
    }
    test_prefix_free(innerLoops as *mut libc::c_void);
    test_prefix_free(bboxes as *mut libc::c_void);
    resultCode
}
#[no_mangle]
pub unsafe extern "C" fn isClockwiseLinkedGeoLoop(mut loop_0: *const LinkedGeoLoop) -> bool {
    isClockwiseNormalizedLinkedGeoLoop(loop_0, 0 as libc::c_int != 0)
}
#[no_mangle]
pub unsafe extern "C" fn bboxFromLinkedGeoLoop(
    mut loop_0: *const LinkedGeoLoop,
    mut bbox: *mut BBox,
) {
    if ((*loop_0).first).is_null() {
        *bbox = {
            
            BBox {
                north: 0 as libc::c_int as libc::c_double,
                south: 0.,
                east: 0.,
                west: 0.,
            }
        };
        return;
    }
    (*bbox).south = 1.797_693_134_862_315_7e308_f64;
    (*bbox).west = 1.797_693_134_862_315_7e308_f64;
    (*bbox).north = -1.797_693_134_862_315_7e308_f64;
    (*bbox).east = -1.797_693_134_862_315_7e308_f64;
    let mut minPosLng: libc::c_double = 1.797_693_134_862_315_7e308_f64;
    let mut maxNegLng: libc::c_double = -1.797_693_134_862_315_7e308_f64;
    let mut isTransmeridian: bool = 0 as libc::c_int != 0;
    let mut lat: libc::c_double = 0.;
    let mut lng: libc::c_double = 0.;
    let mut coord: LatLng = LatLng { lat: 0., lng: 0. };
    let mut next: LatLng = LatLng { lat: 0., lng: 0. };
    let mut currentCoord: *mut LinkedLatLng = std::ptr::null_mut::<LinkedLatLng>();
    let mut nextCoord: *mut LinkedLatLng = std::ptr::null_mut::<LinkedLatLng>();
    loop {
        currentCoord = if currentCoord.is_null() {
            (*loop_0).first
        } else {
            (*currentCoord).next
        };
        if currentCoord.is_null() {
            break;
        }
        coord = (*currentCoord).vertex;
        nextCoord = if ((*currentCoord).next).is_null() {
            (*loop_0).first
        } else {
            (*currentCoord).next
        };
        next = (*nextCoord).vertex;
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
        if fabs(lng - next.lng) > 3.141_592_653_589_793_f64 {
            isTransmeridian = 1 as libc::c_int != 0;
        }
    }
    if isTransmeridian {
        (*bbox).east = maxNegLng;
        (*bbox).west = minPosLng;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pointInsideLinkedGeoLoop(
    mut loop_0: *const LinkedGeoLoop,
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
            + 6.283_185_307_179_586
                .to_f64()
                .unwrap()
    } else {
        (*coord).lng
    };
    let mut a: LatLng = LatLng { lat: 0., lng: 0. };
    let mut b: LatLng = LatLng { lat: 0., lng: 0. };
    let mut currentCoord: *mut LinkedLatLng = std::ptr::null_mut::<LinkedLatLng>();
    let mut nextCoord: *mut LinkedLatLng = std::ptr::null_mut::<LinkedLatLng>();
    loop {
        currentCoord = if currentCoord.is_null() {
            (*loop_0).first
        } else {
            (*currentCoord).next
        };
        if currentCoord.is_null() {
            break;
        }
        a = (*currentCoord).vertex;
        nextCoord = if ((*currentCoord).next).is_null() {
            (*loop_0).first
        } else {
            (*currentCoord).next
        };
        b = (*nextCoord).vertex;
        if a.lat > b.lat {
            std::mem::swap(&mut a, &mut b);
        }
        if lat == a.lat || lat == b.lat {
            lat += 2.220_446_049_250_313e-16_f64;
        }
        if lat < a.lat || lat > b.lat {
            continue;
        }
        let mut aLng: libc::c_double =
            if isTransmeridian as libc::c_int != 0 && a.lng < 0 as libc::c_int as libc::c_double {
                a.lng
                    + 6.283_185_307_179_586
                        .to_f64()
                        .unwrap()
            } else {
                a.lng
            };
        let mut bLng: libc::c_double =
            if isTransmeridian as libc::c_int != 0 && b.lng < 0 as libc::c_int as libc::c_double {
                b.lng
                    + 6.283_185_307_179_586
                        .to_f64()
                        .unwrap()
            } else {
                b.lng
            };
        if aLng == lng || bLng == lng {
            lng -= 2.220_446_049_250_313e-16_f64;
        }
        let mut ratio: libc::c_double = (lat - a.lat) / (b.lat - a.lat);
        let mut testLng: libc::c_double = if isTransmeridian as libc::c_int != 0
            && aLng + (bLng - aLng) * ratio < 0 as libc::c_int as libc::c_double
        {
            aLng + (bLng - aLng) * ratio
                + 6.283_185_307_179_586
                    .to_f64()
                    .unwrap()
        } else {
            aLng + (bLng - aLng) * ratio
        };
        if testLng > lng {
            contains = !contains;
        }
    }
    contains
}
unsafe extern "C" fn isClockwiseNormalizedLinkedGeoLoop(
    mut loop_0: *const LinkedGeoLoop,
    mut isTransmeridian: bool,
) -> bool {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut a: LatLng = LatLng { lat: 0., lng: 0. };
    let mut b: LatLng = LatLng { lat: 0., lng: 0. };
    let mut currentCoord: *mut LinkedLatLng = std::ptr::null_mut::<LinkedLatLng>();
    let mut nextCoord: *mut LinkedLatLng = std::ptr::null_mut::<LinkedLatLng>();
    loop {
        currentCoord = if currentCoord.is_null() {
            (*loop_0).first
        } else {
            (*currentCoord).next
        };
        if currentCoord.is_null() {
            break;
        }
        a = (*currentCoord).vertex;
        nextCoord = if ((*currentCoord).next).is_null() {
            (*loop_0).first
        } else {
            (*currentCoord).next
        };
        b = (*nextCoord).vertex;
        if !isTransmeridian && fabs(a.lng - b.lng) > 3.141_592_653_589_793_f64 {
            return isClockwiseNormalizedLinkedGeoLoop(loop_0, 1 as libc::c_int != 0);
        }
        sum += ((if isTransmeridian as libc::c_int != 0
            && b.lng < 0 as libc::c_int as libc::c_double
        {
            b.lng
                + 6.283_185_307_179_586
                    .to_f64()
                    .unwrap()
        } else {
            b.lng
        }) - (if isTransmeridian as libc::c_int != 0
            && a.lng < 0 as libc::c_int as libc::c_double
        {
            a.lng
                + 6.283_185_307_179_586
                    .to_f64()
                    .unwrap()
        } else {
            a.lng
        })) * (b.lat + a.lat);
    }
    sum > 0 as libc::c_int as libc::c_double
}
