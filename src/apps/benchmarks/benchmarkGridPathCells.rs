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
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn gridPathCellsSize(start: H3Index, end: H3Index, size: *mut int64_t) -> H3Error;
    fn gridPathCells(start: H3Index, end: H3Index, out: *mut H3Index) -> H3Error;
}
pub type __darwin_time_t = libc::c_long;
pub type int64_t = libc::c_longlong;
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
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type H3Index = uint64_t;
pub type H3Error = uint32_t;
#[no_mangle]
pub static mut startIndex: H3Index = 0x89283082803ffff as libc::c_long as H3Index;
#[no_mangle]
pub static mut endNear: H3Index = 0x892830814b3ffff as libc::c_long as H3Index;
#[no_mangle]
pub static mut endFar: H3Index = 0x8929a5653c3ffff as libc::c_long as H3Index;
unsafe fn main_0(mut _argc: libc::c_int, mut _argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut size: int64_t = 0;
    gridPathCellsSize(startIndex, endFar, &mut size);
    let mut out: *mut H3Index = calloc(
        size as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let iterations: libc::c_int = 10000 as libc::c_int;
    let mut name: *const libc::c_char = b"gridPathCellsNear\0" as *const u8 as *const libc::c_char;
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < iterations {
        gridPathCells(startIndex, endNear, out);
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
    let iterations_0: libc::c_int = 1000 as libc::c_int;
    let mut name_0: *const libc::c_char = b"gridPathCellsFar\0" as *const u8 as *const libc::c_char;
    let mut start_0: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_0);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < iterations_0 {
        gridPathCells(startIndex, endFar, out);
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
    let duration_0: f64 = (elapsed_0.tv_sec as libc::c_double * 1E9f64
        + elapsed_0.tv_nsec as libc::c_double)
        / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_0,
        duration_0 / iterations_0 as f64,
        iterations_0,
    );
    free(out as *mut libc::c_void);
    0
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
        ))
    }
}
