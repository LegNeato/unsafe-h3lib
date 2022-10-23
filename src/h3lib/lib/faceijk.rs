use ::libc;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn _setIJK(ijk: *mut CoordIJK, i: libc::c_int, j: libc::c_int, k: libc::c_int);
    fn _ijkRotate60cw(ijk: *mut CoordIJK);
    fn _ijkRotate60ccw(ijk: *mut CoordIJK);
    fn _downAp3r(ijk: *mut CoordIJK);
    fn _downAp3(ijk: *mut CoordIJK);
    fn _downAp7r(ijk: *mut CoordIJK);
    fn _ijkNormalize(c: *mut CoordIJK);
    fn _ijkScale(c: *mut CoordIJK, factor: libc::c_int);
    fn _ijkSub(h1: *const CoordIJK, h2: *const CoordIJK, diff: *mut CoordIJK);
    fn _ijkAdd(h1: *const CoordIJK, h2: *const CoordIJK, sum: *mut CoordIJK);
    fn _ijkToHex2d(h: *const CoordIJK, v: *mut Vec2d);
    fn _hex2dToCoordIJK(v: *const Vec2d, h: *mut CoordIJK);
    fn _v2dAlmostEquals(p0: *const Vec2d, p1: *const Vec2d) -> bool;
    fn _v2dIntersect(
        p0: *const Vec2d,
        p1: *const Vec2d,
        p2: *const Vec2d,
        p3: *const Vec2d,
        inter: *mut Vec2d,
    );
    fn _v2dMag(v: *const Vec2d) -> libc::c_double;
    fn _geoAzDistanceRads(
        p1: *const LatLng,
        az: libc::c_double,
        distance: libc::c_double,
        p2: *mut LatLng,
    );
    fn _geoAzimuthRads(p1: *const LatLng, p2: *const LatLng) -> libc::c_double;
    fn _posAngleRads(rads: libc::c_double) -> libc::c_double;
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn isResolutionClassIII(r: libc::c_int) -> libc::c_int;
    fn _geoToVec3d(geo: *const LatLng, point: *mut Vec3d);
    fn _pointSquareDist(p1: *const Vec3d, p2: *const Vec3d) -> libc::c_double;
}
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
pub struct Vec2d {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoordIJK {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub k: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaceIJK {
    pub face: libc::c_int,
    pub coord: CoordIJK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaceOrientIJK {
    pub face: libc::c_int,
    pub translate: CoordIJK,
    pub ccwRot60: libc::c_int,
}
pub type Overage = libc::c_uint;
pub const NEW_FACE: Overage = 2;
pub const FACE_EDGE: Overage = 1;
pub const NO_OVERAGE: Overage = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3d {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
}
#[no_mangle]
pub static mut faceCenterGeo: [LatLng; 20] = [
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
static mut faceCenterPoint: [Vec3d; 20] = [
    {
        Vec3d {
            x: 0.2199307791404606f64,
            y: 0.6583691780274996f64,
            z: 0.7198475378926182f64,
        }
    },
    {
        Vec3d {
            x: -0.2139234834501421f64,
            y: 0.1478171829550703f64,
            z: 0.9656017935214205f64,
        }
    },
    {
        Vec3d {
            x: 0.1092625278784797f64,
            y: -0.481_195_157_287_321_f64,
            z: 0.8697775121287253f64,
        }
    },
    {
        Vec3d {
            x: 0.7428567301586791f64,
            y: -0.3593941678278028f64,
            z: 0.5648005936517033f64,
        }
    },
    {
        Vec3d {
            x: 0.8112534709140969f64,
            y: 0.3448953237639384f64,
            z: 0.472_138_773_641_393_f64,
        }
    },
    {
        Vec3d {
            x: -0.1055498149613921f64,
            y: 0.9794457296411413f64,
            z: 0.1718874610009365f64,
        }
    },
    {
        Vec3d {
            x: -0.8075407579970092f64,
            y: 0.1533552485898818f64,
            z: 0.5695261994882688f64,
        }
    },
    {
        Vec3d {
            x: -0.2846148069787907f64,
            y: -0.8644080972654206f64,
            z: 0.4144792552473539f64,
        }
    },
    {
        Vec3d {
            x: 0.7405621473854482f64,
            y: -0.6673299564565524f64,
            z: -0.0789837646326737f64,
        }
    },
    {
        Vec3d {
            x: 0.8512303986474293f64,
            y: 0.4722343788582681f64,
            z: -0.2289137388687808f64,
        }
    },
    {
        Vec3d {
            x: -0.7405621473854481f64,
            y: 0.6673299564565524f64,
            z: 0.0789837646326737f64,
        }
    },
    {
        Vec3d {
            x: -0.8512303986474292f64,
            y: -0.4722343788582682f64,
            z: 0.2289137388687808f64,
        }
    },
    {
        Vec3d {
            x: 0.1055498149613919f64,
            y: -0.9794457296411413f64,
            z: -0.1718874610009365f64,
        }
    },
    {
        Vec3d {
            x: 0.8075407579970092f64,
            y: -0.1533552485898819f64,
            z: -0.5695261994882688f64,
        }
    },
    {
        Vec3d {
            x: 0.2846148069787908f64,
            y: 0.8644080972654204f64,
            z: -0.4144792552473539f64,
        }
    },
    {
        Vec3d {
            x: -0.7428567301586791f64,
            y: 0.3593941678278027f64,
            z: -0.5648005936517033f64,
        }
    },
    {
        Vec3d {
            x: -0.811_253_470_914_097_f64,
            y: -0.3448953237639382f64,
            z: -0.472_138_773_641_393_f64,
        }
    },
    {
        Vec3d {
            x: -0.2199307791404607f64,
            y: -0.6583691780274996f64,
            z: -0.7198475378926182f64,
        }
    },
    {
        Vec3d {
            x: 0.213_923_483_450_142_f64,
            y: -0.1478171829550704f64,
            z: -0.9656017935214205f64,
        }
    },
    {
        Vec3d {
            x: -0.1092625278784796f64,
            y: 0.481_195_157_287_321_f64,
            z: -0.8697775121287253f64,
        }
    },
];
static mut faceAxesAzRadsCII: [[libc::c_double; 3]; 20] = [
    [
        5.619_958_268_523_939_5_f64,
        3.525_563_166_130_744_7_f64,
        1.431_168_063_737_548_8_f64,
    ],
    [
        5.760_339_081_714_187_5_f64,
        3.665_943_979_320_992_f64,
        1.571_548_876_927_796_f64,
    ],
    [
        0.780_213_654_393_43_f64,
        4.969_003_859_179_821_f64,
        2.874_608_756_786_625_6_f64,
    ],
    [
        0.430_469_363_979_999_9_f64,
        4.619_259_568_766_391_f64,
        2.524_864_466_373_195_6_f64,
    ],
    [
        6.130_269_123_335_111_f64,
        4.035_874_020_941_915_5_f64,
        1.941_478_918_548_720_2_f64,
    ],
    [
        2.692_877_706_530_643_f64,
        0.598_482_604_137_447_1_f64,
        4.787_272_808_923_838_f64,
    ],
    [
        2.982_963_003_477_244_f64,
        0.888_567_901_084_048_4_f64,
        5.077_358_105_870_44_f64,
    ],
    [
        3.532_912_002_790_141_f64,
        1.438_516_900_396_945_6_f64,
        5.627_307_105_183_337_f64,
    ],
    [
        3.494_305_004_259_568_f64,
        1.399_909_901_866_372_8_f64,
        5.588_700_106_652_764_f64,
    ],
    [
        3.003_214_169_499_538_2_f64,
        0.908_819_067_106_343_f64,
        5.097_609_271_892_733_5_f64,
    ],
    [
        5.930_472_956_509_812_f64,
        3.836_077_854_116_616_f64,
        1.741_682_751_723_420_4_f64,
    ],
    [
        0.138_378_484_090_254_86_f64,
        4.327_168_688_876_646_f64,
        2.232_773_586_483_45_f64,
    ],
    [
        0.448_714_947_059_150_4_f64,
        4.637_505_151_845_541_5_f64,
        2.543_110_049_452_346_f64,
    ],
    [
        0.158_629_650_112_549_37_f64,
        4.347_419_854_898_940_5_f64,
        2.253_024_752_505_744_7_f64,
    ],
    [
        5.891_865_957_979_238_f64,
        3.797_470_855_586_043_f64,
        1.703_075_753_192_847_5_f64,
    ],
    [
        2.711_123_289_609_793_f64,
        0.616_728_187_216_597_7_f64,
        4.805_518_392_002_988_5_f64,
    ],
    [
        3.294_508_837_434_268_f64,
        1.200_113_735_041_072_9_f64,
        5.388_903_939_827_464_f64,
    ],
    [
        3.804_819_692_245_44_f64,
        1.710_424_589_852_244_5_f64,
        5.899_214_794_638_635_5_f64,
    ],
    [
        3.664_438_879_055_192_3_f64,
        1.570_043_776_661_997_f64,
        5.758_833_981_448_388_f64,
    ],
    [
        2.361_378_999_196_363_f64,
        0.266_983_896_803_167_6_f64,
        4.455_774_101_589_559_f64,
    ],
];
static mut faceNeighbors: [[FaceOrientIJK; 4]; 20] = [
    [
        {
            FaceOrientIJK {
                face: 0 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 4 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 1 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 5 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 1 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 0 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 2 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 6 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 2 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 1 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 3 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 7 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 3 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 2 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 4 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 8 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 4 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 3 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 0 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 9 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 5 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 10 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 14 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 0 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 6 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 11 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 10 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 1 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 7 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 12 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 11 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 2 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 8 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 13 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 12 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 3 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 9 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 14 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 13 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 4 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 10 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 5 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 6 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 15 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 11 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 6 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 7 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 16 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 12 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 7 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 8 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 17 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 13 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 8 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 9 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 18 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 14 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 9 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 5 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 19 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 15 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 16 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 19 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 10 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 16 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 17 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 15 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 11 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 17 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 18 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 16 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 12 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 18 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 19 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 17 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 13 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
    [
        {
            FaceOrientIJK {
                face: 19 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 0 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 15 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 1 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 18 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    }
                },
                ccwRot60: 5 as libc::c_int,
            }
        },
        {
            FaceOrientIJK {
                face: 14 as libc::c_int,
                translate: {
                    CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    }
                },
                ccwRot60: 3 as libc::c_int,
            }
        },
    ],
];
static mut adjacentFaceDir: [[libc::c_int; 20]; 20] = [
    [
        0 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        3 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        2 as libc::c_int,
        0 as libc::c_int,
    ],
];
static mut maxDimByCIIres: [libc::c_int; 17] = [
    2 as libc::c_int,
    -(1 as libc::c_int),
    14 as libc::c_int,
    -(1 as libc::c_int),
    98 as libc::c_int,
    -(1 as libc::c_int),
    686 as libc::c_int,
    -(1 as libc::c_int),
    4802 as libc::c_int,
    -(1 as libc::c_int),
    33614 as libc::c_int,
    -(1 as libc::c_int),
    235298 as libc::c_int,
    -(1 as libc::c_int),
    1647086 as libc::c_int,
    -(1 as libc::c_int),
    11529602 as libc::c_int,
];
static mut unitScaleByCIIres: [libc::c_int; 17] = [
    1 as libc::c_int,
    -(1 as libc::c_int),
    7 as libc::c_int,
    -(1 as libc::c_int),
    49 as libc::c_int,
    -(1 as libc::c_int),
    343 as libc::c_int,
    -(1 as libc::c_int),
    2401 as libc::c_int,
    -(1 as libc::c_int),
    16807 as libc::c_int,
    -(1 as libc::c_int),
    117649 as libc::c_int,
    -(1 as libc::c_int),
    823543 as libc::c_int,
    -(1 as libc::c_int),
    5764801 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn _geoToFaceIjk(
    mut g: *const LatLng,
    mut res: libc::c_int,
    mut h: *mut FaceIJK,
) {
    let mut v: Vec2d = Vec2d { x: 0., y: 0. };
    _geoToHex2d(g, res, &mut (*h).face, &mut v);
    _hex2dToCoordIJK(&v, &mut (*h).coord);
}
#[no_mangle]
pub unsafe extern "C" fn _geoToHex2d(
    mut g: *const LatLng,
    mut res: libc::c_int,
    mut face: *mut libc::c_int,
    mut v: *mut Vec2d,
) {
    let mut sqd: libc::c_double = 0.;
    _geoToClosestFace(g, face, &mut sqd);
    let mut r: libc::c_double =
        acos(1 as libc::c_int as libc::c_double - sqd / 2 as libc::c_int as libc::c_double);
    if r < 0.0000000000000001 {
        (*v).y = (0.0).to_f64().unwrap();
        (*v).x = (*v).y;
        return;
    }
    let mut theta: libc::c_double = _posAngleRads(
        faceAxesAzRadsCII[*face as usize][0 as libc::c_int as usize]
            - _posAngleRads(_geoAzimuthRads(
                &*faceCenterGeo.as_ptr().offset(*face as isize),
                g,
            )),
    );
    if isResolutionClassIII(res) != 0 {
        theta = _posAngleRads((theta - 0.333_473_172_251_832_1).to_f64().unwrap());
    }
    r = tan(r);
    r = (r / 0.381_966_011_250_105).to_f64().unwrap();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < res {
        r = (r * 2.645_751_311_064_590_7).to_f64().unwrap();
        i += 1;
    }
    (*v).x = r * cos(theta);
    (*v).y = r * sin(theta);
}
#[no_mangle]
pub unsafe extern "C" fn _hex2dToGeo(
    mut v: *const Vec2d,
    mut face: libc::c_int,
    mut res: libc::c_int,
    mut substrate: libc::c_int,
    mut g: *mut LatLng,
) {
    let mut r: libc::c_double = _v2dMag(v);
    if r < 0.0000000000000001 {
        *g = faceCenterGeo[face as usize];
        return;
    }
    let mut theta: libc::c_double = atan2((*v).y, (*v).x);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < res {
        r = (r / 2.645_751_311_064_590_7).to_f64().unwrap();
        i += 1;
    }
    if substrate != 0 {
        r /= 3.0f64;
        if isResolutionClassIII(res) != 0 {
            r = (r / 2.645_751_311_064_590_7).to_f64().unwrap();
        }
    }
    r = (r * 0.381_966_011_250_105).to_f64().unwrap();
    r = atan(r);
    if substrate == 0 && isResolutionClassIII(res) != 0 {
        theta = _posAngleRads((theta + 0.333_473_172_251_832_1).to_f64().unwrap());
    }
    theta = _posAngleRads(faceAxesAzRadsCII[face as usize][0 as libc::c_int as usize] - theta);
    _geoAzDistanceRads(&*faceCenterGeo.as_ptr().offset(face as isize), theta, r, g);
}
#[no_mangle]
pub unsafe extern "C" fn _faceIjkToGeo(
    mut h: *const FaceIJK,
    mut res: libc::c_int,
    mut g: *mut LatLng,
) {
    let mut v: Vec2d = Vec2d { x: 0., y: 0. };
    _ijkToHex2d(&(*h).coord, &mut v);
    _hex2dToGeo(&v, (*h).face, res, 0 as libc::c_int, g);
}
#[no_mangle]
pub unsafe extern "C" fn _faceIjkPentToCellBoundary(
    mut h: *const FaceIJK,
    mut res: libc::c_int,
    mut start: libc::c_int,
    mut length: libc::c_int,
    mut g: *mut CellBoundary,
) {
    let mut adjRes: libc::c_int = res;
    let mut centerIJK: FaceIJK = *h;
    let mut fijkVerts: [FaceIJK; 5] = [FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    }; 5];
    _faceIjkPentToVerts(&mut centerIJK, &mut adjRes, fijkVerts.as_mut_ptr());
    let mut additionalIteration: libc::c_int = if length == 5 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*g).numVerts = 0 as libc::c_int;
    let mut lastFijk: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    let mut vert: libc::c_int = start;
    while vert < start + length + additionalIteration {
        let mut v: libc::c_int = vert % 5 as libc::c_int;
        let mut fijk: FaceIJK = fijkVerts[v as usize];
        _adjustPentVertOverage(&mut fijk, adjRes);
        if isResolutionClassIII(res) != 0 && vert > start {
            let mut tmpFijk: FaceIJK = fijk;
            let mut orig2d0: Vec2d = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(&lastFijk.coord, &mut orig2d0);
            let mut currentToLastDir: libc::c_int =
                adjacentFaceDir[tmpFijk.face as usize][lastFijk.face as usize];
            let mut fijkOrient: *const FaceOrientIJK =
                &*(*faceNeighbors.as_ptr().offset(tmpFijk.face as isize))
                    .as_ptr()
                    .offset(currentToLastDir as isize) as *const FaceOrientIJK;
            tmpFijk.face = (*fijkOrient).face;
            let mut ijk: *mut CoordIJK = &mut tmpFijk.coord;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < (*fijkOrient).ccwRot60 {
                _ijkRotate60ccw(ijk);
                i += 1;
            }
            let mut transVec: CoordIJK = (*fijkOrient).translate;
            _ijkScale(
                &mut transVec,
                unitScaleByCIIres[adjRes as usize] * 3 as libc::c_int,
            );
            _ijkAdd(ijk, &transVec, ijk);
            _ijkNormalize(ijk);
            let mut orig2d1: Vec2d = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(ijk, &mut orig2d1);
            let mut maxDim: libc::c_int = maxDimByCIIres[adjRes as usize];
            let mut v0: Vec2d = {
                Vec2d {
                    x: 3.0f64 * maxDim as libc::c_double,
                    y: 0.0f64,
                }
            };
            let mut v1: Vec2d = {
                Vec2d {
                    x: -1.5f64 * maxDim as libc::c_double,
                    y: (3.0f64 * 0.866_025_403_784_438_6 * maxDim as f64)
                        .to_f64()
                        .unwrap(),
                }
            };
            let mut v2: Vec2d = {
                Vec2d {
                    x: -1.5f64 * maxDim as libc::c_double,
                    y: (-3.0f64 * 0.866_025_403_784_438_6 * maxDim as f64)
                        .to_f64()
                        .unwrap(),
                }
            };
            let mut edge0: *mut Vec2d = std::ptr::null_mut::<Vec2d>();
            let mut edge1: *mut Vec2d = std::ptr::null_mut::<Vec2d>();
            match adjacentFaceDir[tmpFijk.face as usize][fijk.face as usize] {
                1 => {
                    edge0 = &mut v0;
                    edge1 = &mut v1;
                }
                3 => {
                    edge0 = &mut v1;
                    edge1 = &mut v2;
                }
                _ => {
                    if i64::from(adjacentFaceDir[tmpFijk.face as usize][fijk.face as usize]) != 2 {
                        __assert_rtn(
                            (*::core::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                                b"_faceIjkPentToCellBoundary\0",
                            ))
                            .as_ptr(),
                            b"faceijk.c\0" as *const u8 as *const libc::c_char,
                            571 as libc::c_int,
                            b"adjacentFaceDir[tmpFijk.face][fijk.face] == KI\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                    };
                    edge0 = &mut v2;
                    edge1 = &mut v0;
                }
            }
            let mut inter: Vec2d = Vec2d { x: 0., y: 0. };
            _v2dIntersect(&orig2d0, &orig2d1, edge0, edge1, &mut inter);
            _hex2dToGeo(
                &inter,
                tmpFijk.face,
                adjRes,
                1 as libc::c_int,
                &mut *((*g).verts).as_mut_ptr().offset((*g).numVerts as isize),
            );
            (*g).numVerts += 1;
        }
        if vert < start + 5 as libc::c_int {
            let mut vec: Vec2d = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(&fijk.coord, &mut vec);
            _hex2dToGeo(
                &vec,
                fijk.face,
                adjRes,
                1 as libc::c_int,
                &mut *((*g).verts).as_mut_ptr().offset((*g).numVerts as isize),
            );
            (*g).numVerts += 1;
        }
        lastFijk = fijk;
        vert += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _faceIjkPentToVerts(
    mut fijk: *mut FaceIJK,
    mut res: *mut libc::c_int,
    mut fijkVerts: *mut FaceIJK,
) {
    let mut vertsCII: [CoordIJK; 5] = [
        {
            CoordIJK {
                i: 2 as libc::c_int,
                j: 1 as libc::c_int,
                k: 0 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 1 as libc::c_int,
                j: 2 as libc::c_int,
                k: 0 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 0 as libc::c_int,
                j: 2 as libc::c_int,
                k: 1 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 0 as libc::c_int,
                j: 1 as libc::c_int,
                k: 2 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 1 as libc::c_int,
                j: 0 as libc::c_int,
                k: 2 as libc::c_int,
            }
        },
    ];
    let mut vertsCIII: [CoordIJK; 5] = [
        {
            CoordIJK {
                i: 5 as libc::c_int,
                j: 4 as libc::c_int,
                k: 0 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 1 as libc::c_int,
                j: 5 as libc::c_int,
                k: 0 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 0 as libc::c_int,
                j: 5 as libc::c_int,
                k: 4 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 0 as libc::c_int,
                j: 1 as libc::c_int,
                k: 5 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 4 as libc::c_int,
                j: 0 as libc::c_int,
                k: 5 as libc::c_int,
            }
        },
    ];
    let mut verts: *mut CoordIJK = std::ptr::null_mut::<CoordIJK>();
    if isResolutionClassIII(*res) != 0 {
        verts = vertsCIII.as_mut_ptr();
    } else {
        verts = vertsCII.as_mut_ptr();
    }
    _downAp3(&mut (*fijk).coord);
    _downAp3r(&mut (*fijk).coord);
    if isResolutionClassIII(*res) != 0 {
        _downAp7r(&mut (*fijk).coord);
        *res += 1 as libc::c_int;
    }
    let mut v: libc::c_int = 0 as libc::c_int;
    while v < 5 as libc::c_int {
        (*fijkVerts.offset(v as isize)).face = (*fijk).face;
        _ijkAdd(
            &(*fijk).coord,
            &*verts.offset(v as isize),
            &mut (*fijkVerts.offset(v as isize)).coord,
        );
        _ijkNormalize(&mut (*fijkVerts.offset(v as isize)).coord);
        v += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _faceIjkToCellBoundary(
    mut h: *const FaceIJK,
    mut res: libc::c_int,
    mut start: libc::c_int,
    mut length: libc::c_int,
    mut g: *mut CellBoundary,
) {
    let mut adjRes: libc::c_int = res;
    let mut centerIJK: FaceIJK = *h;
    let mut fijkVerts: [FaceIJK; 6] = [FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    }; 6];
    _faceIjkToVerts(&mut centerIJK, &mut adjRes, fijkVerts.as_mut_ptr());
    let mut additionalIteration: libc::c_int = if length == 6 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*g).numVerts = 0 as libc::c_int;
    let mut lastFace: libc::c_int = -(1 as libc::c_int);
    let mut lastOverage: Overage = NO_OVERAGE;
    let mut vert: libc::c_int = start;
    while vert < start + length + additionalIteration {
        let mut v: libc::c_int = vert % 6 as libc::c_int;
        let mut fijk: FaceIJK = fijkVerts[v as usize];
        let pentLeading4: libc::c_int = 0 as libc::c_int;
        let mut overage: Overage =
            _adjustOverageClassII(&mut fijk, adjRes, pentLeading4, 1 as libc::c_int);
        if isResolutionClassIII(res) != 0
            && vert > start
            && fijk.face != lastFace
            && lastOverage as libc::c_uint != FACE_EDGE as libc::c_int as libc::c_uint
        {
            let mut lastV: libc::c_int = (v + 5 as libc::c_int) % 6 as libc::c_int;
            let mut orig2d0: Vec2d = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(
                &(*fijkVerts.as_mut_ptr().offset(lastV as isize)).coord,
                &mut orig2d0,
            );
            let mut orig2d1: Vec2d = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(
                &(*fijkVerts.as_mut_ptr().offset(v as isize)).coord,
                &mut orig2d1,
            );
            let mut maxDim: libc::c_int = maxDimByCIIres[adjRes as usize];
            let mut v0: Vec2d = {
                Vec2d {
                    x: 3.0f64 * maxDim as libc::c_double,
                    y: 0.0f64,
                }
            };
            let mut v1: Vec2d = {
                Vec2d {
                    x: -1.5f64 * maxDim as libc::c_double,
                    y: (3.0f64 * 0.866_025_403_784_438_6 * maxDim as f64)
                        .to_f64()
                        .unwrap(),
                }
            };
            let mut v2: Vec2d = {
                Vec2d {
                    x: -1.5f64 * maxDim as libc::c_double,
                    y: (-3.0f64 * 0.866_025_403_784_438_6 * maxDim as f64)
                        .to_f64()
                        .unwrap(),
                }
            };
            let mut face2: libc::c_int = if lastFace == centerIJK.face {
                fijk.face
            } else {
                lastFace
            };
            let mut edge0: *mut Vec2d = std::ptr::null_mut::<Vec2d>();
            let mut edge1: *mut Vec2d = std::ptr::null_mut::<Vec2d>();
            match adjacentFaceDir[centerIJK.face as usize][face2 as usize] {
                1 => {
                    edge0 = &mut v0;
                    edge1 = &mut v1;
                }
                3 => {
                    edge0 = &mut v1;
                    edge1 = &mut v2;
                }
                _ => {
                    if i64::from(adjacentFaceDir[centerIJK.face as usize][face2 as usize]) != 2 {
                        __assert_rtn(
                            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                                b"_faceIjkToCellBoundary\0",
                            ))
                            .as_ptr(),
                            b"faceijk.c\0" as *const u8 as *const libc::c_char,
                            736 as libc::c_int,
                            b"adjacentFaceDir[centerIJK.face][face2] == KI\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                    };
                    edge0 = &mut v2;
                    edge1 = &mut v0;
                }
            }
            let mut inter: Vec2d = Vec2d { x: 0., y: 0. };
            _v2dIntersect(&orig2d0, &orig2d1, edge0, edge1, &mut inter);
            let mut isIntersectionAtVertex: bool =
                _v2dAlmostEquals(&orig2d0, &inter) as libc::c_int != 0
                    || _v2dAlmostEquals(&orig2d1, &inter) as libc::c_int != 0;
            if !isIntersectionAtVertex {
                _hex2dToGeo(
                    &inter,
                    centerIJK.face,
                    adjRes,
                    1 as libc::c_int,
                    &mut *((*g).verts).as_mut_ptr().offset((*g).numVerts as isize),
                );
                (*g).numVerts += 1;
            }
        }
        if vert < start + 6 as libc::c_int {
            let mut vec: Vec2d = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(&fijk.coord, &mut vec);
            _hex2dToGeo(
                &vec,
                fijk.face,
                adjRes,
                1 as libc::c_int,
                &mut *((*g).verts).as_mut_ptr().offset((*g).numVerts as isize),
            );
            (*g).numVerts += 1;
        }
        lastFace = fijk.face;
        lastOverage = overage;
        vert += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _faceIjkToVerts(
    mut fijk: *mut FaceIJK,
    mut res: *mut libc::c_int,
    mut fijkVerts: *mut FaceIJK,
) {
    let mut vertsCII: [CoordIJK; 6] = [
        {
            CoordIJK {
                i: 2 as libc::c_int,
                j: 1 as libc::c_int,
                k: 0 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 1 as libc::c_int,
                j: 2 as libc::c_int,
                k: 0 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 0 as libc::c_int,
                j: 2 as libc::c_int,
                k: 1 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 0 as libc::c_int,
                j: 1 as libc::c_int,
                k: 2 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 1 as libc::c_int,
                j: 0 as libc::c_int,
                k: 2 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 2 as libc::c_int,
                j: 0 as libc::c_int,
                k: 1 as libc::c_int,
            }
        },
    ];
    let mut vertsCIII: [CoordIJK; 6] = [
        {
            CoordIJK {
                i: 5 as libc::c_int,
                j: 4 as libc::c_int,
                k: 0 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 1 as libc::c_int,
                j: 5 as libc::c_int,
                k: 0 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 0 as libc::c_int,
                j: 5 as libc::c_int,
                k: 4 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 0 as libc::c_int,
                j: 1 as libc::c_int,
                k: 5 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 4 as libc::c_int,
                j: 0 as libc::c_int,
                k: 5 as libc::c_int,
            }
        },
        {
            CoordIJK {
                i: 5 as libc::c_int,
                j: 0 as libc::c_int,
                k: 1 as libc::c_int,
            }
        },
    ];
    let mut verts: *mut CoordIJK = std::ptr::null_mut::<CoordIJK>();
    if isResolutionClassIII(*res) != 0 {
        verts = vertsCIII.as_mut_ptr();
    } else {
        verts = vertsCII.as_mut_ptr();
    }
    _downAp3(&mut (*fijk).coord);
    _downAp3r(&mut (*fijk).coord);
    if isResolutionClassIII(*res) != 0 {
        _downAp7r(&mut (*fijk).coord);
        *res += 1 as libc::c_int;
    }
    let mut v: libc::c_int = 0 as libc::c_int;
    while v < 6 as libc::c_int {
        (*fijkVerts.offset(v as isize)).face = (*fijk).face;
        _ijkAdd(
            &(*fijk).coord,
            &*verts.offset(v as isize),
            &mut (*fijkVerts.offset(v as isize)).coord,
        );
        _ijkNormalize(&mut (*fijkVerts.offset(v as isize)).coord);
        v += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _adjustOverageClassII(
    mut fijk: *mut FaceIJK,
    mut res: libc::c_int,
    mut pentLeading4: libc::c_int,
    mut substrate: libc::c_int,
) -> Overage {
    let mut overage: Overage = NO_OVERAGE;
    let mut ijk: *mut CoordIJK = &mut (*fijk).coord;
    let mut maxDim: libc::c_int = maxDimByCIIres[res as usize];
    if substrate != 0 {
        maxDim *= 3 as libc::c_int;
    }
    if substrate != 0 && (*ijk).i + (*ijk).j + (*ijk).k == maxDim {
        overage = FACE_EDGE;
    } else if (*ijk).i + (*ijk).j + (*ijk).k > maxDim {
        overage = NEW_FACE;
        let mut fijkOrient: *const FaceOrientIJK = std::ptr::null::<FaceOrientIJK>();
        if (*ijk).k > 0 as libc::c_int {
            if (*ijk).j > 0 as libc::c_int {
                fijkOrient = &*(*faceNeighbors.as_ptr().offset((*fijk).face as isize))
                    .as_ptr()
                    .offset(3 as libc::c_int as isize)
                    as *const FaceOrientIJK;
            } else {
                fijkOrient = &*(*faceNeighbors.as_ptr().offset((*fijk).face as isize))
                    .as_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *const FaceOrientIJK;
                if pentLeading4 != 0 {
                    let mut origin: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
                    _setIJK(&mut origin, maxDim, 0 as libc::c_int, 0 as libc::c_int);
                    let mut tmp: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
                    _ijkSub(ijk, &origin, &mut tmp);
                    _ijkRotate60cw(&mut tmp);
                    _ijkAdd(&tmp, &origin, ijk);
                }
            }
        } else {
            fijkOrient = &*(*faceNeighbors.as_ptr().offset((*fijk).face as isize))
                .as_ptr()
                .offset(1 as libc::c_int as isize) as *const FaceOrientIJK;
        }
        (*fijk).face = (*fijkOrient).face;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*fijkOrient).ccwRot60 {
            _ijkRotate60ccw(ijk);
            i += 1;
        }
        let mut transVec: CoordIJK = (*fijkOrient).translate;
        let mut unitScale: libc::c_int = unitScaleByCIIres[res as usize];
        if substrate != 0 {
            unitScale *= 3 as libc::c_int;
        }
        _ijkScale(&mut transVec, unitScale);
        _ijkAdd(ijk, &transVec, ijk);
        _ijkNormalize(ijk);
        if substrate != 0 && (*ijk).i + (*ijk).j + (*ijk).k == maxDim {
            overage = FACE_EDGE;
        }
    }
    overage
}
#[no_mangle]
pub unsafe extern "C" fn _adjustPentVertOverage(
    mut fijk: *mut FaceIJK,
    mut res: libc::c_int,
) -> Overage {
    let mut pentLeading4: libc::c_int = 0 as libc::c_int;
    let mut overage: Overage = NO_OVERAGE;
    loop {
        overage = _adjustOverageClassII(fijk, res, pentLeading4, 1 as libc::c_int);
        if overage as libc::c_uint != NEW_FACE as libc::c_int as libc::c_uint {
            break;
        }
    }
    overage
}
#[no_mangle]
pub unsafe extern "C" fn _geoToClosestFace(
    mut g: *const LatLng,
    mut face: *mut libc::c_int,
    mut sqd: *mut libc::c_double,
) {
    let mut v3d: Vec3d = Vec3d {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    _geoToVec3d(g, &mut v3d);
    *face = 0 as libc::c_int;
    *sqd = 5.0f64;
    let mut f: libc::c_int = 0 as libc::c_int;
    while f < 20 as libc::c_int {
        let mut sqdT: libc::c_double =
            _pointSquareDist(&*faceCenterPoint.as_ptr().offset(f as isize), &v3d);
        if sqdT < *sqd {
            *face = f;
            *sqd = sqdT;
        }
        f += 1;
    }
}
