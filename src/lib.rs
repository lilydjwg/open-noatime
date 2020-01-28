use std::os::raw::c_char;
use libc::O_NOATIME;

redhook::hook! {
  unsafe fn open64(
    path: *const c_char,
    flags: i32,
    mode: i32
  ) -> i32 => fileopen64 {
    redhook::real!(open64)(
      path, flags | O_NOATIME, mode
    )
  }
}


redhook::hook! {
  unsafe fn open(
    path: *const c_char,
    flags: i32,
    mode: i32
  ) -> i32 => fileopen {
    redhook::real!(open)(
      path, flags | O_NOATIME, mode
    )
  }
}

