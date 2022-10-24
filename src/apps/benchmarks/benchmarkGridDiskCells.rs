extern crate unsafe_h3lib_benchmarks;
use ::libc;
extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn gridDisk(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
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
pub static mut hex: H3Index = 0x89283080ddbffff as libc::c_long as H3Index;
#[no_mangle]
pub static mut pentagon: H3Index = 0x89080000003ffff as libc::c_long as H3Index;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut outSz: int64_t = 0;
    if maxGridDiskSize(40 as libc::c_int, &mut outSz) != 0 {
        printf(b"Failed\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut out: *mut H3Index = calloc(
        outSz as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let iterations: libc::c_int = 10000 as libc::c_int;
    let mut name: *const libc::c_char = b"gridDisk10\0" as *const u8 as *const libc::c_char;
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < iterations {
        gridDisk(hex, 10 as libc::c_int, out);
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
    let mut name_0: *const libc::c_char = b"gridDisk20\0" as *const u8 as *const libc::c_char;
    let mut start_0: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_0);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < iterations_0 {
        gridDisk(hex, 20 as libc::c_int, out);
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
    let duration_0: f64 = ((elapsed_0.tv_sec as libc::c_double * 1E9f64
        + elapsed_0.tv_nsec as libc::c_double)
        / 1E3f64);
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_0,
        duration_0 / iterations_0 as f64,
        iterations_0,
    );
    let iterations_1: libc::c_int = 10000 as libc::c_int;
    let mut name_1: *const libc::c_char = b"gridDisk30\0" as *const u8 as *const libc::c_char;
    let mut start_1: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_1);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < iterations_1 {
        gridDisk(hex, 30 as libc::c_int, out);
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
    let iterations_2: libc::c_int = 10000 as libc::c_int;
    let mut name_2: *const libc::c_char = b"gridDisk40\0" as *const u8 as *const libc::c_char;
    let mut start_2: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_2);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < iterations_2 {
        gridDisk(hex, 40 as libc::c_int, out);
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
    let iterations_3: libc::c_int = 500 as libc::c_int;
    let mut name_3: *const libc::c_char =
        b"gridDiskPentagon10\0" as *const u8 as *const libc::c_char;
    let mut start_3: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_3);
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < iterations_3 {
        gridDisk(pentagon, 10 as libc::c_int, out);
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
    let iterations_4: libc::c_int = 500 as libc::c_int;
    let mut name_4: *const libc::c_char =
        b"gridDiskPentagon20\0" as *const u8 as *const libc::c_char;
    let mut start_4: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_4);
    let mut i_4: libc::c_int = 0 as libc::c_int;
    while i_4 < iterations_4 {
        gridDisk(pentagon, 20 as libc::c_int, out);
        i_4 += 1;
    }
    let mut end_4: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut end_4);
    let mut elapsed_4: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    elapsed_4.tv_nsec = end_4.tv_nsec - start_4.tv_nsec;
    elapsed_4.tv_sec = end_4.tv_sec - start_4.tv_sec;
    if elapsed_4.tv_nsec < 0 as libc::c_int as libc::c_long {
        elapsed_4.tv_sec -= 1;
        elapsed_4.tv_nsec = (1E9f64 + elapsed_4.tv_nsec as libc::c_double) as libc::c_long;
    }
    let duration_4: f64 = (elapsed_4.tv_sec as libc::c_double * 1E9f64
        + elapsed_4.tv_nsec as libc::c_double)
        / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_4,
        duration_4 / iterations_4 as f64,
        iterations_4,
    );
    let iterations_5: libc::c_int = 50 as libc::c_int;
    let mut name_5: *const libc::c_char =
        b"gridDiskPentagon30\0" as *const u8 as *const libc::c_char;
    let mut start_5: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_5);
    let mut i_5: libc::c_int = 0 as libc::c_int;
    while i_5 < iterations_5 {
        gridDisk(pentagon, 30 as libc::c_int, out);
        i_5 += 1;
    }
    let mut end_5: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut end_5);
    let mut elapsed_5: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    elapsed_5.tv_nsec = end_5.tv_nsec - start_5.tv_nsec;
    elapsed_5.tv_sec = end_5.tv_sec - start_5.tv_sec;
    if elapsed_5.tv_nsec < 0 as libc::c_int as libc::c_long {
        elapsed_5.tv_sec -= 1;
        elapsed_5.tv_nsec = (1E9f64 + elapsed_5.tv_nsec as libc::c_double) as libc::c_long;
    }
    let duration_5: f64 = (elapsed_5.tv_sec as libc::c_double * 1E9f64
        + elapsed_5.tv_nsec as libc::c_double)
        / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_5,
        duration_5 / iterations_5 as f64,
        iterations_5,
    );
    let iterations_6: libc::c_int = 10 as libc::c_int;
    let mut name_6: *const libc::c_char =
        b"gridDiskPentagon40\0" as *const u8 as *const libc::c_char;
    let mut start_6: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_6);
    let mut i_6: libc::c_int = 0 as libc::c_int;
    while i_6 < iterations_6 {
        gridDisk(pentagon, 40 as libc::c_int, out);
        i_6 += 1;
    }
    let mut end_6: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut end_6);
    let mut elapsed_6: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    elapsed_6.tv_nsec = end_6.tv_nsec - start_6.tv_nsec;
    elapsed_6.tv_sec = end_6.tv_sec - start_6.tv_sec;
    if elapsed_6.tv_nsec < 0 as libc::c_int as libc::c_long {
        elapsed_6.tv_sec -= 1;
        elapsed_6.tv_nsec = (1E9f64 + elapsed_6.tv_nsec as libc::c_double) as libc::c_long;
    }
    let duration_6: f64 = (elapsed_6.tv_sec as libc::c_double * 1E9f64
        + elapsed_6.tv_nsec as libc::c_double)
        / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_6,
        duration_6 / iterations_6 as f64,
        iterations_6,
    );
    free(out as *mut libc::c_void);
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
