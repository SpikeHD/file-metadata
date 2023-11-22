use std::fs;

pub fn get_file_metadata(path: &str) -> Result<fs::Metadata, std::io::Error> {
  fs::metadata(path)
}

pub fn log_and_return(e: impl std::fmt::Display) -> f64 {
  eprintln!("{}", e);
  -1.0
}

#[no_mangle]
pub extern fn file_creation_date(path: *const i8) -> f64 {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = match std::str::from_utf8(path) {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  let metadata = match get_file_metadata(path) {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  let creation_time = match metadata.created() {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  creation_time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as f64
}

#[no_mangle]
pub extern fn file_modification_date(path: *const i8) -> f64 {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = match std::str::from_utf8(path) {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  let metadata = match get_file_metadata(path) {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  let modification_time = match metadata.modified() {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  modification_time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as f64
}

#[no_mangle]
pub extern fn file_access_date(path: *const i8) -> f64 {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = match std::str::from_utf8(path) {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  let metadata = match get_file_metadata(path) {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  let access_time = match metadata.accessed() {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  access_time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as f64
}

#[no_mangle]
pub extern fn file_size(path: *const i8) -> f64 {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = match std::str::from_utf8(path) {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  let metadata = match get_file_metadata(path) {
    Ok(v) => v,
    Err(e) => return log_and_return(e),
  };

  metadata.len() as f64
}

#[no_mangle]
pub extern fn file_exists(path: *const i8) -> bool {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = match std::str::from_utf8(path) {
    Ok(v) => v,
    Err(_) => return false,
  };

  let metadata = get_file_metadata(path);
  
  metadata.is_ok()
}

#[no_mangle]
pub extern fn file_is_directory(path: *const i8) -> bool {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = match std::str::from_utf8(path) {
    Ok(v) => v,
    Err(_) => return false,
  };

  let metadata = match get_file_metadata(path) {
    Ok(v) => v,
    Err(_) => return false,
  };

  metadata.is_dir()
}