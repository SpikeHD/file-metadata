use std::fs;

pub fn get_file_metadata(path: &str) -> Result<fs::Metadata, std::io::Error> {
  fs::metadata(path)
}

#[no_mangle]
pub extern fn file_creation_date(path: *const i8) -> u64 {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = std::str::from_utf8(path).expect("Invalid UTF-8 string for path");

  let metadata = get_file_metadata(path).expect("Failed to get file metadata");
  let creation_time = metadata.created().expect("Failed to get file creation time");

  creation_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

#[no_mangle]
pub extern fn file_modification_date(path: *const i8) -> u64 {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = std::str::from_utf8(path).expect("Invalid UTF-8 string for path");

  let metadata = get_file_metadata(path).expect("Failed to get file metadata");
  let modification_time = metadata.modified().expect("Failed to get file modification time");

  modification_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

#[no_mangle]
pub extern fn file_access_date(path: *const i8) -> u64 {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = std::str::from_utf8(path).expect("Invalid UTF-8 string for path");

  let metadata = get_file_metadata(path).expect("Failed to get file metadata");
  let access_time = metadata.accessed().expect("Failed to get file access time");

  access_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

#[no_mangle]
pub extern fn file_size(path: *const i8) -> u64 {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = std::str::from_utf8(path).expect("Invalid UTF-8 string for path");

  let metadata = get_file_metadata(path).expect("Failed to get file metadata");
  metadata.len()
}

#[no_mangle]
pub extern fn file_exists(path: *const i8) -> bool {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = std::str::from_utf8(path).expect("Invalid UTF-8 string for path");

  let metadata = get_file_metadata(path);
  match metadata {
    Ok(_) => true,
    Err(_) => false
  }
}

#[no_mangle]
pub extern fn file_is_directory(path: *const i8) -> bool {
  let path = unsafe { std::ffi::CStr::from_ptr(path).to_bytes() };
  let path = std::str::from_utf8(path).expect("Invalid UTF-8 string for path");

  let metadata = get_file_metadata(path).expect("Failed to get file metadata");
  metadata.is_dir()
}