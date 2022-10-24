#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib_miscapps;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _geoToVec3d(geo: *const LatLng, point: *mut Vec3d);
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub _seek: Option<unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t>,
    pub _write: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut libc::c_void,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3d {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
}
#[no_mangle]
pub static mut faceCenterGeoCopy: [LatLng; 20] = [
    {
        LatLng {
            lat: 0.803_582_649_718_99_f64,
            lng: 1.248_397_419_617_396_1_f64,
        }
    },
    {
        LatLng {
            lat: 1.307_747_883_455_638_2_f64,
            lng: 2.536_945_009_877_921_4_f64,
        }
    },
    {
        LatLng {
            lat: 1.054_751_253_523_952_f64,
            lng: -1.347_517_358_900_396_6_f64,
        }
    },
    {
        LatLng {
            lat: 0.600_191_595_538_186_8_f64,
            lng: -0.450_603_909_469_755_76_f64,
        }
    },
    {
        LatLng {
            lat: 0.491_715_428_198_773_84_f64,
            lng: 0.401_988_202_911_306_94_f64,
        }
    },
    {
        LatLng {
            lat: 0.172_745_327_415_618_7_f64,
            lng: 1.678_146_885_280_433_8_f64,
        }
    },
    {
        LatLng {
            lat: 0.605_929_321_571_350_7_f64,
            lng: 2.953_923_329_812_411_7_f64,
        }
    },
    {
        LatLng {
            lat: 0.427_370_518_328_979_65_f64,
            lng: -1.888_876_200_336_285_3_f64,
        }
    },
    {
        LatLng {
            lat: -0.079_066_118_549_212_83_f64,
            lng: -0.733_429_513_380_867_7_f64,
        }
    },
    {
        LatLng {
            lat: -0.230_961_644_455_383_64_f64,
            lng: 0.506_495_587_332_349_f64,
        }
    },
    {
        LatLng {
            lat: 0.079_066_118_549_212_83_f64,
            lng: 2.408_163_140_208_925_4_f64,
        }
    },
    {
        LatLng {
            lat: 0.230_961_644_455_383_64_f64,
            lng: -2.635_097_066_257_444_f64,
        }
    },
    {
        LatLng {
            lat: -0.172_745_327_415_618_7_f64,
            lng: -1.463_445_768_309_359_6_f64,
        }
    },
    {
        LatLng {
            lat: -0.605_929_321_571_350_7_f64,
            lng: -0.187_669_323_777_381_63_f64,
        }
    },
    {
        LatLng {
            lat: -0.427_370_518_328_979_65_f64,
            lng: 1.252_716_453_253_507_8_f64,
        }
    },
    {
        LatLng {
            lat: -0.600_191_595_538_186_8_f64,
            lng: 2.690_988_744_120_037_5_f64,
        }
    },
    {
        LatLng {
            lat: -0.491_715_428_198_773_84_f64,
            lng: -2.739_604_450_678_486_5_f64,
        }
    },
    {
        LatLng {
            lat: -0.803_582_649_718_99_f64,
            lng: -1.893_195_233_972_397_2_f64,
        }
    },
    {
        LatLng {
            lat: -1.307_747_883_455_638_2_f64,
            lng: -0.604_647_643_711_872_1_f64,
        }
    },
    {
        LatLng {
            lat: -1.054_751_253_523_952_f64,
            lng: 1.794_075_294_689_396_5_f64,
        }
    },
];
unsafe extern "C" fn generate() {
    printf(
        b"static const Vec3d faceCenterPoint[NUM_ICOSA_FACES] = {\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        let mut centerCoords: LatLng = faceCenterGeoCopy[i as usize];
        let mut centerPoint: Vec3d = Vec3d {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        _geoToVec3d(&mut centerCoords, &mut centerPoint);
        printf(
            b"    {%.16f, %.16f, %.16f}, // face %2d\n\0" as *const u8 as *const libc::c_char,
            centerPoint.x,
            centerPoint.y,
            centerPoint.z,
            i,
        );
        i += 1;
    }
    printf(b"};\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    if argc > 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"usage: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    generate();
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
