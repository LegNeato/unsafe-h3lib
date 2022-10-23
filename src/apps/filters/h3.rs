use ::libc;
extern "C" {
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn parseArgs(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        numArgs: libc::c_int,
        args: *mut *mut Arg,
        helpArg: *const Arg,
        helpText: *const libc::c_char,
    ) -> libc::c_int;
    fn h3Println(h: H3Index);
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn degsToRads(degrees: libc::c_double) -> libc::c_double;
    fn radsToDegs(radians: libc::c_double) -> libc::c_double;
}
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type H3Index = uint64_t;
pub type H3Error = uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const E_OPTION_INVALID: C2RustUnnamed = 15;
pub const E_MEMORY_BOUNDS: C2RustUnnamed = 14;
pub const E_MEMORY_ALLOC: C2RustUnnamed = 13;
pub const E_RES_MISMATCH: C2RustUnnamed = 12;
pub const E_NOT_NEIGHBORS: C2RustUnnamed = 11;
pub const E_DUPLICATE_INPUT: C2RustUnnamed = 10;
pub const E_PENTAGON: C2RustUnnamed = 9;
pub const E_VERTEX_INVALID: C2RustUnnamed = 8;
pub const E_UNDIR_EDGE_INVALID: C2RustUnnamed = 7;
pub const E_DIR_EDGE_INVALID: C2RustUnnamed = 6;
pub const E_CELL_INVALID: C2RustUnnamed = 5;
pub const E_RES_DOMAIN: C2RustUnnamed = 4;
pub const E_LATLNG_DOMAIN: C2RustUnnamed = 3;
pub const E_DOMAIN: C2RustUnnamed = 2;
pub const E_FAILED: C2RustUnnamed = 1;
pub const E_SUCCESS: C2RustUnnamed = 0;
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
pub unsafe extern "C" fn has(
    mut subcommand: *mut libc::c_char,
    mut level: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> bool {
    return strcasecmp(subcommand, *argv.offset(level as isize)) == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cellToLatLngCmd(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> bool {
    let mut cellToLatLngArg: Arg = {
        let mut init = Arg {
            names: [
                b"cellToLatLng\0" as *const u8 as *const libc::c_char,
                std::ptr::null::<libc::c_char>(),
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Convert an H3 cell to a latitude/longitude coordinate\0" as *const u8
                as *const libc::c_char,
        };
        init
    };
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
    let mut cell: H3Index = 0 as libc::c_int as H3Index;
    let mut cellArg: Arg = {
        let mut init = Arg {
            names: [
                b"-c\0" as *const u8 as *const libc::c_char,
                b"--cell\0" as *const u8 as *const libc::c_char,
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: b"%llx\0" as *const u8 as *const libc::c_char,
            valueName: b"index\0" as *const u8 as *const libc::c_char,
            value: &mut cell as *mut H3Index as *mut libc::c_void,
            found: false,
            helpText: b"H3 Cell\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut args: [*mut Arg; 3] = [&mut cellToLatLngArg, &mut helpArg, &mut cellArg];
    if parseArgs(
        argc,
        argv,
        3 as libc::c_int,
        args.as_mut_ptr(),
        &mut helpArg,
        b"Convert an H3 cell to a latitude/longitude coordinate\0" as *const u8
            as *const libc::c_char,
    ) != 0
    {
        return helpArg.found;
    }
    let mut ll: LatLng = LatLng { lat: 0., lng: 0. };
    cellToLatLng(cell, &mut ll);
    printf(
        b"%.10lf, %.10lf\n\0" as *const u8 as *const libc::c_char,
        radsToDegs(ll.lat),
        radsToDegs(ll.lng),
    );
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn latLngToCellCmd(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> bool {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut lat: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut lng: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut latLngToCellArg: Arg = {
        let mut init = Arg {
            names: [
                b"latLngToCell\0" as *const u8 as *const libc::c_char,
                std::ptr::null::<libc::c_char>(),
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Convert degrees latitude/longitude coordinate to an H3 cell.\0" as *const u8
                as *const libc::c_char,
        };
        init
    };
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
            required: 1 as libc::c_int != 0,
            scanFormat: b"%d\0" as *const u8 as *const libc::c_char,
            valueName: b"res\0" as *const u8 as *const libc::c_char,
            value: &mut res as *mut libc::c_int as *mut libc::c_void,
            found: false,
            helpText: b"Resolution, 0-15 inclusive.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut latArg: Arg = {
        let mut init = Arg {
            names: [
                b"--lat\0" as *const u8 as *const libc::c_char,
                b"--latitude\0" as *const u8 as *const libc::c_char,
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: b"%lf\0" as *const u8 as *const libc::c_char,
            valueName: b"lat\0" as *const u8 as *const libc::c_char,
            value: &mut lat as *mut libc::c_double as *mut libc::c_void,
            found: false,
            helpText: b"Latitude in degrees.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut lngArg: Arg = {
        let mut init = Arg {
            names: [
                b"--lng\0" as *const u8 as *const libc::c_char,
                b"--longitude\0" as *const u8 as *const libc::c_char,
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: b"%lf\0" as *const u8 as *const libc::c_char,
            valueName: b"lng\0" as *const u8 as *const libc::c_char,
            value: &mut lng as *mut libc::c_double as *mut libc::c_void,
            found: false,
            helpText: b"Longitude in degrees.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut args: [*mut Arg; 5] = [
        &mut latLngToCellArg,
        &mut helpArg,
        &mut resArg,
        &mut latArg,
        &mut lngArg,
    ];
    if parseArgs(
        argc,
        argv,
        5 as libc::c_int,
        args.as_mut_ptr(),
        &mut helpArg,
        b"Convert degrees latitude/longitude coordinate to an H3 cell.\0" as *const u8
            as *const libc::c_char,
    ) != 0
    {
        return helpArg.found;
    }
    let mut ll: LatLng = {
        let mut init = LatLng {
            lat: degsToRads(lat),
            lng: degsToRads(lng),
        };
        init
    };
    let mut c: H3Index = 0;
    let mut e: H3Error = latLngToCell(&mut ll, res, &mut c);
    if e == E_SUCCESS as libc::c_int as libc::c_uint {
        h3Println(c);
    } else {
        h3Println(0 as libc::c_int as H3Index);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn generalHelp(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> bool {
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
    let mut cellToLatLngArg: Arg = {
        let mut init = Arg {
            names: [
                b"cellToLatLng\0" as *const u8 as *const libc::c_char,
                std::ptr::null::<libc::c_char>(),
            ],
            required: false,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Convert an H3 cell to a latitude/longitude coordinate\0" as *const u8
                as *const libc::c_char,
        };
        init
    };
    let mut latLngToCellArg: Arg = {
        let mut init = Arg {
            names: [
                b"latLngToCell\0" as *const u8 as *const libc::c_char,
                std::ptr::null::<libc::c_char>(),
            ],
            required: false,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Convert degrees latitude/longitude coordinate to an H3 cell.\0" as *const u8
                as *const libc::c_char,
        };
        init
    };
    let mut args: [*mut Arg; 3] = [&mut helpArg, &mut cellToLatLngArg, &mut latLngToCellArg];
    let mut helpText: *const libc::c_char = b"Please use one of the subcommands listed to perform an H3 calculation. Use h3 <SUBCOMMAND> --help for details on the usage of any subcommand.\0"
        as *const u8 as *const libc::c_char;
    return parseArgs(
        argc,
        argv,
        3 as libc::c_int,
        args.as_mut_ptr(),
        &mut helpArg,
        helpText,
    ) != 0;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    if argc <= 1 as libc::c_int {
        printf(
            b"Please use h3 --help to see how to use this command.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if has(
        b"cellToLatLng\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
        argv,
    ) as libc::c_int
        != 0
        && cellToLatLngCmd(argc, argv) as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    if has(
        b"latLngToCell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
        argv,
    ) as libc::c_int
        != 0
        && latLngToCellCmd(argc, argv) as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    if generalHelp(argc, argv) {
        return 0 as libc::c_int;
    }
    printf(
        b"Please use h3 --help to see how to use this command.\n\0" as *const u8
            as *const libc::c_char,
    );
    return 1 as libc::c_int;
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
