#![feature(once_cell)]

use allo_isolate::Isolate;
use ffi_helpers::null_pointer_check;
use std::{ffi::CStr, lazy::SyncLazy, os::raw};
use tokio::runtime::{Builder, Runtime};

// TODO: change to a static SMOL_RUNTIME/EXECUTOR ?
static RUNTIME: SyncLazy<Runtime> = SyncLazy::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_name("bernard")
        .build()
        .unwrap()
});

#[repr(C)]
pub struct TestStruct {
    _i: i8,
    _u: u8,
}

macro_rules! error {
    ($result:expr) => {
        error!($result, 0);
    };
    ($result:expr, $error:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                ffi_helpers::update_last_error(e);
                return $error;
            }
        }
    };
}

macro_rules! cstr {
    ($ptr:expr) => {
        cstr!($ptr, 0);
    };
    ($ptr:expr, $error:expr) => {{
        null_pointer_check!($ptr);
        error!(unsafe { CStr::from_ptr($ptr).to_str() }, $error)
    }};
}

#[no_mangle]
pub unsafe extern "C" fn last_error_length() -> i32 {
    ffi_helpers::error_handling::last_error_length()
}

#[no_mangle]
pub unsafe extern "C" fn error_message_utf8(buf: *mut raw::c_char, length: i32) -> i32 {
    ffi_helpers::error_handling::error_message_utf8(buf, length)
}

#[no_mangle]
pub extern "C" fn load_page(port: i64, url: *const raw::c_char) -> i32 {
    let url = cstr!(url);
    RUNTIME.spawn(async move {
        let result = scrap::load_page(url).await;
        let isolate = Isolate::new(port);
        isolate.post(result);
    });
    1
}

// #[no_mangle]
// pub extern "C" fn test(port: i64, url: *const TestStruct) -> i32 {
//     1
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
