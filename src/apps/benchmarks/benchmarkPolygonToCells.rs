extern crate unsafe_h3lib_benchmarks;
use ::libc;
extern "C" {
    fn maxPolygonToCellsSize(
        geoPolygon: *const GeoPolygon,
        res: libc::c_int,
        flags: uint32_t,
        out: *mut int64_t,
    ) -> H3Error;
    fn polygonToCells(
        geoPolygon: *const GeoPolygon,
        res: libc::c_int,
        flags: uint32_t,
        out: *mut H3Index,
    ) -> H3Error;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
}
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type __darwin_time_t = libc::c_long;
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
pub struct GeoLoop {
    pub numVerts: libc::c_int,
    pub verts: *mut LatLng,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoPolygon {
    pub geoloop: GeoLoop,
    pub numHoles: libc::c_int,
    pub holes: *mut GeoLoop,
}
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
#[no_mangle]
pub static mut sfVerts: [LatLng; 6] = [
    {
        let mut init = LatLng {
            lat: 0.659966917655f64,
            lng: -2.1364398519396f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595011102219f64,
            lng: -2.1359434279405f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6583348114025f64,
            lng: -2.1354884206045f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6581220034068f64,
            lng: -2.1382437718946f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594479998527f64,
            lng: -2.1384597563896f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6599990002976f64,
            lng: -2.1376771158464f64,
        };
        init
    },
];
#[no_mangle]
pub static mut sfGeoLoop: GeoLoop = GeoLoop {
    numVerts: 0,
    verts: std::ptr::null::<LatLng>() as *mut LatLng,
};
#[no_mangle]
pub static mut sfGeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
#[no_mangle]
pub static mut alamedaVerts: [LatLng; 50] = [
    {
        let mut init = LatLng {
            lat: 0.6597959342671712f64,
            lng: -2.133241848488897f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6597959348850178f64,
            lng: -2.133241848495878f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6598352639563587f64,
            lng: -2.1331688423977755f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6601346536539207f64,
            lng: -2.13270417124178f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6601594763880223f64,
            lng: -2.1326680320633344f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6601512007732382f64,
            lng: -2.1326594176574534f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6598535076212304f64,
            lng: -2.1323049630593562f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596565748646488f64,
            lng: -2.132069889917591f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594645035394391f64,
            lng: -2.131843148468039f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593438094209757f64,
            lng: -2.1316994860539844f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591174422311021f64,
            lng: -2.131429776816562f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.658849344286881f64,
            lng: -2.1311111485483867f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6588348862079956f64,
            lng: -2.1310988536794455f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6586273138317915f64,
            lng: -2.131668420800747f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6583729538174264f64,
            lng: -2.132370426573979f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6582479206289285f64,
            lng: -2.132718691911663f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6582322393220743f64,
            lng: -2.1327614200082317f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6583003647098981f64,
            lng: -2.132837478687196f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6584457274847966f64,
            lng: -2.132827956758973f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6585526679060995f64,
            lng: -2.1330231566043203f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6587379099516777f64,
            lng: -2.1331602726234538f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6587273684736642f64,
            lng: -2.1332676321559063f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6584638025857692f64,
            lng: -2.133305719954319f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6583545950288919f64,
            lng: -2.1334323622944993f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6584427148370682f64,
            lng: -2.1335885223323947f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6584715236640714f64,
            lng: -2.133649780409862f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6584715242505019f64,
            lng: -2.133649780481421f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.658474662092443f64,
            lng: -2.1336459234695804f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591666596433436f64,
            lng: -2.1348354004882926f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591809355063646f64,
            lng: -2.1348424115474565f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593477498700266f64,
            lng: -2.1351460576998926f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6597155087395117f64,
            lng: -2.1351049454274f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6597337410387994f64,
            lng: -2.135113899444683f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6598277083823935f64,
            lng: -2.1351065432309517f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659837290351688f64,
            lng: -2.1350919904836627f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6598391300107502f64,
            lng: -2.1350911731005957f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6598335712627461f64,
            lng: -2.1350732321630828f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6597162034032434f64,
            lng: -2.134664026354221f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596785831942451f64,
            lng: -2.134651647657116f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596627824684727f64,
            lng: -2.13458880305965f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596785832500957f64,
            lng: -2.134530719130462f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596093592822273f64,
            lng: -2.13428052987356f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596116166352313f64,
            lng: -2.134221493755564f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595973199434513f64,
            lng: -2.134146270344056f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595536764042369f64,
            lng: -2.1340805688066653f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594611172376618f64,
            lng: -2.133753252031165f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594829406269346f64,
            lng: -2.1337342082305697f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594897134102581f64,
            lng: -2.1337104032834757f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6597920983773051f64,
            lng: -2.1332343063312775f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6597959342671712f64,
            lng: -2.133241848488897f64,
        };
        init
    },
];
#[no_mangle]
pub static mut alamedaGeoLoop: GeoLoop = GeoLoop {
    numVerts: 0,
    verts: std::ptr::null::<LatLng>() as *mut LatLng,
};
#[no_mangle]
pub static mut alamedaGeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
#[no_mangle]
pub static mut southernVerts: [LatLng; 23] = [
    {
        let mut init = LatLng {
            lat: 0.6367481147484843f64,
            lng: -2.1290865397798906f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6367481152301953f64,
            lng: -2.129086539469222f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6367550754426818f64,
            lng: -2.128887436716856f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6367816002113981f64,
            lng: -2.1273204058681094f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6380814125349741f64,
            lng: -2.127201274803692f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6388614350074809f64,
            lng: -2.12552061082428f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6393520289210095f64,
            lng: -2.124274316938293f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.639524834205869f64,
            lng: -2.122168447308359f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6405714857447717f64,
            lng: -2.122083222593005f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.640769478635285f64,
            lng: -2.120979885974894f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6418936996869471f64,
            lng: -2.1147667448862255f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6419094141707652f64,
            lng: -2.1146521242709584f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6269997808948107f64,
            lng: -2.1038647304637257f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6252080524974937f64,
            lng: -2.1195521728170457f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.626379700264057f64,
            lng: -2.1203708632511162f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6282200029232767f64,
            lng: -2.1210412050690723f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6283657301211779f64,
            lng: -2.1219496416754393f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6305651783819565f64,
            lng: -2.123628532238016f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6308259852882764f64,
            lng: -2.124225549648211f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6317049665784865f64,
            lng: -2.124887756638367f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6323403882676475f64,
            lng: -2.1266205835454053f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6334397909415498f64,
            lng: -2.1277211741619553f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6367481147484843f64,
            lng: -2.1290865397798906f64,
        };
        init
    },
];
#[no_mangle]
pub static mut southernGeoLoop: GeoLoop = GeoLoop {
    numVerts: 0,
    verts: std::ptr::null::<LatLng>() as *mut LatLng,
};
#[no_mangle]
pub static mut southernGeoPolygon: GeoPolygon = GeoPolygon {
    geoloop: GeoLoop {
        numVerts: 0,
        verts: std::ptr::null::<LatLng>() as *mut LatLng,
    },
    numHoles: 0,
    holes: std::ptr::null::<GeoLoop>() as *mut GeoLoop,
};
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    sfGeoLoop.numVerts = 6 as libc::c_int;
    sfGeoLoop.verts = sfVerts.as_mut_ptr();
    sfGeoPolygon.geoloop = sfGeoLoop;
    alamedaGeoLoop.numVerts = 50 as libc::c_int;
    alamedaGeoLoop.verts = alamedaVerts.as_mut_ptr();
    alamedaGeoPolygon.geoloop = alamedaGeoLoop;
    southernGeoLoop.numVerts = 23 as libc::c_int;
    southernGeoLoop.verts = southernVerts.as_mut_ptr();
    southernGeoPolygon.geoloop = southernGeoLoop;
    let mut numHexagons: int64_t = 0;
    let mut hexagons: *mut H3Index = 0 as *mut H3Index;
    let iterations: libc::c_int = 500 as libc::c_int;
    let mut name: *const libc::c_char = b"polygonToCellsSF\0" as *const u8 as *const libc::c_char;
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < iterations {
        maxPolygonToCellsSize(
            &mut sfGeoPolygon,
            9 as libc::c_int,
            0 as libc::c_int as uint32_t,
            &mut numHexagons,
        );
        hexagons = calloc(
            numHexagons as libc::c_ulong,
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
        ) as *mut H3Index;
        polygonToCells(
            &mut sfGeoPolygon,
            9 as libc::c_int,
            0 as libc::c_int as uint32_t,
            hexagons,
        );
        free(hexagons as *mut libc::c_void);
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
    let iterations_0: libc::c_int = 500 as libc::c_int;
    let mut name_0: *const libc::c_char =
        b"polygonToCellsAlameda\0" as *const u8 as *const libc::c_char;
    let mut start_0: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_0);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < iterations_0 {
        maxPolygonToCellsSize(
            &mut alamedaGeoPolygon,
            9 as libc::c_int,
            0 as libc::c_int as uint32_t,
            &mut numHexagons,
        );
        hexagons = calloc(
            numHexagons as libc::c_ulong,
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
        ) as *mut H3Index;
        polygonToCells(
            &mut alamedaGeoPolygon,
            9 as libc::c_int,
            0 as libc::c_int as uint32_t,
            hexagons,
        );
        free(hexagons as *mut libc::c_void);
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
    let iterations_1: libc::c_int = 10 as libc::c_int;
    let mut name_1: *const libc::c_char =
        b"polygonToCellsSouthernExpansion\0" as *const u8 as *const libc::c_char;
    let mut start_1: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_1);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < iterations_1 {
        maxPolygonToCellsSize(
            &mut southernGeoPolygon,
            9 as libc::c_int,
            0 as libc::c_int as uint32_t,
            &mut numHexagons,
        );
        hexagons = calloc(
            numHexagons as libc::c_ulong,
            ::core::mem::size_of::<H3Index>() as libc::c_ulong,
        ) as *mut H3Index;
        polygonToCells(
            &mut southernGeoPolygon,
            9 as libc::c_int,
            0 as libc::c_int as uint32_t,
            hexagons,
        );
        free(hexagons as *mut libc::c_void);
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
