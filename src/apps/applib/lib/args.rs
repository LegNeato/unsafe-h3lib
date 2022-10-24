use ::libc;
extern "C" {

    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
pub unsafe extern "C" fn parseArgs(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut numArgs: libc::c_int,
    mut args: *mut *mut Arg,
    mut helpArg: *const Arg,
    mut helpText: *const libc::c_char,
) -> libc::c_int {
    let mut errorMessage: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut errorDetails: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut failed: libc::c_int = _parseArgsList(
        argc,
        argv,
        numArgs,
        args,
        helpArg,
        &mut errorMessage,
        &mut errorDetails,
    );
    if failed != 0 || (*helpArg).found as libc::c_int != 0 {
        printHelp(
            if (*helpArg).found as libc::c_int != 0 {
                __stdoutp
            } else {
                __stderrp
            },
            *argv.offset(0 as libc::c_int as isize),
            helpText,
            numArgs,
            args,
            errorMessage,
            errorDetails,
        );
        return if failed != 0 as libc::c_int {
            failed
        } else {
            1 as libc::c_int
        };
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn _parseArgsList(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut numArgs: libc::c_int,
    mut args: *mut *mut Arg,
    mut helpArg: *const Arg,
    mut errorMessage: *mut *const libc::c_char,
    mut errorDetail: *mut *const libc::c_char,
) -> libc::c_int {
    let mut foundHelp: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < argc {
        let mut foundMatch: bool = 0 as libc::c_int != 0;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < numArgs {
            let mut argName: *const libc::c_char = std::ptr::null::<libc::c_char>();
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < 2 as libc::c_int {
                if !((**args.offset(j as isize)).names[k as usize]).is_null()
                    && strcasecmp(
                        *argv.offset(i as isize),
                        (**args.offset(j as isize)).names[k as usize],
                    ) == 0 as libc::c_int
                {
                    argName = (**args.offset(j as isize)).names[k as usize];
                    break;
                }
                k += 1;
            }
            if argName.is_null() {
                j += 1;
            } else {
                if (**args.offset(j as isize)).found {
                    *errorMessage =
                        b"Argument specified multiple times\0" as *const u8 as *const libc::c_char;
                    *errorDetail = argName;
                    return 2 as libc::c_int;
                }
                if !((**args.offset(j as isize)).scanFormat).is_null() {
                    i += 1;
                    if i >= argc {
                        *errorMessage =
                            b"Argument value not present\0" as *const u8 as *const libc::c_char;
                        *errorDetail = argName;
                        return 3 as libc::c_int;
                    }
                    if sscanf(
                        *argv.offset(i as isize),
                        (**args.offset(j as isize)).scanFormat,
                        (**args.offset(j as isize)).value,
                    ) == 0
                    {
                        *errorMessage =
                            b"Failed to parse argument\0" as *const u8 as *const libc::c_char;
                        *errorDetail = argName;
                        return 4 as libc::c_int;
                    }
                }
                if *args.offset(j as isize) == helpArg as *mut Arg {
                    foundHelp = 1 as libc::c_int != 0;
                }
                (**args.offset(j as isize)).found = 1 as libc::c_int != 0;
                foundMatch = 1 as libc::c_int != 0;
                break;
            }
        }
        if !foundMatch {
            *errorMessage = b"Unknown argument\0" as *const u8 as *const libc::c_char;
            return 5 as libc::c_int;
        }
        i += 1;
    }
    if !foundHelp {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < numArgs {
            if (**args.offset(i_0 as isize)).required as libc::c_int != 0
                && !(**args.offset(i_0 as isize)).found
            {
                *errorMessage = b"Required argument missing\0" as *const u8 as *const libc::c_char;
                *errorDetail = (**args.offset(i_0 as isize)).names[0 as libc::c_int as usize];
                return 6 as libc::c_int;
            }
            i_0 += 1;
        }
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn printHelp(
    mut out: *mut FILE,
    mut programName: *const libc::c_char,
    mut helpText: *const libc::c_char,
    mut numArgs: libc::c_int,
    mut args: *mut *mut Arg,
    mut errorMessage: *const libc::c_char,
    mut errorDetails: *const libc::c_char,
) {
    if !errorMessage.is_null() {
        fprintf(
            out,
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            programName,
            errorMessage,
        );
        if !errorDetails.is_null() {
            fprintf(
                out,
                b": %s\0" as *const u8 as *const libc::c_char,
                errorDetails,
            );
        }
        fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        out,
        b"%s: %s\n\0" as *const u8 as *const libc::c_char,
        programName,
        helpText,
    );
    fprintf(
        out,
        b"H3 %d.%d.%d\n\n\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numArgs {
        fprintf(out, b"\t\0" as *const u8 as *const libc::c_char);
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            if !((**args.offset(i as isize)).names[j as usize]).is_null() {
                if j != 0 as libc::c_int {
                    fprintf(out, b", \0" as *const u8 as *const libc::c_char);
                }
                fprintf(
                    out,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (**args.offset(i as isize)).names[j as usize],
                );
            }
            j += 1;
        }
        if !((**args.offset(i as isize)).scanFormat).is_null() {
            fprintf(
                out,
                b" <%s>\0" as *const u8 as *const libc::c_char,
                (**args.offset(i as isize)).valueName,
            );
        }
        fprintf(out, b"\t\0" as *const u8 as *const libc::c_char);
        if (**args.offset(i as isize)).required {
            fprintf(out, b"Required. \0" as *const u8 as *const libc::c_char);
        }
        fprintf(
            out,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            (**args.offset(i as isize)).helpText,
        );
        i += 1;
    }
}
