use std::{ffi::c_void, os::raw::c_char};
use std::{ffi::CString, ptr};

pub type napi_callback =
    Option<unsafe extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_env__ {
    _unused: [u8; 0],
}

/// Env ptr
pub type napi_env = *mut napi_env__;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_value__ {
    _unused: [u8; 0],
}
/// JsValue ptr
pub type napi_value = *mut napi_value__;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_callback_info__ {
    _unused: [u8; 0],
}
pub type napi_callback_info = *mut napi_callback_info__;

pub type napi_valuetype = i32;

pub mod ValueType {
    pub const napi_undefined: i32 = 0;
    pub const napi_null: i32 = 1;
    pub const napi_boolean: i32 = 2;
    pub const napi_number: i32 = 3;
    pub const napi_string: i32 = 4;
    pub const napi_symbol: i32 = 5;
    pub const napi_object: i32 = 6;
    pub const napi_function: i32 = 7;
    pub const napi_external: i32 = 8;
    pub const napi_bigint: i32 = 9;
}

pub type napi_status = i32;

pub mod Status {
    pub const napi_ok: i32 = 0;
    pub const napi_invalid_arg: i32 = 1;
    pub const napi_object_expected: i32 = 2;
    pub const napi_string_expected: i32 = 3;
    pub const napi_name_expected: i32 = 4;
    pub const napi_function_expected: i32 = 5;
    pub const napi_number_expected: i32 = 6;
    pub const napi_boolean_expected: i32 = 7;
    pub const napi_array_expected: i32 = 8;
    pub const napi_generic_failure: i32 = 9;
    pub const napi_pending_exception: i32 = 10;
    pub const napi_cancelled: i32 = 11;
    pub const napi_escape_called_twice: i32 = 12;
    pub const napi_handle_scope_mismatch: i32 = 13;
    pub const napi_callback_scope_mismatch: i32 = 14;
    pub const napi_queue_full: i32 = 15;
    pub const napi_closing: i32 = 16;
    pub const napi_bigint_expected: i32 = 17;
    pub const napi_date_expected: i32 = 18;
    pub const napi_arraybuffer_expected: i32 = 19;
    pub const napi_detachable_arraybuffer_expected: i32 = 20;
    pub const napi_would_deadlock: i32 = 21; // unused
}

extern "C" {
    pub fn napi_typeof(
        env: napi_env,
        value: napi_value,
        result: *mut napi_valuetype,
    ) -> napi_status;

    pub fn napi_get_cb_info(
        env: napi_env,
        cbinfo: napi_callback_info,
        argc: *mut usize,
        argv: *mut napi_value,
        this_arg: *mut napi_value,
        data: *mut *mut c_void,
    ) -> napi_status;

    pub fn napi_set_named_property(
        env: napi_env,
        object: napi_value,
        utf8name: *const c_char,
        value: napi_value,
    ) -> napi_status;

    pub fn napi_create_function(
        env: napi_env,
        utf8name: *const c_char,
        length: usize,
        cb: napi_callback,
        data: *mut c_void,
        result: *mut napi_value,
    ) -> napi_status;
}

#[doc(hidden)]
#[macro_export]
macro_rules! check_status {
    ($code:expr) => {{
        let c = $code;
        match c {
            Status::napi_ok => Ok(()),
            _ => Err(()),
        }
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! type_oooof {
    ($env:expr, $value:expr) => {{
        let mut value_type = 0;
        let status = napi_typeof($env, $value, &mut value_type);
        check_status!(status).map(|_| value_type)
    }};
}

unsafe extern "C" fn type_of(raw_env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let mut argc = 1;
    let mut raw_args: [napi_value; 1] = [ptr::null_mut(); 1];
    let status = napi_get_cb_info(
        raw_env,
        cb_info,
        &mut argc,
        raw_args.as_mut_ptr(),
        ptr::null_mut(),
        ptr::null_mut(),
    );
    assert!(status == Status::napi_ok, "napi_get_cb_info failed");

    let input_arg = raw_args[0];
    let value_type = type_oooof!(raw_env, input_arg).unwrap();

    println!(
        "Value type: {}, expected Value type: {}",
        value_type,
        ValueType::napi_string
    );
    assert!(value_type == ValueType::napi_string);

    ptr::null_mut()
}

#[no_mangle]
unsafe extern "C" fn napi_register_module_v1(
    raw_env: napi_env,
    raw_exports: napi_value,
) -> napi_value {
    let mut func = ptr::null_mut();
    let create_function_status = napi_create_function(
        raw_env,
        CString::new("TypeOf").unwrap().into_raw(),
        6,
        Some(type_of),
        ptr::null_mut(),
        &mut func,
    );
    assert!(
        create_function_status == Status::napi_ok,
        "Create function failed"
    );
    let set_named_property_status = napi_set_named_property(
        raw_env,
        raw_exports,
        CString::new("TypeOf").unwrap().into_raw(),
        func,
    );

    assert!(
        set_named_property_status == Status::napi_ok,
        "Set named property failed"
    );
    raw_exports
}
