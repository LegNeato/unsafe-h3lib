use ::libc;
extern "C" {
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn _zeroIndexDigits(h: H3Index, start: libc::c_int, end: libc::c_int) -> H3Index;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
}
pub type uint64_t = libc::c_ulonglong;
pub type H3Index = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IterCellsChildren {
    pub h: H3Index,
    pub _parentRes: libc::c_int,
    pub _skipDigit: libc::c_int,
}
pub type Direction = libc::c_uint;
pub const PENTAGON_SKIPPED_DIGIT: Direction = 1;
pub const NUM_DIGITS: Direction = 7;
pub const INVALID_DIGIT: Direction = 7;
pub const IJ_AXES_DIGIT: Direction = 6;
pub const IK_AXES_DIGIT: Direction = 5;
pub const I_AXES_DIGIT: Direction = 4;
pub const JK_AXES_DIGIT: Direction = 3;
pub const J_AXES_DIGIT: Direction = 2;
pub const K_AXES_DIGIT: Direction = 1;
pub const CENTER_DIGIT: Direction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IterCellsResolution {
    pub h: H3Index,
    pub _baseCellNum: libc::c_int,
    pub _res: libc::c_int,
    pub _itC: IterCellsChildren,
}
unsafe extern "C" fn _getResDigit(
    mut it: *mut IterCellsChildren,
    mut res: libc::c_int,
) -> libc::c_int {
    ((*it).h >> ((15 as libc::c_int - res) * 3 as libc::c_int) & 7 as libc::c_int as uint64_t)
        as Direction as libc::c_int
}
unsafe extern "C" fn _incrementResDigit(mut it: *mut IterCellsChildren, mut res: libc::c_int) {
    let mut val: H3Index = 1 as libc::c_int as H3Index;
    val <<= 3 as libc::c_int * (15 as libc::c_int - res);
    (*it).h = ((*it).h as libc::c_ulonglong).wrapping_add(val) as H3Index as H3Index;
}
unsafe extern "C" fn _null_iter() -> IterCellsChildren {
    {
        IterCellsChildren {
            h: 0 as libc::c_int as H3Index,
            _parentRes: -(1 as libc::c_int),
            _skipDigit: -(1 as libc::c_int),
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn iterInitParent(
    mut h: H3Index,
    mut childRes: libc::c_int,
) -> IterCellsChildren {
    let mut it: IterCellsChildren = IterCellsChildren {
        h: 0,
        _parentRes: 0,
        _skipDigit: 0,
    };
    it._parentRes =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if childRes < it._parentRes
        || childRes > 15 as libc::c_int
        || h == 0 as libc::c_int as libc::c_ulonglong
    {
        return _null_iter();
    }
    it.h = _zeroIndexDigits(h, it._parentRes + 1 as libc::c_int, childRes);
    it.h = it.h & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (childRes as uint64_t) << 52 as libc::c_int;
    if isPentagon(it.h) != 0 {
        it._skipDigit = childRes;
    } else {
        it._skipDigit = -(1 as libc::c_int);
    }
    it
}
#[no_mangle]
pub unsafe extern "C" fn iterStepChild(mut it: *mut IterCellsChildren) {
    if (*it).h == 0 as libc::c_int as libc::c_ulonglong {
        return;
    }
    let mut childRes: libc::c_int = (((*it).h & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int;
    _incrementResDigit(it, childRes);
    let mut i: libc::c_int = childRes;
    while i >= (*it)._parentRes {
        if i == (*it)._parentRes {
            *it = _null_iter();
            return;
        }
        if i == (*it)._skipDigit && _getResDigit(it, i) == PENTAGON_SKIPPED_DIGIT as libc::c_int {
            _incrementResDigit(it, i);
            (*it)._skipDigit -= 1 as libc::c_int;
            return;
        }
        if _getResDigit(it, i) != INVALID_DIGIT as libc::c_int {
            break;
        }
        _incrementResDigit(it, i);
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn iterInitBaseCellNum(
    mut baseCellNum: libc::c_int,
    mut childRes: libc::c_int,
) -> IterCellsChildren {
    if baseCellNum < 0 as libc::c_int
        || baseCellNum >= 122 as libc::c_int
        || childRes < 0 as libc::c_int
        || childRes > 15 as libc::c_int
    {
        return _null_iter();
    }
    let mut baseCell: H3Index = 0;
    setH3Index(&mut baseCell, 0 as libc::c_int, baseCellNum, CENTER_DIGIT);
    iterInitParent(baseCell, childRes)
}
#[no_mangle]
pub unsafe extern "C" fn iterInitRes(mut res: libc::c_int) -> IterCellsResolution {
    let mut itC: IterCellsChildren = iterInitBaseCellNum(0 as libc::c_int, res);
    let mut itR: IterCellsResolution = {
        IterCellsResolution {
            h: itC.h,
            _baseCellNum: 0 as libc::c_int,
            _res: res,
            _itC: itC,
        }
    };
    itR
}
#[no_mangle]
pub unsafe extern "C" fn iterStepRes(mut itR: *mut IterCellsResolution) {
    if (*itR).h == 0 as libc::c_int as libc::c_ulonglong {
        return;
    }
    iterStepChild(&mut (*itR)._itC);
    if (*itR)._itC.h == 0 as libc::c_int as libc::c_ulonglong
        && ((*itR)._baseCellNum + 1 as libc::c_int) < 122 as libc::c_int
    {
        (*itR)._baseCellNum += 1 as libc::c_int;
        (*itR)._itC = iterInitBaseCellNum((*itR)._baseCellNum, (*itR)._res);
    }
    (*itR).h = (*itR)._itC.h;
}
