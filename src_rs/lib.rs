use std::os::raw::{c_int, c_void};

extern "C" {
  pub fn SCL_GetWindows(windows: *mut c_void, windows_size: c_int) -> c_int;
}
