extern crate ejdb_sys;

use ejdbquery::EJDBQuery;
use ejdbquery::EJDBSerializable;

pub struct EJDB {
    db: ejdb_sys::EJDB
}

impl EJDBSerializable<String> for &str {
    fn from_jbl(jbl: ejdb_sys::JBL) -> Result<String, ejdb_sys::iwrc>
    {
        let xstr = unsafe { ejdb_sys::iwxstr_new() };

        let rc5 = unsafe {
            ejdb_sys::jbl_as_json(
                jbl,
                Some(ejdb_sys::jbl_xstr_json_printer),
                xstr as *mut ::std::os::raw::c_void,
                1,
            )
        };

        if rc5 != 0 {
            println!("failed to convert json to str {}", rc5);
            unsafe {
                ejdb_sys::iwxstr_destroy(xstr);
            };
            return Err(rc5);
        }

        let str2 = unsafe {
            std::ffi::CStr::from_ptr(ejdb_sys::iwxstr_ptr(xstr))
                .to_str()
                .unwrap()
        };

        let result = String::from(str2);
        unsafe {
            ejdb_sys::iwxstr_destroy(xstr);
        };

        return Ok(result);
    }

    fn to_jbl(&self) -> Result<ejdb_sys::JBL, ejdb_sys::iwrc > 
    {
        let mut jbl: ejdb_sys::JBL = std::ptr::null_mut();
        let json = std::ffi::CString::new(*self).unwrap();
        let rc3 = unsafe { ejdb_sys::jbl_from_json(&mut jbl, json.as_ptr()) };

        if rc3 != 0 {
            println!("json error: {}", EJDB::err_to_str(rc3));
            unsafe { ejdb_sys::jbl_destroy(&mut jbl) };
            return Err(rc3);
        }
        return Ok(jbl);
    }
}

impl EJDBSerializable<String> for String {
    fn from_jbl(jbl: ejdb_sys::JBL) -> Result<String, ejdb_sys::iwrc>
    {
        let xstr = unsafe { ejdb_sys::iwxstr_new() };

        let rc5 = unsafe {
            ejdb_sys::jbl_as_json(
                jbl,
                Some(ejdb_sys::jbl_xstr_json_printer),
                xstr as *mut ::std::os::raw::c_void,
                1,
            )
        };

        if rc5 != 0 {
            println!("failed to convert json to str {}", rc5);
            unsafe {
                ejdb_sys::iwxstr_destroy(xstr);
            };
            return Err(rc5);
        }

        let str2 = unsafe {
            std::ffi::CStr::from_ptr(ejdb_sys::iwxstr_ptr(xstr))
                .to_str()
                .unwrap()
        };

        let result = String::from(str2);
        unsafe {
            ejdb_sys::iwxstr_destroy(xstr);
        };

        return Ok(result);
    }

    fn to_jbl(&self) -> Result<ejdb_sys::JBL, ejdb_sys::iwrc > 
    {
        let mut jbl: ejdb_sys::JBL = std::ptr::null_mut();
        let json = std::ffi::CString::new(self.as_str()).unwrap();
        let rc3 = unsafe { ejdb_sys::jbl_from_json(&mut jbl, json.as_ptr()) };

        if rc3 != 0 {
            println!("json error: {}", EJDB::err_to_str(rc3));
            unsafe { ejdb_sys::jbl_destroy(&mut jbl) };
            return Err(rc3);
        }
        return Ok(jbl);
    }
}

impl Drop for EJDB {
    fn drop(&mut self) {
        unsafe {
            ejdb_sys::ejdb_close(&mut self.db);
        }
        self.db = std::ptr::null_mut();
        println!("db closed");
    }
}

impl EJDB {
    fn err_to_str(mut rc: ejdb_sys::iwrc) -> &'static str {
        let error_code = unsafe { ejdb_sys::iwrc_strip_errno(&mut rc) };

        match error_code {
            ejdb_sys::iw_ecode_IW_OK => "No error.",
            ejdb_sys::iw_ecode_IW_ERROR_FAIL => "Unspecified error.",
            ejdb_sys::iw_ecode_IW_ERROR_ERRNO => "Error with expected errno status set.",
            ejdb_sys::iw_ecode_IW_ERROR_IO_ERRNO => "IO error with expected errno status set.",
            ejdb_sys::iw_ecode_IW_ERROR_AGAIN => "Again",
            ejdb_sys::iw_ecode_IW_ERROR_NOT_EXISTS => "Resource is not exists.",
            ejdb_sys::iw_ecode_IW_ERROR_READONLY => "Resource is readonly.",
            ejdb_sys::iw_ecode_IW_ERROR_ALREADY_OPENED => "Resource is already opened.",
            ejdb_sys::iw_ecode_IW_ERROR_THREADING => "Threading error.",
            ejdb_sys::iw_ecode_IW_ERROR_THREADING_ERRNO => "Threading error with errno status set.",
            ejdb_sys::iw_ecode_IW_ERROR_ASSERTION => "Generic assertion error.",
            ejdb_sys::iw_ecode_IW_ERROR_INVALID_HANDLE => "Invalid HANDLE value.",
            ejdb_sys::iw_ecode_IW_ERROR_OUT_OF_BOUNDS => "Invalid bounds specified.",
            ejdb_sys::iw_ecode_IW_ERROR_NOT_IMPLEMENTED => "Method is not implemented.",
            ejdb_sys::iw_ecode_IW_ERROR_ALLOC => "Memory allocation failed.",
            ejdb_sys::iw_ecode_IW_ERROR_INVALID_STATE => "Illegal state error.",
            ejdb_sys::iw_ecode_IW_ERROR_NOT_ALIGNED => "Argument is not aligned properly.",
            ejdb_sys::iw_ecode_IW_ERROR_FALSE => "Request rejection/false response.",
            ejdb_sys::iw_ecode_IW_ERROR_INVALID_ARGS => "Invalid function arguments.",
            ejdb_sys::iw_ecode_IW_ERROR_OVERFLOW => "Overflow.",
            ejdb_sys::iw_ecode_IW_ERROR_INVALID_VALUE => "Invalid value.",
            ejdb_sys::iw_ecode_IW_ERROR_UNEXPECTED_RESPONSE => {
                "Unexpected response (IW_ERROR_UNEXPECTED_RESPONSE)"
            }
            ejdb_sys::iw_ecode_IW_ERROR_NOT_ALLOWED => {
                "Action is not allowed. (IW_ERROR_NOT_ALLOWED)"
            }
            ejdb_sys::iw_ecode_IW_ERROR_UNSUPPORTED => {
                "Unsupported opration. (IW_ERROR_UNSUPPORTED)"
            }
            _ => "Unknown error code",
        }
    }

    pub fn init() -> ejdb_sys::iwrc {
        let rc = unsafe { ejdb_sys::ejdb_init() };

        if rc != 0 {
            println!("error code: {}", EJDB::err_to_str(rc));
        }

        rc
    }

    pub fn new() -> EJDB {
        EJDB {
            db: std::ptr::null_mut(),
        }
    }

    pub fn open(&mut self, path: &str) -> ejdb_sys::iwrc {
        let path_cstr = std::ffi::CString::new(path).expect("CString::new failed");
        let opts = ejdb_sys::EJDB_OPTS {
            kv: ejdb_sys::IWKV_OPTS {
                path: path_cstr.as_ptr(),
                random_seed: 0,
                fmt_version: 0,
                oflags: 0,
                file_lock_fail_fast: false,
                wal: ejdb_sys::IWKV_WAL_OPTS {
                    enabled: true,
                    check_crc_on_checkpoint: false,
                    savepoint_timeout_sec: 0,
                    checkpoint_timeout_sec: 0,
                    wal_buffer_sz: 0,
                    checkpoint_buffer_sz: 0,
                    wal_lock_interceptor: None,
                    wal_lock_interceptor_opaque: std::ptr::null_mut(),
                },
            },
            document_buffer_sz: 0,
            http: ejdb_sys::EJDB_HTTP {
                enabled: false,
                port: 0,
                access_token: std::ptr::null_mut(),
                access_token_len: 0,
                blocking: false,
                cors: false,
                max_body_size: 0,
                bind: std::ptr::null_mut(),
                read_anon: false,
            },
            no_wal: false,
            sort_buffer_sz: 0,
        };

        let rc = unsafe { ejdb_sys::ejdb_open(&opts, &mut self.db) };

        if rc != 0 {
            println!("failed to open db: {}", EJDB::err_to_str(rc));
        }

        rc
    }

    pub fn put_new<I: EJDBSerializable<O>, O >(&self, collection: &str, json_str: &I) -> Result<i64, ejdb_sys::iwrc> {
        let mut jbl: ejdb_sys::JBL = match json_str.to_jbl() {
            Result::Ok(val) => val,
            Result::Err(err) => {
                println!("json error: {}", EJDB::err_to_str(err));
                return Err(err);
            }
        };

        let mut id: i64 = 0;
        let rc4 = unsafe {
            ejdb_sys::ejdb_put_new(
                self.db,
                std::ffi::CString::new(collection).unwrap().as_ptr(),
                jbl,
                &mut id,
            )
        };

        if rc4 != 0 {
            println!("failed to put: {}", EJDB::err_to_str(rc4));
            unsafe { ejdb_sys::jbl_destroy(&mut jbl) };
            return Err(rc4);
        }
        Ok(id)
    }

    pub fn close(&mut self) -> ejdb_sys::iwrc {
        let rc = unsafe { ejdb_sys::ejdb_close(&mut self.db) };
        if rc != 0 {
            println!("failed to close db: {}", EJDB::err_to_str(rc));
            return rc;
        }
        self.db = std::ptr::null_mut();
        rc
    }

    pub fn patch<I: EJDBSerializable<O>, O >(&self, collection: &str, json_str: &I, id: i64) -> ejdb_sys::iwrc {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let mut jbl: ejdb_sys::JBL = match json_str.to_jbl() {
            Result::Ok(val) => val,
            Result::Err(err) => {
                println!("json error: {}", EJDB::err_to_str(err));
                return err;
            }
        };

        let rc =
            unsafe { ejdb_sys::ejdb_patch_jbl(self.db, collection_str.as_ptr(),jbl, id) };

        if rc != 0 {
            println!("failed to patch {} {}", id, rc);
        }
        unsafe {ejdb_sys::jbl_destroy(&mut jbl)};
        rc
    }

    pub fn get< O :EJDBSerializable <O> >(&self, collection: &str, id: i64) -> Result<O, ejdb_sys::iwrc> {
        let mut jbl: ejdb_sys::JBL = std::ptr::null_mut();
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let rc = unsafe { ejdb_sys::ejdb_get(self.db, collection_str.as_ptr(), id, &mut jbl) };
        if rc != 0 {
            println!("failed to get {} {}", id, rc);
            unsafe { ejdb_sys::jbl_destroy(&mut jbl) };
            return Err(rc);
        } else if jbl == std::ptr::null_mut() {
            return Err(0);
        }

        let result:O = match <O>::from_jbl(jbl) {
            Result::Ok(val) => {
                unsafe { ejdb_sys::jbl_destroy(&mut jbl) };
                val
            },
            Result::Err(err) => {
                unsafe { ejdb_sys::jbl_destroy(&mut jbl) };
                return Err(err);
            }
        };
        
        Ok(result)
    }

    pub fn info< O :EJDBSerializable <O> >(&self) -> Result<O, ejdb_sys::iwrc> {
        let mut jbl2: ejdb_sys::JBL = std::ptr::null_mut();

        let rc4 = unsafe { ejdb_sys::ejdb_get_meta(self.db, &mut jbl2) };

        if rc4 != 0 {
            println!("failed to get db meta {}", rc4);
            unsafe { ejdb_sys::jbl_destroy(&mut jbl2) };
            return Err(rc4);
        }

        let result:O = match <O>::from_jbl(jbl2) {
            Result::Ok(val) => {
                unsafe { ejdb_sys::jbl_destroy(&mut jbl2) };
                val
            },
            Result::Err(err) => {
                unsafe { ejdb_sys::jbl_destroy(&mut jbl2) };
                return Err(err);
            }
        };
       
        Ok(result)
    }

    pub fn del(&self, collection: &str, id: i64) -> ejdb_sys::iwrc {
        let collection_str = std::ffi::CString::new(collection).unwrap();

        let rc4 = unsafe { ejdb_sys::ejdb_del(self.db, collection_str.as_ptr(), id) };

        if rc4 != 0 {
            println!("failed to del {} {}", id, rc4);
        }

        rc4
    }

    pub fn rename_collection(
        &self,
        old_collection_name: &str,
        new_collection_name: &str,
    ) -> ejdb_sys::iwrc {
        let old_collection_name_str = std::ffi::CString::new(old_collection_name).unwrap();
        let new_collection_name_str = std::ffi::CString::new(new_collection_name).unwrap();

        let rc = unsafe {
            ejdb_sys::ejdb_rename_collection(
                self.db,
                old_collection_name_str.as_ptr(),
                new_collection_name_str.as_ptr(),
            )
        };

        if rc != 0 {
            println!("failed to rename collection {} {}", old_collection_name, rc);
        }

        rc
    }

    pub fn ensure_index(
        &self,
        collection: &str,
        path: &str,
        mode: ejdb_sys::ejdb_idx_mode_t,
    ) -> ejdb_sys::iwrc {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let path_str = std::ffi::CString::new(path).unwrap();

        let rc = unsafe {
            ejdb_sys::ejdb_ensure_index(self.db, collection_str.as_ptr(), path_str.as_ptr(), mode)
        };

        if rc != 0 {
            println!(
                "failed to ensure index for collection {} {} {} {}",
                collection, path, mode, rc
            );
        }
        rc
    }

    pub fn remove_index(
        &self,
        collection: &str,
        path: &str,
        mode: ejdb_sys::ejdb_idx_mode_t,
    ) -> ejdb_sys::iwrc {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let path_str = std::ffi::CString::new(path).unwrap();

        let rc = unsafe {
            ejdb_sys::ejdb_remove_index(self.db, collection_str.as_ptr(), path_str.as_ptr(), mode)
        };

        if rc != 0 {
            println!(
                "failed to remove index for collection {} {} {} {}",
                collection, path, mode, rc
            );
        }
        rc
    }

    pub fn remove_collection(&self, collection: &str) -> ejdb_sys::iwrc {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let rc = unsafe { ejdb_sys::ejdb_remove_collection(self.db, collection_str.as_ptr()) };

        if rc != 0 {
            println!("failed to remove collection {} {}", collection, rc);
        }

        rc
    }

    pub fn online_backup(&self, filename: &str) -> Result<u64, ejdb_sys::iwrc> {
        let mut ts: u64 = 0;
        let filename_str = std::ffi::CString::new(filename).unwrap();
        let rc = unsafe { ejdb_sys::ejdb_online_backup(self.db, &mut ts, filename_str.as_ptr()) };

        if rc != 0 {
            println!("failed to backup {}", rc);
            return Err(rc);
        }

        return Ok(ts);
    }

    pub fn exec< O :EJDBSerializable <O> >(
        &self,
        q: &EJDBQuery, 
        mut f: fn(doc: O) -> ejdb_sys::iwrc,
    ) -> ejdb_sys::iwrc {

        let callback_ptr: *mut std::ffi::c_void = &mut f as *mut _ as *mut std::ffi::c_void;

        let mut ux = ejdb_sys::EJDB_EXEC {
            db: self.db,
            cnt: 0,
            limit: 0,
            log: std::ptr::null_mut(),
            opaque: callback_ptr,
            pool: std::ptr::null_mut(),
            skip: 0,
            visitor: Some(document_visitor::<O>),
            q: q.q,
        };

        let rc3 = unsafe { ejdb_sys::ejdb_exec(&mut ux) };

        if rc3 != 0 {
            println!("unable to exec {}", rc3)
        }

        return rc3;
    }
}

unsafe extern "C" fn document_visitor< O :EJDBSerializable <O> >(
    ctx: *mut ejdb_sys::_EJDB_EXEC,
    doc: ejdb_sys::EJDB_DOC,
    step: *mut i64,
) -> ejdb_sys::iwrc {

    let result:O = match <O>::from_jbl((*doc).raw) {
        Result::Ok(val) => {
            val
        },
        Result::Err(err) => {
            return err;
        }
    };

    let data: &mut fn(O) -> ejdb_sys::iwrc =
        &mut *((*ctx).opaque
            as *mut fn(O) -> ejdb_sys::iwrc);

    return data(result);
}
