extern crate cmake;

use cmake::Config;

fn main() {
  let dest = Config::new(".").profile("Release").build();
  println!("cargo:rustc-link-search={}", dest.join("lib").display());
  println!("cargo:rustc-link-lib=static=screen_capture_lite");
}
