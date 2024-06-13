#![allow(
    non_camel_case_types,
    non_snake_case,
    unused_variables,
    non_upper_case_globals
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
