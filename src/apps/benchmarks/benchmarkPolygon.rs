
use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn bboxFromGeoLoop(loop_0: *const GeoLoop, bbox: *mut BBox);
    fn pointInsideGeoLoop(
        loop_0: *const GeoLoop,
        bbox: *const BBox,
        coord_0: *const LatLng,
    ) -> bool;
}
pub type __darwin_time_t = libc::c_long;
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
pub struct BBox {
    pub north: libc::c_double,
    pub south: libc::c_double,
    pub east: libc::c_double,
    pub west: libc::c_double,
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
pub static mut coord: LatLng = {
    let mut init = LatLng {
        lat: 0.6593020122281105f64,
        lng: -2.136646587644049f64,
    };
    init
};
#[no_mangle]
pub static mut smallVerts: [LatLng; 7] = [
    {
        let mut init = LatLng {
            lat: 0.6593216174404631f64,
            lng: -2.136686544190228f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592922488566673f64,
            lng: -2.1367052400423f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592659902057862f64,
            lng: -2.136683255590617f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592690997315311f64,
            lng: -2.136642575268508f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592984683133152f64,
            lng: -2.1366238782059224f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593247273713713f64,
            lng: -2.136645862675915f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593216174404631f64,
            lng: -2.136686544190228f64,
        };
        init
    },
];
#[no_mangle]
pub static mut smallGeoLoop: GeoLoop = GeoLoop {
    numVerts: 0,
    verts: 0 as *const LatLng as *mut LatLng,
};
#[no_mangle]
pub static mut smallBBox: BBox = BBox {
    north: 0.,
    south: 0.,
    east: 0.,
    west: 0.,
};
#[no_mangle]
pub static mut largeVerts: [LatLng; 91] = [
    {
        let mut init = LatLng {
            lat: 0.659094230575688f64,
            lng: -2.1371021015485354f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6590648582999955f64,
            lng: -2.137120785446624f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6590386025000088f64,
            lng: -2.1370988011284138f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6590417185683777f64,
            lng: -2.1370581328924323f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6590154614299164f64,
            lng: -2.1370361492494108f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6590185761584009f64,
            lng: -2.136995480477803f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589923176815468f64,
            lng: -2.136973497510139f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589954310701313f64,
            lng: -2.1369328282032267f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589691712549669f64,
            lng: -2.13691084591109f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589722833036354f64,
            lng: -2.136870176069197f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589460221502428f64,
            lng: -2.1368481944527566f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589491328589803f64,
            lng: -2.1368075240762017f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.658922870367441f64,
            lng: -2.136785543135628f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589259797362319f64,
            lng: -2.136744872224736f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6588997159066284f64,
            lng: -2.1367228919601997f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589028239354577f64,
            lng: -2.13668222051529f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.658876558767872f64,
            lng: -2.1366602409269593f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6588796654567243f64,
            lng: -2.136619568948354f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589090377185067f64,
            lng: -2.136600875366403f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589121434726317f64,
            lng: -2.136560201662654f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589415152070064f64,
            lng: -2.1365415063736677f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589446200261924f64,
            lng: -2.136500830944929f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589739912329232f64,
            lng: -2.1364821339488738f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6589770951169582f64,
            lng: -2.136441456795302f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659006465795809f64,
            lng: -2.136422758092143f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6590095687444818f64,
            lng: -2.1363820792138917f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6590389388952168f64,
            lng: -2.136363378803596f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6590420409083151f64,
            lng: -2.136322698200821f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6590714105306988f64,
            lng: -2.1363039960833543f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659074511608011f64,
            lng: -2.1362633137562117f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591038807018073f64,
            lng: -2.1362446099315373f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591069808431212f64,
            lng: -2.1362039258801833f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591363494080951f64,
            lng: -2.1361852203482696f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591626178344537f64,
            lng: -2.136207200077111f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591919858731505f64,
            lng: -2.136188494047296f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592182533686533f64,
            lng: -2.1362104744708654f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592476208809147f64,
            lng: -2.136191767943126f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592738874454308f64,
            lng: -2.136213749061465f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593032544310998f64,
            lng: -2.136195042035775f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593295200644976f64,
            lng: -2.136217023848928f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593588865234161f64,
            lng: -2.1361983163252596f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659385151225565f64,
            lng: -2.1362202988332704f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594145171575755f64,
            lng: -2.1362015908116003f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594407809283439f64,
            lng: -2.136223574014512f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594701463332887f64,
            lng: -2.1362048654948134f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594964091725459f64,
            lng: -2.136226849392669f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659525774050267f64,
            lng: -2.1362081403749165f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659552035957882f64,
            lng: -2.1362301249677595f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595489325807627f64,
            lng: -2.1362708185614188f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595751931496329f64,
            lng: -2.1362928038322995f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595720884311551f64,
            lng: -2.1363334968937866f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595983476611981f64,
            lng: -2.1363554828425366f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595952416013771f64,
            lng: -2.1363961753715284f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596214994925101f64,
            lng: -2.1364181619979767f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596183920913612f64,
            lng: -2.136458853994151f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659644648643503f64,
            lng: -2.1364808412981295f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596415399010412f64,
            lng: -2.1365215327611633f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596677951141098f64,
            lng: -2.1365435207425008f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596646850303514f64,
            lng: -2.1365842116720724f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659690938904264f64,
            lng: -2.136606200330602f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596878274792236f64,
            lng: -2.1366468907263876f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6597140800138991f64,
            lng: -2.136668880061937f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6597109672475924f64,
            lng: -2.136709569923617f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596816023515922f64,
            lng: -2.136728269257012f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596784886490965f64,
            lng: -2.136768957391624f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596491232239399f64,
            lng: -2.1367876550164606f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596460085854672f64,
            lng: -2.1368283414238483f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659616642631389f64,
            lng: -2.1368470373401607f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6596135270571513f64,
            lng: -2.1368877220201696f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595841605743876f64,
            lng: -2.1369064162279927f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595810440645967f64,
            lng: -2.1369470991804675f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595516770533838f64,
            lng: -2.1369657916798346f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595485596082516f64,
            lng: -2.137006472904621f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595191920688248f64,
            lng: -2.1370251636955686f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6595160736885637f64,
            lng: -2.137065843192512f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594867056211589f64,
            lng: -2.137084532275072f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594835863059806f64,
            lng: -2.1371252100440166f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594542177108332f64,
            lng: -2.137143897418225f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6594279684321879f64,
            lng: -2.137121908235114f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659398599310775f64,
            lng: -2.1371405951126565f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593723491001167f64,
            lng: -2.137118606624661f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593429794525963f64,
            lng: -2.137137293005565f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6593167283100556f64,
            lng: -2.1371153052126406f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592873581365857f64,
            lng: -2.137133991096931f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592611060622934f64,
            lng: -2.1371120039990363f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6592317353630313f64,
            lng: -2.1371306893867383f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659205482357119f64,
            lng: -2.137108702983829f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591761111322224f64,
            lng: -2.1371273878749673f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591498571948209f64,
            lng: -2.137105402167002f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.6591204854444472f64,
            lng: -2.137124086561602f64,
        };
        init
    },
    {
        let mut init = LatLng {
            lat: 0.659094230575688f64,
            lng: -2.1371021015485354f64,
        };
        init
    },
];
#[no_mangle]
pub static mut largeGeoLoop: GeoLoop = GeoLoop {
    numVerts: 0,
    verts: 0 as *const LatLng as *mut LatLng,
};
#[no_mangle]
pub static mut largeBBox: BBox = BBox {
    north: 0.,
    south: 0.,
    east: 0.,
    west: 0.,
};
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    smallGeoLoop.numVerts = 6 as libc::c_int;
    smallGeoLoop.verts = smallVerts.as_mut_ptr();
    bboxFromGeoLoop(&mut smallGeoLoop, &mut smallBBox);
    largeGeoLoop.numVerts = 90 as libc::c_int;
    largeGeoLoop.verts = largeVerts.as_mut_ptr();
    bboxFromGeoLoop(&mut largeGeoLoop, &mut largeBBox);
    let iterations: libc::c_int = 100000 as libc::c_int;
    let mut name: *const libc::c_char =
        b"pointInsideGeoLoopSmall\0" as *const u8 as *const libc::c_char;
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < iterations {
        pointInsideGeoLoop(&mut smallGeoLoop, &mut smallBBox, &mut coord);
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
    let iterations_0: libc::c_int = 100000 as libc::c_int;
    let mut name_0: *const libc::c_char =
        b"pointInsideGeoLoopLarge\0" as *const u8 as *const libc::c_char;
    let mut start_0: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_0);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < iterations_0 {
        pointInsideGeoLoop(&mut largeGeoLoop, &mut largeBBox, &mut coord);
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
    let duration_0: f64 = 
        (elapsed_0.tv_sec as libc::c_double * 1E9f64 + elapsed_0.tv_nsec as libc::c_double)
            / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_0,
        duration_0 / iterations_0 as f64,
        iterations_0,
    );
    let iterations_1: libc::c_int = 100000 as libc::c_int;
    let mut name_1: *const libc::c_char =
        b"bboxFromGeoLoopSmall\0" as *const u8 as *const libc::c_char;
    let mut start_1: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_1);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < iterations_1 {
        bboxFromGeoLoop(&mut smallGeoLoop, &mut smallBBox);
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
    let duration_1: f64 = 
        (elapsed_1.tv_sec as libc::c_double * 1E9f64 + elapsed_1.tv_nsec as libc::c_double)
            / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_1,
        duration_1 / iterations_1 as f64,
        iterations_1,
    );
    let iterations_2: libc::c_int = 100000 as libc::c_int;
    let mut name_2: *const libc::c_char =
        b"bboxFromGeoLoopLarge\0" as *const u8 as *const libc::c_char;
    let mut start_2: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(_CLOCK_MONOTONIC, &mut start_2);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < iterations_2 {
        bboxFromGeoLoop(&mut largeGeoLoop, &mut largeBBox);
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
    let duration_2: f64 = 
        (elapsed_2.tv_sec as libc::c_double * 1E9f64 + elapsed_2.tv_nsec as libc::c_double)
            / 1E3f64;
    printf(
        b"\t-- %s: %Lf microseconds per iteration (%d iterations)\n\0" as *const u8
            as *const libc::c_char,
        name_2,
        duration_2 / iterations_2 as f64,
        iterations_2,
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
