use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec2d {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _v2dMag(mut v: *const Vec2d) -> libc::c_double {
    return sqrt((*v).x * (*v).x + (*v).y * (*v).y);
}
#[no_mangle]
pub unsafe extern "C" fn _v2dIntersect(
    mut p0: *const Vec2d,
    mut p1: *const Vec2d,
    mut p2: *const Vec2d,
    mut p3: *const Vec2d,
    mut inter: *mut Vec2d,
) {
    let mut s1: Vec2d = Vec2d { x: 0., y: 0. };
    let mut s2: Vec2d = Vec2d { x: 0., y: 0. };
    s1.x = (*p1).x - (*p0).x;
    s1.y = (*p1).y - (*p0).y;
    s2.x = (*p3).x - (*p2).x;
    s2.y = (*p3).y - (*p2).y;
    let mut t: libc::c_double = 0.;
    t = (s2.x * ((*p0).y - (*p2).y) - s2.y * ((*p0).x - (*p2).x)) / (-s2.x * s1.y + s1.x * s2.y);
    (*inter).x = (*p0).x + t * s1.x;
    (*inter).y = (*p0).y + t * s1.y;
}
#[no_mangle]
pub unsafe extern "C" fn _v2dAlmostEquals(mut v1: *const Vec2d, mut v2: *const Vec2d) -> bool {
    return fabs((*v1).x - (*v2).x) < 1.19209290e-7f32 as libc::c_double
        && fabs((*v1).y - (*v2).y) < 1.19209290e-7f32 as libc::c_double;
}
