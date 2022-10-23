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
static mut faceCenterPoint: [Vec3d; 20] = [
    {
        let mut init = Vec3d {
            x: 0.2199307791404606f64,
            y: 0.6583691780274996f64,
            z: 0.7198475378926182f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.2139234834501421f64,
            y: 0.1478171829550703f64,
            z: 0.9656017935214205f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: 0.1092625278784797f64,
            y: -0.4811951572873210f64,
            z: 0.8697775121287253f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: 0.7428567301586791f64,
            y: -0.3593941678278028f64,
            z: 0.5648005936517033f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: 0.8112534709140969f64,
            y: 0.3448953237639384f64,
            z: 0.4721387736413930f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.1055498149613921f64,
            y: 0.9794457296411413f64,
            z: 0.1718874610009365f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.8075407579970092f64,
            y: 0.1533552485898818f64,
            z: 0.5695261994882688f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.2846148069787907f64,
            y: -0.8644080972654206f64,
            z: 0.4144792552473539f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: 0.7405621473854482f64,
            y: -0.6673299564565524f64,
            z: -0.0789837646326737f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: 0.8512303986474293f64,
            y: 0.4722343788582681f64,
            z: -0.2289137388687808f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.7405621473854481f64,
            y: 0.6673299564565524f64,
            z: 0.0789837646326737f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.8512303986474292f64,
            y: -0.4722343788582682f64,
            z: 0.2289137388687808f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: 0.1055498149613919f64,
            y: -0.9794457296411413f64,
            z: -0.1718874610009365f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: 0.8075407579970092f64,
            y: -0.1533552485898819f64,
            z: -0.5695261994882688f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: 0.2846148069787908f64,
            y: 0.8644080972654204f64,
            z: -0.4144792552473539f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.7428567301586791f64,
            y: 0.3593941678278027f64,
            z: -0.5648005936517033f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.8112534709140971f64,
            y: -0.3448953237639382f64,
            z: -0.4721387736413930f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.2199307791404607f64,
            y: -0.6583691780274996f64,
            z: -0.7198475378926182f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: 0.2139234834501420f64,
            y: -0.1478171829550704f64,
            z: -0.9656017935214205f64,
        };
        init
    },
    {
        let mut init = Vec3d {
            x: -0.1092625278784796f64,
            y: 0.4811951572873210f64,
            z: -0.8697775121287253f64,
        };
        init
    },
];
static mut faceAxesAzRadsCII: [[libc::c_double; 3]; 20] = [
    [
        5.619958268523939882f64,
        3.525563166130744542f64,
        1.431168063737548730f64,
    ],
    [
        5.760339081714187279f64,
        3.665943979320991689f64,
        1.571548876927796127f64,
    ],
    [
        0.780213654393430055f64,
        4.969003859179821079f64,
        2.874608756786625655f64,
    ],
    [
        0.430469363979999913f64,
        4.619259568766391033f64,
        2.524864466373195467f64,
    ],
    [
        6.130269123335111400f64,
        4.035874020941915804f64,
        1.941478918548720291f64,
    ],
    [
        2.692877706530642877f64,
        0.598482604137447119f64,
        4.787272808923838195f64,
    ],
    [
        2.982963003477243874f64,
        0.888567901084048369f64,
        5.077358105870439581f64,
    ],
    [
        3.532912002790141181f64,
        1.438516900396945656f64,
        5.627307105183336758f64,
    ],
    [
        3.494305004259568154f64,
        1.399909901866372864f64,
        5.588700106652763840f64,
    ],
    [
        3.003214169499538391f64,
        0.908819067106342928f64,
        5.097609271892733906f64,
    ],
    [
        5.930472956509811562f64,
        3.836077854116615875f64,
        1.741682751723420374f64,
    ],
    [
        0.138378484090254847f64,
        4.327168688876645809f64,
        2.232773586483450311f64,
    ],
    [
        0.448714947059150361f64,
        4.637505151845541521f64,
        2.543110049452346120f64,
    ],
    [
        0.158629650112549365f64,
        4.347419854898940135f64,
        2.253024752505744869f64,
    ],
    [
        5.891865957979238535f64,
        3.797470855586042958f64,
        1.703075753192847583f64,
    ],
    [
        2.711123289609793325f64,
        0.616728187216597771f64,
        4.805518392002988683f64,
    ],
    [
        3.294508837434268316f64,
        1.200113735041072948f64,
        5.388903939827463911f64,
    ],
    [
        3.804819692245439833f64,
        1.710424589852244509f64,
        5.899214794638635174f64,
    ],
    [
        3.664438879055192436f64,
        1.570043776661997111f64,
        5.758833981448388027f64,
    ],
    [
        2.361378999196363184f64,
        0.266983896803167583f64,
        4.455774101589558636f64,
    ],
];
static mut faceNeighbors: [[FaceOrientIJK; 4]; 20] = [
    [
        {
            let mut init = FaceOrientIJK {
                face: 0 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 4 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 1 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 5 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 1 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 0 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 2 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 6 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 2 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 1 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 3 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 7 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 3 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 2 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 4 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 8 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 4 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 3 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 0 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 9 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 5 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 10 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 14 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 0 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 6 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 11 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 10 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 1 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 7 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 12 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 11 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 2 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 8 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 13 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 12 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 3 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 9 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 14 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 13 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 4 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 10 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 5 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 6 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 15 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 11 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 6 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 7 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 16 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 12 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 7 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 8 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 17 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 13 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 8 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 9 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 18 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 14 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 9 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 5 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 19 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 15 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 16 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 19 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 10 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 16 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 17 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 15 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 11 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 17 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 18 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 16 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 12 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 18 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 19 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 17 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 13 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
        },
    ],
    [
        {
            let mut init = FaceOrientIJK {
                face: 19 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 15 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 0 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 18 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 2 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 0 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = FaceOrientIJK {
                face: 14 as libc::c_int,
                translate: {
                    let mut init = CoordIJK {
                        i: 0 as libc::c_int,
                        j: 2 as libc::c_int,
                        k: 2 as libc::c_int,
                    };
                    init
                },
                ccwRot60: 3 as libc::c_int,
            };
            init
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
    _hex2dToCoordIJK(&mut v, &mut (*h).coord);
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
        theta = _posAngleRads(
            (theta - 0.333473172251832115336090755351601070065900389)
                .to_f64()
                .unwrap(),
        );
    }
    r = tan(r);
    r = (r / 0.38196601125010500003).to_f64().unwrap();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < res {
        r = (r * 2.6457513110645905905016157536392604257102)
            .to_f64()
            .unwrap();
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
        r = (r / 2.6457513110645905905016157536392604257102)
            .to_f64()
            .unwrap();
        i += 1;
    }
    if substrate != 0 {
        r /= 3.0f64;
        if isResolutionClassIII(res) != 0 {
            r = (r / 2.6457513110645905905016157536392604257102)
                .to_f64()
                .unwrap();
        }
    }
    r = (r * 0.38196601125010500003).to_f64().unwrap();
    r = atan(r);
    if substrate == 0 && isResolutionClassIII(res) != 0 {
        theta = _posAngleRads(
            (theta + 0.333473172251832115336090755351601070065900389)
                .to_f64()
                .unwrap(),
        );
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
    _hex2dToGeo(&mut v, (*h).face, res, 0 as libc::c_int, g);
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
            _ijkToHex2d(&mut lastFijk.coord, &mut orig2d0);
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
            _ijkAdd(ijk, &mut transVec, ijk);
            _ijkNormalize(ijk);
            let mut orig2d1: Vec2d = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(ijk, &mut orig2d1);
            let mut maxDim: libc::c_int = maxDimByCIIres[adjRes as usize];
            let mut v0: Vec2d = {
                let mut init = Vec2d {
                    x: 3.0f64 * maxDim as libc::c_double,
                    y: 0.0f64,
                };
                init
            };
            let mut v1: Vec2d = {
                let mut init = Vec2d {
                    x: -1.5f64 * maxDim as libc::c_double,
                    y: (3.0f64 * 0.8660254037844386467637231707529361834714 * maxDim as f64)
                        .to_f64()
                        .unwrap(),
                };
                init
            };
            let mut v2: Vec2d = {
                let mut init = Vec2d {
                    x: -1.5f64 * maxDim as libc::c_double,
                    y: (-3.0f64 * 0.8660254037844386467637231707529361834714 * maxDim as f64)
                        .to_f64()
                        .unwrap(),
                };
                init
            };
            let mut edge0: *mut Vec2d = 0 as *mut Vec2d;
            let mut edge1: *mut Vec2d = 0 as *mut Vec2d;
            match adjacentFaceDir[tmpFijk.face as usize][fijk.face as usize] {
                1 => {
                    edge0 = &mut v0;
                    edge1 = &mut v1;
                }
                3 => {
                    edge0 = &mut v1;
                    edge1 = &mut v2;
                }
                2 | _ => {
                    if !(adjacentFaceDir[tmpFijk.face as usize][fijk.face as usize]
                        == 2 as libc::c_int) as libc::c_int as libc::c_long
                        != 0
                    {
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
            _v2dIntersect(&mut orig2d0, &mut orig2d1, edge0, edge1, &mut inter);
            _hex2dToGeo(
                &mut inter,
                tmpFijk.face,
                adjRes,
                1 as libc::c_int,
                &mut *((*g).verts).as_mut_ptr().offset((*g).numVerts as isize),
            );
            (*g).numVerts += 1;
        }
        if vert < start + 5 as libc::c_int {
            let mut vec: Vec2d = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(&mut fijk.coord, &mut vec);
            _hex2dToGeo(
                &mut vec,
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
            let mut init = CoordIJK {
                i: 2 as libc::c_int,
                j: 1 as libc::c_int,
                k: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 1 as libc::c_int,
                j: 2 as libc::c_int,
                k: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 2 as libc::c_int,
                k: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 1 as libc::c_int,
                k: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 1 as libc::c_int,
                j: 0 as libc::c_int,
                k: 2 as libc::c_int,
            };
            init
        },
    ];
    let mut vertsCIII: [CoordIJK; 5] = [
        {
            let mut init = CoordIJK {
                i: 5 as libc::c_int,
                j: 4 as libc::c_int,
                k: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 1 as libc::c_int,
                j: 5 as libc::c_int,
                k: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 5 as libc::c_int,
                k: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 1 as libc::c_int,
                k: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 4 as libc::c_int,
                j: 0 as libc::c_int,
                k: 5 as libc::c_int,
            };
            init
        },
    ];
    let mut verts: *mut CoordIJK = 0 as *mut CoordIJK;
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
            &mut (*fijk).coord,
            &mut *verts.offset(v as isize),
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
                &mut (*fijkVerts.as_mut_ptr().offset(lastV as isize)).coord,
                &mut orig2d0,
            );
            let mut orig2d1: Vec2d = Vec2d { x: 0., y: 0. };
            _ijkToHex2d(
                &mut (*fijkVerts.as_mut_ptr().offset(v as isize)).coord,
                &mut orig2d1,
            );
            let mut maxDim: libc::c_int = maxDimByCIIres[adjRes as usize];
            let mut v0: Vec2d = {
                let mut init = Vec2d {
                    x: 3.0f64 * maxDim as libc::c_double,
                    y: 0.0f64,
                };
                init
            };
            let mut v1: Vec2d = {
                let mut init = Vec2d {
                    x: -1.5f64 * maxDim as libc::c_double,
                    y: (3.0f64 * 0.8660254037844386467637231707529361834714 * maxDim as f64)
                        .to_f64()
                        .unwrap(),
                };
                init
            };
            let mut v2: Vec2d = {
                let mut init = Vec2d {
                    x: -1.5f64 * maxDim as libc::c_double,
                    y: (-3.0f64 * 0.8660254037844386467637231707529361834714 * maxDim as f64)
                        .to_f64()
                        .unwrap(),
                };
                init
            };
            let mut face2: libc::c_int = if lastFace == centerIJK.face {
                fijk.face
            } else {
                lastFace
            };
            let mut edge0: *mut Vec2d = 0 as *mut Vec2d;
            let mut edge1: *mut Vec2d = 0 as *mut Vec2d;
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
                    if !(adjacentFaceDir[centerIJK.face as usize][face2 as usize]
                        == 2 as libc::c_int) as libc::c_int as libc::c_long
                        != 0
                    {
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
            _v2dIntersect(&mut orig2d0, &mut orig2d1, edge0, edge1, &mut inter);
            let mut isIntersectionAtVertex: bool =
                _v2dAlmostEquals(&mut orig2d0, &mut inter) as libc::c_int != 0
                    || _v2dAlmostEquals(&mut orig2d1, &mut inter) as libc::c_int != 0;
            if !isIntersectionAtVertex {
                _hex2dToGeo(
                    &mut inter,
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
            _ijkToHex2d(&mut fijk.coord, &mut vec);
            _hex2dToGeo(
                &mut vec,
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
            let mut init = CoordIJK {
                i: 2 as libc::c_int,
                j: 1 as libc::c_int,
                k: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 1 as libc::c_int,
                j: 2 as libc::c_int,
                k: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 2 as libc::c_int,
                k: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 1 as libc::c_int,
                k: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 1 as libc::c_int,
                j: 0 as libc::c_int,
                k: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 2 as libc::c_int,
                j: 0 as libc::c_int,
                k: 1 as libc::c_int,
            };
            init
        },
    ];
    let mut vertsCIII: [CoordIJK; 6] = [
        {
            let mut init = CoordIJK {
                i: 5 as libc::c_int,
                j: 4 as libc::c_int,
                k: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 1 as libc::c_int,
                j: 5 as libc::c_int,
                k: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 5 as libc::c_int,
                k: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 1 as libc::c_int,
                k: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 4 as libc::c_int,
                j: 0 as libc::c_int,
                k: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = CoordIJK {
                i: 5 as libc::c_int,
                j: 0 as libc::c_int,
                k: 1 as libc::c_int,
            };
            init
        },
    ];
    let mut verts: *mut CoordIJK = 0 as *mut CoordIJK;
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
            &mut (*fijk).coord,
            &mut *verts.offset(v as isize),
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
        let mut fijkOrient: *const FaceOrientIJK = 0 as *const FaceOrientIJK;
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
                    _ijkSub(ijk, &mut origin, &mut tmp);
                    _ijkRotate60cw(&mut tmp);
                    _ijkAdd(&mut tmp, &mut origin, ijk);
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
        _ijkAdd(ijk, &mut transVec, ijk);
        _ijkNormalize(ijk);
        if substrate != 0 && (*ijk).i + (*ijk).j + (*ijk).k == maxDim {
            overage = FACE_EDGE;
        }
    }
    return overage;
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
        if !(overage as libc::c_uint == NEW_FACE as libc::c_int as libc::c_uint) {
            break;
        }
    }
    return overage;
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
            _pointSquareDist(&*faceCenterPoint.as_ptr().offset(f as isize), &mut v3d);
        if sqdT < *sqd {
            *face = f;
            *sqd = sqdT;
        }
        f += 1;
    }
}
