fn main() {
  let window_count = unsafe {
    screen_capture_lite::SCL_GetWindows(std::ptr::null_mut(), 0)
  };

  println!("window count: {window_count}");
}
