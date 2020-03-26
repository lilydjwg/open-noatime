use std::os::raw::c_char;
use std::io::{Error, ErrorKind};
use libc::O_NOATIME;

redhook::hook! {
  unsafe fn open64(
    path: *const c_char,
    flags: i32,
    mode: i32
  ) -> i32 => fileopen64 {
    let ret = redhook::real!(open64)(
      path, flags | O_NOATIME, mode
    );

    if ret < 0 && Error::last_os_error().kind() == ErrorKind::PermissionDenied {
      redhook::real!(open64)(path, flags, mode)
    } else {
      ret
    }
  }
}


redhook::hook! {
  unsafe fn open(
    path: *const c_char,
    flags: i32,
    mode: i32
  ) -> i32 => fileopen {
    let ret = redhook::real!(open)(
      path, flags | O_NOATIME, mode
    );

    if ret < 0 && Error::last_os_error().kind() == ErrorKind::PermissionDenied {
      redhook::real!(open)(path, flags, mode)
    } else {
      ret
    }
  }
}

redhook::hook! {
  unsafe fn __open64_2(
    path: *const c_char,
    flags: i32
  ) -> i32 => fileopen64_2 {
    let ret = redhook::real!(__open64_2)(
      path, flags | O_NOATIME
    );

    if ret < 0 && Error::last_os_error().kind() == ErrorKind::PermissionDenied {
      redhook::real!(__open64_2)(path, flags)
    } else {
      ret
    }
  }
}

