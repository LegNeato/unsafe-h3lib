extern crate unsafe_h3lib_benchmarks;
use ::libc;
extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn cellToVertexes(origin: H3Index, vertexes: *mut H3Index) -> H3Error;
}
pub type __darwin_time_t = libc::c_long;
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
#[no_mangle]
pub static mut ring2: [H3Index; 19] = [
    0x89283081083ffff as libc::c_long as H3Index,
    0x8928308109bffff as libc::c_long as H3Index,
    0x8928308108bffff as libc::c_long as H3Index,
    0x8928308108fffff as libc::c_long as H3Index,
    0x89283081087ffff as libc::c_long as H3Index,
    0x89283081097ffff as libc::c_long as H3Index,
    0x89283081093ffff as libc::c_long as H3Index,
    0x89283081467ffff as libc::c_long as H3Index,
    0x8928308146fffff as libc::c_long as H3Index,
    0x892830810d7ffff as libc::c_long as H3Index,
    0x892830810c7ffff as libc::c_long as H3Index,
    0x89283081013ffff as libc::c_long as H3Index,
    0x89283081017ffff as libc::c_long as H3Index,
    0x892830810bbffff as libc::c_long as H3Index,
    0x892830810b3ffff as libc::c_long as H3Index,
    0x8928308154bffff as libc::c_long as H3Index,
    0x8928308155bffff as libc::c_long as H3Index,
    0x8928308142fffff as libc::c_long as H3Index,
    0x8928308142bffff as libc::c_long as H3Index,
];
#[no_mangle]
pub static mut ring2Count: libc::c_int = 19 as libc::c_int;
#[no_mangle]
pub static mut ring2Pent: [H3Index; 16] = [
    0x8508008bfffffff as libc::c_long as H3Index,
    0x8508000ffffffff as libc::c_long as H3Index,
    0x85080077fffffff as libc::c_long as H3Index,
    0x85080047fffffff as libc::c_long as H3Index,
    0x85080017fffffff as libc::c_long as H3Index,
    0x85080003fffffff as libc::c_long as H3Index,
    0x8508000bfffffff as libc::c_long as H3Index,
    0x85080073fffffff as libc::c_long as H3Index,
    0x85080057fffffff as libc::c_long as H3Index,
    0x850800abfffffff as libc::c_long as H3Index,
    0x8508008ffffffff as libc::c_long as H3Index,
    0x85080013fffffff as libc::c_long as H3Index,
    0x8508001bfffffff as libc::c_long as H3Index,
    0x850800c7fffffff as libc::c_long as H3Index,
    0x850800cffffffff as libc::c_long as H3Index,
    0x850800bbfffffff as libc::c_long as H3Index,
];
#[no_mangle]
pub static mut ring2PentCount: libc::c_int = 16 as libc::c_int;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut vertexes: *mut H3Index = calloc(
        6 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let iterations: libc::c_int = 10000 as libc::c_int;
    let mut name: *const libc::c_char = b"cellToVertexes\0" as *const u8 as *const libc::c_char;
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < iterations {
        cellToVertexes(hex, vertexes);
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
        b"cellToVertexesPent\0" as *const u8 as *const libc::c_char;
    let mut start_0: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_0);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < iterations_0 {
        cellToVertexes(pentagon, vertexes);
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
    let iterations_1: libc::c_int = 10000 as libc::c_int;
    let mut name_1: *const libc::c_char =
        b"cellToVertexesRing\0" as *const u8 as *const libc::c_char;
    let mut start_1: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_1);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < iterations_1 {
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < ring2Count {
            cellToVertexes(ring2[i_2 as usize], vertexes);
            i_2 += 1;
        }
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
    let mut name_2: *const libc::c_char =
        b"cellToVertexesRingPent\0" as *const u8 as *const libc::c_char;
    let mut start_2: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_2);
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < iterations_2 {
        let mut i_4: libc::c_int = 0 as libc::c_int;
        while i_4 < ring2PentCount {
            cellToVertexes(ring2Pent[i_4 as usize], vertexes);
            i_4 += 1;
        }
        i_3 += 1;
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
    free(vertexes as *mut libc::c_void);
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
