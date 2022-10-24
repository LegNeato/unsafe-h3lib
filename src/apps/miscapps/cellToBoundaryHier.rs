extern crate unsafe_h3lib_miscapps;
use ::libc;
extern "C" {

    static mut __stderrp: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
    fn printHelp(
        out: *mut FILE,
        programName: *const libc::c_char,
        helpText: *const libc::c_char,
        numArgs: libc::c_int,
        args: *mut *mut Arg,
        errorMessage: *const libc::c_char,
        errorDetails: *const libc::c_char,
    );
    fn parseArgs(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        numArgs: libc::c_int,
        args: *mut *mut Arg,
        helpArg: *const Arg,
        helpText: *const libc::c_char,
    ) -> libc::c_int;
    fn cellBoundaryPrintln(b: *const CellBoundary);
    fn h3ToString(h: H3Index, str: *mut libc::c_char, sz: size_t) -> H3Error;
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn kmlBoundaryHeader(name: *const libc::c_char, desc: *const libc::c_char);
    fn kmlBoundaryFooter();
    fn outputBoundaryKML(b: *const CellBoundary, name: *const libc::c_char);
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
    pub _extra: *mut libc::c_void,
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
pub struct CellBoundary {
    pub numVerts: libc::c_int,
    pub verts: [LatLng; 10],
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
    let mut b: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    cellToBoundary(h, &mut b);
    let mut label: [libc::c_char; 256] = [0; 256];
    h3ToString(h, label.as_mut_ptr(), 256 as libc::c_int as size_t);
    if isKmlOut != 0 {
        outputBoundaryKML(&mut b, label.as_mut_ptr());
    } else {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            label.as_mut_ptr(),
        );
        cellBoundaryPrintln(&mut b);
    };
}
#[no_mangle]
pub unsafe extern "C" fn recursiveH3IndexToGeo(
    mut h: H3Index,
    mut res: libc::c_int,
    mut isKmlOut: libc::c_int,
) {
    let mut d: libc::c_int = 0 as libc::c_int;
    while d < 7 as libc::c_int {
        if !(isPentagon(h) != 0 && d == 1 as libc::c_int) {
            h = h & !((7 as libc::c_int as uint64_t)
                << (15 as libc::c_int - res) * 3 as libc::c_int)
                | (d as uint64_t) << (15 as libc::c_int - res) * 3 as libc::c_int;
            if res
                == ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
                    as libc::c_int
            {
                doCell(h, isKmlOut);
            } else {
                recursiveH3IndexToGeo(h, res + 1 as libc::c_int, isKmlOut);
            }
        }
        d += 1;
    }
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut parentIndex: H3Index = 0 as libc::c_int as H3Index;
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
    let mut resArg: Arg = {
        let mut init = Arg {
            names: [
                b"-r\0" as *const u8 as *const libc::c_char,
                b"--resolution\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: b"%d\0" as *const u8 as *const libc::c_char,
            valueName: b"res\0" as *const u8 as *const libc::c_char,
            value: &mut res as *mut libc::c_int as *mut libc::c_void,
            found: false,
            helpText: b"Resolution, if less than the resolution of the parent only the parent is printed. Default the resolution of the parent.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut parentArg: Arg = {
        let mut init = Arg {
            names: [
                b"-p\0" as *const u8 as *const libc::c_char,
                b"--parent\0" as *const u8 as *const libc::c_char,
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: b"%llx\0" as *const u8 as *const libc::c_char,
            valueName: b"parent\0" as *const u8 as *const libc::c_char,
            value: &mut parentIndex as *mut H3Index as *mut libc::c_void,
            found: false,
            helpText: b"Print cell boundaries descendent from this index.\0" as *const u8
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
    let mut args: [*mut Arg; 6] = [
        &mut helpArg,
        &mut resArg,
        &mut parentArg,
        &mut kmlArg,
        &mut kmlNameArg,
        &mut kmlDescArg,
    ];
    let numArgs: libc::c_int = 6 as libc::c_int;
    let mut helpText: *const libc::c_char =
        b"Print cell boundaries for descendants of an index\0" as *const u8 as *const libc::c_char;
    if parseArgs(
        argc,
        argv,
        numArgs,
        args.as_mut_ptr(),
        &mut helpArg,
        helpText,
    ) != 0
    {
        return if helpArg.found as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    if res > 15 as libc::c_int {
        printHelp(
            __stderrp,
            *argv.offset(0 as libc::c_int as isize),
            helpText,
            numArgs,
            args.as_mut_ptr(),
            b"Resolution exceeds maximum resolution.\0" as *const u8 as *const libc::c_char,
            std::ptr::null::<libc::c_char>(),
        );
        return 1 as libc::c_int;
    }
    if isValidCell(parentIndex) == 0 {
        printHelp(
            __stderrp,
            *argv.offset(0 as libc::c_int as isize),
            helpText,
            numArgs,
            args.as_mut_ptr(),
            b"Parent index is invalid.\0" as *const u8 as *const libc::c_char,
            std::ptr::null::<libc::c_char>(),
        );
        return 1 as libc::c_int;
    }
    let mut rootRes: libc::c_int = ((parentIndex & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int;
    if kmlArg.found {
        let mut kmlName: *mut libc::c_char = 0 as *mut libc::c_char;
        if kmlNameArg.found {
            kmlName = strdup(userKmlName.as_mut_ptr());
        } else {
            kmlName = calloc(
                256 as libc::c_int as libc::c_ulong,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            ) as *mut libc::c_char;
            sprintf(
                kmlName,
                b"Cell %llx Res %d\0" as *const u8 as *const libc::c_char,
                parentIndex,
                if res <= rootRes { rootRes } else { res },
            );
        }
        let mut kmlDesc: *mut libc::c_char = b"Generated by cellToBoundaryHier\0" as *const u8
            as *const libc::c_char
            as *mut libc::c_char;
        if kmlDescArg.found {
            kmlDesc = userKmlDesc.as_mut_ptr();
        }
        kmlBoundaryHeader(kmlName, kmlDesc);
        free(kmlName as *mut libc::c_void);
    }
    if res <= rootRes {
        doCell(parentIndex, kmlArg.found as libc::c_int);
    } else {
        parentIndex = parentIndex & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
            | (res as uint64_t) << 52 as libc::c_int;
        recursiveH3IndexToGeo(
            parentIndex,
            rootRes + 1 as libc::c_int,
            kmlArg.found as libc::c_int,
        );
    }
    if kmlArg.found {
        kmlBoundaryFooter();
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
