#![allow(
    non_camel_case_types,
    non_snake_case,
    unused_variables,
    non_upper_case_globals,
    dead_code
)]

use std::ffi::{c_char, c_int, c_uint};
use std::os::raw::c_void;

pub type TwSetVarCallback =
    Option<unsafe extern "C" fn(value: *const c_void, clientData: *mut c_void)>;
pub type TwGetVarCallback =
    Option<unsafe extern "C" fn(value: *mut c_void, clientData: *mut c_void)>;
pub type TwButtonCallback = Option<unsafe extern "C" fn(clientData: *mut c_void)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CTwBar {
    _unused: [u8; 0],
}

pub type TwBar = CTwBar;

pub type TwType = c_uint;

pub type TwCopyCDStringToClient = Option<
    unsafe extern "C" fn(destinationClientStringPtr: *mut *mut c_char, sourceString: *const c_char),
>;

pub type TwCopyStdStringToClient = ::std::option::Option<
    unsafe extern "C" fn(destinationClientString: *mut c_void, sourceString: *const c_void),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CTwEnumVal {
    pub Value: c_int,
    pub Label: *const c_char,
}
pub type TwEnumVal = CTwEnumVal;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CTwStructMember {
    pub Name: *const c_char,
    pub Type: TwType,
    pub Offset: usize,
    pub DefString: *const c_char,
}
pub type TwStructMember = CTwStructMember;

pub type TwSummaryCallback = Option<
    unsafe extern "C" fn(
        summaryString: *mut c_char,
        summaryMaxLength: usize,
        value: *const c_void,
        clientData: *mut c_void,
    ),
>;

pub type TwParamValueType = c_uint;

pub type TwGraphAPI = c_uint;
pub type TwMouseAction = c_uint;
pub type TwMouseButtonID = c_uint;
pub type TwErrorHandler = Option<unsafe extern "C" fn(errorMessage: *const c_char)>;

pub const ETwKeyModifier_TW_KMOD_NONE: TwKeyModifier = 0;
pub const ETwKeyModifier_TW_KMOD_SHIFT: TwKeyModifier = 3;
pub const ETwKeyModifier_TW_KMOD_CTRL: TwKeyModifier = 192;
pub const ETwKeyModifier_TW_KMOD_ALT: TwKeyModifier = 256;
pub const ETwKeyModifier_TW_KMOD_META: TwKeyModifier = 3072;
pub type TwKeyModifier = c_uint;

pub const KeySpecial_TW_KEY_BACKSPACE: KeySpecial = 8;
pub const KeySpecial_TW_KEY_TAB: KeySpecial = 9;
pub const KeySpecial_TW_KEY_CLEAR: KeySpecial = 12;
pub const KeySpecial_TW_KEY_RETURN: KeySpecial = 13;
pub const KeySpecial_TW_KEY_PAUSE: KeySpecial = 19;
pub const KeySpecial_TW_KEY_ESCAPE: KeySpecial = 27;
pub const KeySpecial_TW_KEY_SPACE: KeySpecial = 32;
pub const KeySpecial_TW_KEY_DELETE: KeySpecial = 127;
pub const KeySpecial_TW_KEY_UP: KeySpecial = 273;
pub const KeySpecial_TW_KEY_DOWN: KeySpecial = 274;
pub const KeySpecial_TW_KEY_RIGHT: KeySpecial = 275;
pub const KeySpecial_TW_KEY_LEFT: KeySpecial = 276;
pub const KeySpecial_TW_KEY_INSERT: KeySpecial = 277;
pub const KeySpecial_TW_KEY_HOME: KeySpecial = 278;
pub const KeySpecial_TW_KEY_END: KeySpecial = 279;
pub const KeySpecial_TW_KEY_PAGE_UP: KeySpecial = 280;
pub const KeySpecial_TW_KEY_PAGE_DOWN: KeySpecial = 281;
pub const KeySpecial_TW_KEY_F1: KeySpecial = 282;
pub const KeySpecial_TW_KEY_F2: KeySpecial = 283;
pub const KeySpecial_TW_KEY_F3: KeySpecial = 284;
pub const KeySpecial_TW_KEY_F4: KeySpecial = 285;
pub const KeySpecial_TW_KEY_F5: KeySpecial = 286;
pub const KeySpecial_TW_KEY_F6: KeySpecial = 287;
pub const KeySpecial_TW_KEY_F7: KeySpecial = 288;
pub const KeySpecial_TW_KEY_F8: KeySpecial = 289;
pub const KeySpecial_TW_KEY_F9: KeySpecial = 290;
pub const KeySpecial_TW_KEY_F10: KeySpecial = 291;
pub const KeySpecial_TW_KEY_F11: KeySpecial = 292;
pub const KeySpecial_TW_KEY_F12: KeySpecial = 293;
pub const KeySpecial_TW_KEY_F13: KeySpecial = 294;
pub const KeySpecial_TW_KEY_F14: KeySpecial = 295;
pub const KeySpecial_TW_KEY_F15: KeySpecial = 296;
pub const KeySpecial_TW_KEY_LAST: KeySpecial = 297;
pub type KeySpecial = c_uint;
