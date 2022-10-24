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
    fn cellToCenterChild(h: H3Index, childRes: libc::c_int, child: *mut H3Index) -> H3Error;
    fn cellToChildren(h: H3Index, childRes: libc::c_int, children: *mut H3Index) -> H3Error;
    fn cellToChildrenSize(h: H3Index, childRes: libc::c_int, out: *mut int64_t) -> H3Error;
    fn isValidCell(h: H3Index) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellArray {
    pub cells: *mut H3Index,
    pub N: int64_t,
}
#[no_mangle]
pub unsafe extern "C" fn pentagonSetup(
    mut parentRes: libc::c_int,
    mut childRes: libc::c_int,
    mut nullEvery: libc::c_int,
) -> CellArray {
    let mut pParent: H3Index = 0x80c3fffffffffff as libc::c_long as H3Index;
    let mut p: H3Index = 0;
    cellToCenterChild(pParent, parentRes, &mut p);
    let mut ca_0: CellArray = CellArray {
        cells: std::ptr::null_mut::<H3Index>(),
        N: 0,
    };
    cellToChildrenSize(p, childRes, &mut ca_0.N);
    ca_0.cells = calloc(
        ca_0.N as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    cellToChildren(p, childRes, ca_0.cells);
    if nullEvery > 0 as libc::c_int {
        let mut i: int64_t = 0 as libc::c_int as int64_t;
        while i < ca_0.N {
            *(ca_0.cells).offset(i as isize) = 0 as libc::c_int as H3Index;
            i += nullEvery as libc::c_longlong;
        }
    }
    ca_0
}
#[inline]
unsafe extern "C" fn runValidation(ca_0: CellArray) {
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < ca_0.N {
        isValidCell(*(ca_0.cells).offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub static mut ca: CellArray = CellArray {
    cells: std::ptr::null_mut::<H3Index>(),
    N: 0,
};
unsafe fn main_0(mut _argc: libc::c_int, mut _argv: *mut *mut libc::c_char) -> libc::c_int {
    ca = pentagonSetup(2 as libc::c_int, 8 as libc::c_int, 0 as libc::c_int);
    let iterations: libc::c_int = 1000 as libc::c_int;
    let mut name: *const libc::c_char =
        b"pentagonChildren_2_8\0" as *const u8 as *const libc::c_char;
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < iterations {
        runValidation(ca);
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
    free(ca.cells as *mut libc::c_void);
    ca = pentagonSetup(8 as libc::c_int, 14 as libc::c_int, 0 as libc::c_int);
    let iterations_0: libc::c_int = 1000 as libc::c_int;
    let mut name_0: *const libc::c_char =
        b"pentagonChildren_8_14\0" as *const u8 as *const libc::c_char;
    let mut start_0: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_0);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < iterations_0 {
        runValidation(ca);
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
    free(ca.cells as *mut libc::c_void);
    ca = pentagonSetup(8 as libc::c_int, 14 as libc::c_int, 2 as libc::c_int);
    let iterations_1: libc::c_int = 1000 as libc::c_int;
    let mut name_1: *const libc::c_char =
        b"pentagonChildren_8_14_null_2\0" as *const u8 as *const libc::c_char;
    let mut start_1: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_1);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < iterations_1 {
        runValidation(ca);
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
    free(ca.cells as *mut libc::c_void);
    ca = pentagonSetup(8 as libc::c_int, 14 as libc::c_int, 10 as libc::c_int);
    let iterations_2: libc::c_int = 1000 as libc::c_int;
    let mut name_2: *const libc::c_char =
        b"pentagonChildren_8_14_null_10\0" as *const u8 as *const libc::c_char;
    let mut start_2: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_2);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < iterations_2 {
        runValidation(ca);
        i_2 += 1;
    }
    let mut end_2: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut end_2);
    let mut elapsed_2: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    elapsed_2.tv_nsec = end_2.tv_nsec - start_2.tv_nsec;
    elapsed_2.tv_sec = end_2.tv_sec - start_2.tv_sec;
    if elapsed_2.tv_nsec < 0 as libc::c_int as libc::c_long {
        elapsed_2.tv_sec -= 1;
        elapsed_2.tv_nsec = (1E9f64 + elapsed_2.tv_nsec as libc::c_double) as libc::c_long;
    }
    let duration_2: f64 = (elapsed_2.tv_sec as libc::c_double * 1E9f64
        + elapsed_2.tv_nsec as libc::c_double)
        / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_2,
        duration_2 / iterations_2 as f64,
        iterations_2,
    );
    free(ca.cells as *mut libc::c_void);
    ca = pentagonSetup(8 as libc::c_int, 14 as libc::c_int, 100 as libc::c_int);
    let iterations_3: libc::c_int = 1000 as libc::c_int;
    let mut name_3: *const libc::c_char =
        b"pentagonChildren_8_14_null_100\0" as *const u8 as *const libc::c_char;
    let mut start_3: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_3);
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < iterations_3 {
        runValidation(ca);
        i_3 += 1;
    }
    let mut end_3: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut end_3);
    let mut elapsed_3: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    elapsed_3.tv_nsec = end_3.tv_nsec - start_3.tv_nsec;
    elapsed_3.tv_sec = end_3.tv_sec - start_3.tv_sec;
    if elapsed_3.tv_nsec < 0 as libc::c_int as libc::c_long {
        elapsed_3.tv_sec -= 1;
        elapsed_3.tv_nsec = (1E9f64 + elapsed_3.tv_nsec as libc::c_double) as libc::c_long;
    }
    let duration_3: f64 = (elapsed_3.tv_sec as libc::c_double * 1E9f64
        + elapsed_3.tv_nsec as libc::c_double)
        / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_3,
        duration_3 / iterations_3 as f64,
        iterations_3,
    );
    free(ca.cells as *mut libc::c_void);
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
