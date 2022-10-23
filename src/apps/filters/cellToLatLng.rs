use ::libc;
extern "C" {
    pub type __sFILEX;
    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut __stdinp: *mut FILE;
    fn feof(_: *mut FILE) -> libc::c_int;
    fn parseArgs(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        numArgs: libc::c_int,
        args: *mut *mut Arg,
        helpArg: *const Arg,
        helpText: *const libc::c_char,
    ) -> libc::c_int;
    fn error(msg: *const libc::c_char);
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn radsToDegs(radians: libc::c_double) -> libc::c_double;
    fn stringToH3(str: *const libc::c_char, out: *mut H3Index) -> H3Error;
    fn h3ToString(h: H3Index, str: *mut libc::c_char, sz: size_t) -> H3Error;
    fn kmlPtsHeader(name: *const libc::c_char, desc: *const libc::c_char);
    fn kmlPtsFooter();
    fn outputPointKML(g: *const LatLng, name: *const libc::c_char);
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type size_t = __darwin_size_t;
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
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
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
pub struct Arg {
    pub names: [*const libc::c_char; 2],
    pub required: bool,
    pub scanFormat: *const libc::c_char,
    pub valueName: *const libc::c_char,
    pub value: *mut libc::c_void,
    pub found: bool,
    pub helpText: *const libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn doCell(mut h: H3Index, mut isKmlOut: libc::c_int) {
    let mut g: LatLng = LatLng { lat: 0., lng: 0. };
    cellToLatLng(h, &mut g);
    let mut label: [libc::c_char; 256] = [0; 256];
    if isKmlOut != 0 {
        h3ToString(h, label.as_mut_ptr(), 256 as libc::c_int as size_t);
        outputPointKML(&mut g, label.as_mut_ptr());
    } else {
        printf(
            b"%.10lf %.10lf\n\0" as *const u8 as *const libc::c_char,
            radsToDegs(g.lat),
            radsToDegs(g.lng),
        );
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut helpArg: Arg = {
        let mut init = Arg {
            names: [
                b"-h\0" as *const u8 as *const libc::c_char,
                b"--help\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Show this help message.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut index: H3Index = 0 as libc::c_int as H3Index;
    let mut indexArg: Arg = {
        let mut init = Arg {
            names: [
                b"-i\0" as *const u8 as *const libc::c_char,
                b"--index\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: b"%llx\0" as *const u8 as *const libc::c_char,
            valueName: b"index\0" as *const u8 as *const libc::c_char,
            value: &mut index as *mut H3Index as *mut libc::c_void,
            found: false,
            helpText: b"Index, or not specified to read indexes from standard input.\0" as *const u8
                as *const libc::c_char,
        };
        init
    };
    let mut kmlArg: Arg = {
        let mut init = Arg {
            names: [
                b"-k\0" as *const u8 as *const libc::c_char,
                b"--kml\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Print output in KML format.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut userKmlName: [libc::c_char; 256] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut kmlNameArg: Arg = {
        let mut init = Arg {
            names: [
                b"--kn\0" as *const u8 as *const libc::c_char,
                b"--kml-name\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: b"%255c\0" as *const u8 as *const libc::c_char,
            valueName: b"name\0" as *const u8 as *const libc::c_char,
            value: &mut userKmlName as *mut [libc::c_char; 256] as *mut libc::c_void,
            found: false,
            helpText: b"Text for the KML name tag, if --kml is specified.\0" as *const u8
                as *const libc::c_char,
        };
        init
    };
    let mut userKmlDesc: [libc::c_char; 256] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut kmlDescArg: Arg = {
        let mut init = Arg {
            names: [
                b"--kd\0" as *const u8 as *const libc::c_char,
                b"--kml-description\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: b"%255c\0" as *const u8 as *const libc::c_char,
            valueName: b"description\0" as *const u8 as *const libc::c_char,
            value: &mut userKmlDesc as *mut [libc::c_char; 256] as *mut libc::c_void,
            found: false,
            helpText: b"Text for the KML description tag, if --kml is specified.\0" as *const u8
                as *const libc::c_char,
        };
        init
    };
    let mut args: [*mut Arg; 5] = [
        &mut helpArg,
        &mut indexArg,
        &mut kmlArg,
        &mut kmlNameArg,
        &mut kmlDescArg,
    ];
    if parseArgs(
        argc,
        argv,
        5 as libc::c_int,
        args.as_mut_ptr(),
        &mut helpArg,
        b"Converts indexes to latitude/longitude center coordinates in degrees\0" as *const u8
            as *const libc::c_char,
    ) != 0
    {
        return if helpArg.found as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    if kmlArg.found {
        let mut kmlName: *mut libc::c_char =
            b"H3 Geometry\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if kmlNameArg.found {
            kmlName = userKmlName.as_mut_ptr();
        }
        let mut kmlDesc: *mut libc::c_char =
            b"Generated by cellToLatLng\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if kmlDescArg.found {
            kmlDesc = userKmlDesc.as_mut_ptr();
        }
        kmlPtsHeader(kmlName, kmlDesc);
    }
    if indexArg.found {
        doCell(index, kmlArg.found as libc::c_int);
    } else {
        let mut buff: [libc::c_char; 256] = [0; 256];
        loop {
            if (fgets(buff.as_mut_ptr(), 256 as libc::c_int, __stdinp)).is_null() {
                if feof(__stdinp) != 0 {
                    break;
                }
                error(b"reading H3 index from stdin\0" as *const u8 as *const libc::c_char);
            }
            let mut h3: H3Index = 0;
            stringToH3(buff.as_mut_ptr(), &mut h3);
            doCell(h3, kmlArg.found as libc::c_int);
        }
    }
    if kmlArg.found {
        kmlPtsFooter();
    }
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
