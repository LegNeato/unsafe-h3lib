use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3d {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _square(mut x: libc::c_double) -> libc::c_double {
    x * x
}
#[no_mangle]
pub unsafe extern "C" fn _pointSquareDist(
    mut v1: *const Vec3d,
    mut v2: *const Vec3d,
) -> libc::c_double {
    _square((*v1).x - (*v2).x) + _square((*v1).y - (*v2).y) + _square((*v1).z - (*v2).z)
}
#[no_mangle]
pub unsafe extern "C" fn _geoToVec3d(mut geo: *const LatLng, mut v: *mut Vec3d) {
    let mut r: libc::c_double = cos((*geo).lat);
    (*v).z = sin((*geo).lat);
    (*v).x = cos((*geo).lng) * r;
    (*v).y = sin((*geo).lng) * r;
}
