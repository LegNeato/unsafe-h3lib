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
        let mut init = LatLng {
            lat: 0.803582649718989942f64,
            lng: 1.248397419617396099f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 1.307747883455638156f64,
            lng: 2.536945009877921159f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 1.054751253523952054f64,
            lng: -1.347517358900396623f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.600191595538186799f64,
            lng: -0.450603909469755746f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.491715428198773866f64,
            lng: 0.401988202911306943f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.172745327415618701f64,
            lng: 1.678146885280433686f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.605929321571350690f64,
            lng: 2.953923329812411617f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.427370518328979641f64,
            lng: -1.888876200336285401f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -0.079066118549212831f64,
            lng: -0.733429513380867741f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -0.230961644455383637f64,
            lng: 0.506495587332349035f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.079066118549212831f64,
            lng: 2.408163140208925497f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.230961644455383637f64,
            lng: -2.635097066257444203f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -0.172745327415618701f64,
            lng: -1.463445768309359553f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -0.605929321571350690f64,
            lng: -0.187669323777381622f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -0.427370518328979641f64,
            lng: 1.252716453253507838f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -0.600191595538186799f64,
            lng: 2.690988744120037492f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -0.491715428198773866f64,
            lng: -2.739604450678486295f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -0.803582649718989942f64,
            lng: -1.893195233972397139f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -1.307747883455638156f64,
            lng: -0.604647643711872080f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: -1.054751253523952054f64,
            lng: 1.794075294689396615f64,
        };
        init
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
