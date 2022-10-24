#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib_benchmarks;
use ::libc;
extern "C" {
    fn cellsToLinkedMultiPolygon(
        h3Set: *const H3Index,
        numHexes: libc::c_int,
        out: *mut LinkedGeoPolygon,
    ) -> H3Error;
    fn destroyLinkedMultiPolygon(polygon: *mut LinkedGeoPolygon);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
}
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type __darwin_time_t = libc::c_long;
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
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: libc::c_long,
}
pub type clockid_t = libc::c_uint;
pub const _CLOCK_THREAD_CPUTIME_ID: clockid_t = 16;
pub const _CLOCK_PROCESS_CPUTIME_ID: clockid_t = 12;
pub const _CLOCK_UPTIME_RAW_APPROX: clockid_t = 9;
pub const _CLOCK_UPTIME_RAW: clockid_t = 8;
pub const _CLOCK_MONOTONIC_RAW_APPROX: clockid_t = 5;
pub const _CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const _CLOCK_MONOTONIC: clockid_t = 6;
pub const _CLOCK_REALTIME: clockid_t = 0;
#[no_mangle]
pub static mut ring2: [H3Index; 19] = [
    0x8930062838bffff as libc::c_long as H3Index,
    0x8930062838fffff as libc::c_long as H3Index,
    0x89300628383ffff as libc::c_long as H3Index,
    0x8930062839bffff as libc::c_long as H3Index,
    0x893006283d7ffff as libc::c_long as H3Index,
    0x893006283c7ffff as libc::c_long as H3Index,
    0x89300628313ffff as libc::c_long as H3Index,
    0x89300628317ffff as libc::c_long as H3Index,
    0x893006283bbffff as libc::c_long as H3Index,
    0x89300628387ffff as libc::c_long as H3Index,
    0x89300628397ffff as libc::c_long as H3Index,
    0x89300628393ffff as libc::c_long as H3Index,
    0x89300628067ffff as libc::c_long as H3Index,
    0x8930062806fffff as libc::c_long as H3Index,
    0x893006283d3ffff as libc::c_long as H3Index,
    0x893006283c3ffff as libc::c_long as H3Index,
    0x893006283cfffff as libc::c_long as H3Index,
    0x8930062831bffff as libc::c_long as H3Index,
    0x89300628303ffff as libc::c_long as H3Index,
];
#[no_mangle]
pub static mut ring2Count: libc::c_int = 19 as libc::c_int;
#[no_mangle]
pub static mut donut: [H3Index; 6] = [
    0x892830828c7ffff as libc::c_long as H3Index,
    0x892830828d7ffff as libc::c_long as H3Index,
    0x8928308289bffff as libc::c_long as H3Index,
    0x89283082813ffff as libc::c_long as H3Index,
    0x8928308288fffff as libc::c_long as H3Index,
    0x89283082883ffff as libc::c_long as H3Index,
];
#[no_mangle]
pub static mut donutCount: libc::c_int = 6 as libc::c_int;
#[no_mangle]
pub static mut nestedDonuts: [H3Index; 24] = [
    0x89283082813ffff as libc::c_long as H3Index,
    0x8928308281bffff as libc::c_long as H3Index,
    0x8928308280bffff as libc::c_long as H3Index,
    0x8928308280fffff as libc::c_long as H3Index,
    0x89283082807ffff as libc::c_long as H3Index,
    0x89283082817ffff as libc::c_long as H3Index,
    0x8928308289bffff as libc::c_long as H3Index,
    0x892830828d7ffff as libc::c_long as H3Index,
    0x892830828c3ffff as libc::c_long as H3Index,
    0x892830828cbffff as libc::c_long as H3Index,
    0x89283082853ffff as libc::c_long as H3Index,
    0x89283082843ffff as libc::c_long as H3Index,
    0x8928308284fffff as libc::c_long as H3Index,
    0x8928308287bffff as libc::c_long as H3Index,
    0x89283082863ffff as libc::c_long as H3Index,
    0x89283082867ffff as libc::c_long as H3Index,
    0x8928308282bffff as libc::c_long as H3Index,
    0x89283082823ffff as libc::c_long as H3Index,
    0x89283082837ffff as libc::c_long as H3Index,
    0x892830828afffff as libc::c_long as H3Index,
    0x892830828a3ffff as libc::c_long as H3Index,
    0x892830828b3ffff as libc::c_long as H3Index,
    0x89283082887ffff as libc::c_long as H3Index,
    0x89283082883ffff as libc::c_long as H3Index,
];
#[no_mangle]
pub static mut nestedDonutsCount: libc::c_int = 24 as libc::c_int;
unsafe fn main_0(mut _argc: libc::c_int, mut _argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut polygon: LinkedGeoPolygon = LinkedGeoPolygon {
        first: 0 as *mut LinkedGeoLoop,
        last: 0 as *mut LinkedGeoLoop,
        next: 0 as *mut LinkedGeoPolygon,
    };
    let iterations: libc::c_int = 10000 as libc::c_int;
    let mut name: *const libc::c_char =
        b"cellsToLinkedMultiPolygonRing2\0" as *const u8 as *const libc::c_char;
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < iterations {
        cellsToLinkedMultiPolygon(ring2.as_mut_ptr(), ring2Count, &mut polygon);
        destroyLinkedMultiPolygon(&mut polygon);
        i += 1;
    }
    let mut end: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut end);
    let mut elapsed: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    elapsed.tv_nsec = end.tv_nsec - start.tv_nsec;
    elapsed.tv_sec = end.tv_sec - start.tv_sec;
    if elapsed.tv_nsec < 0 as libc::c_int as libc::c_long {
        elapsed.tv_sec -= 1;
        elapsed.tv_nsec = (1E9f64 + elapsed.tv_nsec as libc::c_double) as libc::c_long;
    }
    let duration: f64 =
        (elapsed.tv_sec as libc::c_double * 1E9f64 + elapsed.tv_nsec as libc::c_double) / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name,
        duration / iterations as f64,
        iterations,
    );
    let iterations_0: libc::c_int = 10000 as libc::c_int;
    let mut name_0: *const libc::c_char =
        b"cellsToLinkedMultiPolygonDonut\0" as *const u8 as *const libc::c_char;
    let mut start_0: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_0);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < iterations_0 {
        cellsToLinkedMultiPolygon(donut.as_mut_ptr(), donutCount, &mut polygon);
        destroyLinkedMultiPolygon(&mut polygon);
        i_0 += 1;
    }
    let mut end_0: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut end_0);
    let mut elapsed_0: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    elapsed_0.tv_nsec = end_0.tv_nsec - start_0.tv_nsec;
    elapsed_0.tv_sec = end_0.tv_sec - start_0.tv_sec;
    if elapsed_0.tv_nsec < 0 as libc::c_int as libc::c_long {
        elapsed_0.tv_sec -= 1;
        elapsed_0.tv_nsec = (1E9f64 + elapsed_0.tv_nsec as libc::c_double) as libc::c_long;
    }
    let duration_0: f64 = elapsed_0.tv_sec as libc::c_double * 1E9f64
        + elapsed_0.tv_nsec as libc::c_double
        / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_0,
        duration_0 / iterations_0 as f64,
        iterations_0,
    );
    let iterations_1: libc::c_int = 10000 as libc::c_int;
    let mut name_1: *const libc::c_char =
        b"cellsToLinkedMultiPolygonNestedDonuts\0" as *const u8 as *const libc::c_char;
    let mut start_1: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_1);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < iterations_1 {
        cellsToLinkedMultiPolygon(nestedDonuts.as_mut_ptr(), nestedDonutsCount, &mut polygon);
        destroyLinkedMultiPolygon(&mut polygon);
        i_1 += 1;
    }
    let mut end_1: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut end_1);
    let mut elapsed_1: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    elapsed_1.tv_nsec = end_1.tv_nsec - start_1.tv_nsec;
    elapsed_1.tv_sec = end_1.tv_sec - start_1.tv_sec;
    if elapsed_1.tv_nsec < 0 as libc::c_int as libc::c_long {
        elapsed_1.tv_sec -= 1;
        elapsed_1.tv_nsec = (1E9f64 + elapsed_1.tv_nsec as libc::c_double) as libc::c_long;
    }
    let duration_1: f64 = (elapsed_1.tv_sec as libc::c_double * 1E9f64
        + elapsed_1.tv_nsec as libc::c_double)
        / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_1,
        duration_1 / iterations_1 as f64,
        iterations_1,
    );
    return 0;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
