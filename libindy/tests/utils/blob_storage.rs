extern crate libc;

use std::ffi::CString;

use indy::api::blob_storage::*;
use indy::api::ErrorCode;

use utils::callback::CallbackUtils;

pub struct BlobStorageUtils {}

impl BlobStorageUtils {
    pub fn open_reader(type_: &str, config_json: &str, location: &str, hash: &str) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_i32();

        let type_ = CString::new(type_).unwrap();
        let config_json = CString::new(config_json).unwrap();
        let location = CString::new(location).unwrap();
        let hash = CString::new(hash).unwrap();

        let err = indy_blob_storage_open_reader(command_handle,
                                                type_.as_ptr(),
                                                config_json.as_ptr(),
                                                location.as_ptr(),
                                                hash.as_ptr(),
                                                cb);

        super::results::result_to_int(err, receiver)
    }

    pub fn close_reader(reader_handle: i32) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec();

        let err = indy_blob_storage_close_reader(command_handle, reader_handle, cb);

        super::results::result_to_empty(err, receiver)
    }
}