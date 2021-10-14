extern crate ejdb2_sys;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyclass]
struct EJDB {
    db: ejdb2_sys::EJDB,
}

unsafe impl Send for EJDB {}
unsafe impl Sync for EJDB {}

pub trait EJDBSerializable<T> {
    fn from_jbl(jbl: ejdb2_sys::JBL, py: pyo3::prelude::Python<'_>) -> Result<T, ejdb2_sys::iwrc>;

    fn to_jbl(&self, py: pyo3::prelude::Python<'_>) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc>;
}

impl EJDBSerializable<String> for &str {
    fn from_jbl(
        jbl: ejdb2_sys::JBL,
        _py: pyo3::prelude::Python<'_>,
    ) -> Result<String, ejdb2_sys::iwrc> {
        let xstr = unsafe { ejdb2_sys::iwxstr_new() };

        let rc5 = unsafe {
            ejdb2_sys::jbl_as_json(
                jbl,
                Some(ejdb2_sys::jbl_xstr_json_printer),
                xstr as *mut ::std::os::raw::c_void,
                1,
            )
        };

        if rc5 != 0 {
            println!("failed to convert json to str {}", rc5);
            unsafe {
                ejdb2_sys::iwxstr_destroy(xstr);
            };
            return Err(rc5);
        }

        let str2 = unsafe {
            std::ffi::CStr::from_ptr(ejdb2_sys::iwxstr_ptr(xstr))
                .to_str()
                .unwrap()
        };

        let result = String::from(str2);
        unsafe {
            ejdb2_sys::iwxstr_destroy(xstr);
        };

        return Ok(result);
    }

    fn to_jbl(&self, _py: pyo3::prelude::Python<'_>) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc> {
        let mut jbl: ejdb2_sys::JBL = std::ptr::null_mut();
        let json = std::ffi::CString::new(*self).unwrap();
        let rc3 = unsafe { ejdb2_sys::jbl_from_json(&mut jbl, json.as_ptr()) };

        if rc3 != 0 {
            println!("json error: {}", err_to_str(rc3));
            unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
            return Err(rc3);
        }
        return Ok(jbl);
    }
}

impl EJDBSerializable<String> for String {
    fn from_jbl(
        jbl: ejdb2_sys::JBL,
        _py: pyo3::prelude::Python<'_>,
    ) -> Result<String, ejdb2_sys::iwrc> {
        let xstr = unsafe { ejdb2_sys::iwxstr_new() };

        let rc5 = unsafe {
            ejdb2_sys::jbl_as_json(
                jbl,
                Some(ejdb2_sys::jbl_xstr_json_printer),
                xstr as *mut ::std::os::raw::c_void,
                1,
            )
        };

        if rc5 != 0 {
            println!("failed to convert json to str {}", rc5);
            unsafe {
                ejdb2_sys::iwxstr_destroy(xstr);
            };
            return Err(rc5);
        }

        let str2 = unsafe {
            std::ffi::CStr::from_ptr(ejdb2_sys::iwxstr_ptr(xstr))
                .to_str()
                .unwrap()
        };

        let result = String::from(str2);
        unsafe {
            ejdb2_sys::iwxstr_destroy(xstr);
        };

        return Ok(result);
    }

    fn to_jbl(&self, _py: pyo3::prelude::Python<'_>) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc> {
        let mut jbl: ejdb2_sys::JBL = std::ptr::null_mut();
        let json = std::ffi::CString::new(self.as_str()).unwrap();
        let rc3 = unsafe { ejdb2_sys::jbl_from_json(&mut jbl, json.as_ptr()) };

        if rc3 != 0 {
            println!("json error: {}", err_to_str(rc3));
            unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
            return Err(rc3);
        }
        return Ok(jbl);
    }
}

trait EJDBSerializerHelper {
    fn from_jbl_object(
        jbl: ejdb2_sys::JBL,
        py: pyo3::prelude::Python<'_>,
    ) -> Result<pyo3::Py<PyAny>, ejdb2_sys::iwrc>;
    fn from_jbl_array(
        jbl: ejdb2_sys::JBL,
        py: pyo3::prelude::Python<'_>,
    ) -> Result<pyo3::Py<PyAny>, ejdb2_sys::iwrc>;

    fn to_jbl_object(
        object: &pyo3::types::PyAny,
        py: pyo3::prelude::Python<'_>,
    ) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc>;
    fn to_jbl_array(
        array: &pyo3::types::PyAny,
        py: pyo3::prelude::Python<'_>,
    ) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc>;
}

impl EJDBSerializerHelper for pyo3::types::PyAny {
    fn from_jbl_object(
        jbl: ejdb2_sys::JBL,
        py: pyo3::prelude::Python<'_>,
    ) -> Result<pyo3::Py<PyAny>, ejdb2_sys::iwrc> {
        let mut holder: ejdb2_sys::JBL = std::ptr::null_mut();

        let mut it = ejdb2_sys::JBL_iterator {
            pnext: std::ptr::null_mut(),
            plimit: std::ptr::null_mut(),
            type_: 0,
            count: 0,
            current: 0,
        };

        let rc = unsafe { ejdb2_sys::jbl_create_iterator_holder(&mut holder) };
        if rc != 0 {
            unsafe { ejdb2_sys::jbl_destroy(&mut holder) };
            return Err(rc);
        }

        let rc2 = unsafe { ejdb2_sys::jbl_iterator_init(jbl, &mut it) };

        if rc2 != 0 {
            unsafe { ejdb2_sys::jbl_destroy(&mut holder) };
            return Err(rc2);
        }

        let mut key: *mut std::os::raw::c_char = std::ptr::null_mut();
        let mut klen: std::os::raw::c_int = 0;

        let result = pyo3::types::PyDict::new(py);

        while unsafe { ejdb2_sys::jbl_iterator_next(&mut it, holder, &mut key, &mut klen) } {
            let mut dst = [0i8; 256];

            unsafe { std::ptr::copy(key, dst.as_mut_ptr(), klen as usize) };

            let c_str: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(dst.as_ptr()) };
            let str_slice: &str = c_str.to_str().unwrap();

            let jbl_type = unsafe { ejdb2_sys::jbl_type(holder) };

            match jbl_type {
                ejdb2_sys::jbl_type_t_JBV_NONE => {}
                ejdb2_sys::jbl_type_t_JBV_NULL => {
                    //to do, how to initialize NoneType?
                    //result.set_item(str_slice, pyo3::types::PyAny::);
                }
                ejdb2_sys::jbl_type_t_JBV_BOOL => {
                    let value = unsafe { ejdb2_sys::jbl_get_i32(holder) };
                    result
                        .set_item(str_slice, pyo3::types::PyBool::new(py, value != 0))
                        .unwrap();
                }
                ejdb2_sys::jbl_type_t_JBV_I64 => {
                    let value = unsafe { ejdb2_sys::jbl_get_i64(holder) };
                    result.set_item(str_slice, value.into_py(py)).unwrap();
                }
                ejdb2_sys::jbl_type_t_JBV_F64 => {
                    let value = unsafe { ejdb2_sys::jbl_get_f64(holder) };
                    result.set_item(str_slice, value.into_py(py)).unwrap();
                }

                ejdb2_sys::jbl_type_t_JBV_STR => {
                    let value = unsafe { ejdb2_sys::jbl_get_str(holder) };
                    let value_c_str = unsafe { std::ffi::CStr::from_ptr(value) };
                    result
                        .set_item(str_slice, value_c_str.to_str().unwrap().into_py(py))
                        .unwrap();
                }

                ejdb2_sys::jbl_type_t_JBV_OBJECT => {
                    let map = <pyo3::types::PyAny>::from_jbl_object(holder, py).unwrap();
                    result.set_item(str_slice, map).unwrap();
                }
                ejdb2_sys::jbl_type_t_JBV_ARRAY => {
                    let arr = <pyo3::types::PyAny>::from_jbl_array(holder, py).unwrap();
                    result.set_item(str_slice, arr).unwrap();
                }
                _ => {}
            };
        }
        unsafe { ejdb2_sys::jbl_destroy(&mut holder) };

        // let a:& PyAny = result.into;

        return Ok(result.into_py(py));
        //Err(0)
    }

    fn from_jbl_array(
        jbl: ejdb2_sys::JBL,
        py: pyo3::prelude::Python<'_>,
    ) -> Result<pyo3::Py<PyAny>, ejdb2_sys::iwrc> {
        let mut holder: ejdb2_sys::JBL = std::ptr::null_mut();

        let mut it = ejdb2_sys::JBL_iterator {
            pnext: std::ptr::null_mut(),
            plimit: std::ptr::null_mut(),
            type_: 0,
            count: 0,
            current: 0,
        };

        let rc = unsafe { ejdb2_sys::jbl_create_iterator_holder(&mut holder) };
        if rc != 0 {
            unsafe { ejdb2_sys::jbl_destroy(&mut holder) };
            return Err(rc);
        }

        let rc2 = unsafe { ejdb2_sys::jbl_iterator_init(jbl, &mut it) };

        if rc2 != 0 {
            unsafe { ejdb2_sys::jbl_destroy(&mut holder) };
            return Err(rc2);
        }

        let mut key: *mut std::os::raw::c_char = std::ptr::null_mut();
        let mut klen: std::os::raw::c_int = 0;

        let result = pyo3::types::PyList::empty(py);

        while unsafe { ejdb2_sys::jbl_iterator_next(&mut it, holder, &mut key, &mut klen) } {
            let jbl_type = unsafe { ejdb2_sys::jbl_type(holder) };

            match jbl_type {
                ejdb2_sys::jbl_type_t_JBV_NONE => {}
                ejdb2_sys::jbl_type_t_JBV_NULL => {
                    //result.push(serde_json::Value::Null);
                }
                ejdb2_sys::jbl_type_t_JBV_BOOL => {
                    let value = unsafe { ejdb2_sys::jbl_get_i32(holder) };
                    result
                        .append(pyo3::types::PyBool::new(py, value != 0))
                        .unwrap();
                }
                ejdb2_sys::jbl_type_t_JBV_I64 => {
                    let value = unsafe { ejdb2_sys::jbl_get_i64(holder) };
                    result.append(value.into_py(py)).unwrap();
                }
                ejdb2_sys::jbl_type_t_JBV_F64 => {
                    let value = unsafe { ejdb2_sys::jbl_get_f64(holder) };
                    result.append(value.into_py(py)).unwrap();
                }

                ejdb2_sys::jbl_type_t_JBV_STR => {
                    let value = unsafe { ejdb2_sys::jbl_get_str(holder) };
                    let value_c_str = unsafe { std::ffi::CStr::from_ptr(value) };
                    result
                        .append(value_c_str.to_str().unwrap().into_py(py))
                        .unwrap();
                }

                ejdb2_sys::jbl_type_t_JBV_OBJECT => {
                    let map = <pyo3::types::PyAny>::from_jbl_object(holder, py).unwrap();
                    result.append(map).unwrap();
                }
                ejdb2_sys::jbl_type_t_JBV_ARRAY => {
                    let arr = <pyo3::types::PyAny>::from_jbl_array(holder, py).unwrap();
                    result.append(arr).unwrap();
                }
                _ => {}
            };
        }
        unsafe { ejdb2_sys::jbl_destroy(&mut holder) };

        return Ok(result.into_py(py));
    }

    fn to_jbl_object(
        object: &pyo3::types::PyAny,
        py: pyo3::prelude::Python<'_>,
    ) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc> {
        let mut jbl: ejdb2_sys::JBL = std::ptr::null_mut();
        let rc = unsafe { ejdb2_sys::jbl_create_empty_object(&mut jbl) };

        if rc != 0 {
            unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
            return Err(rc);
        }

        for (key, value) in object.downcast::<pyo3::types::PyDict>().unwrap().iter() {
            let key_str = std::ffi::CString::new(
                key.downcast::<pyo3::types::PyString>().unwrap().to_string(),
            )
            .unwrap();
            if value.is_instance::<pyo3::types::PyString>().unwrap() {
                let val_str = std::ffi::CString::new(
                    value
                        .downcast::<pyo3::types::PyString>()
                        .unwrap()
                        .to_string(),
                )
                .unwrap();
                unsafe { ejdb2_sys::jbl_set_string(jbl, key_str.as_ptr(), val_str.as_ptr()) };
            } else if value.is_instance::<pyo3::types::PyList>().unwrap()
                || value.is_instance::<pyo3::types::PySet>().unwrap()
            {
                let mut nested = <pyo3::types::PyAny>::to_jbl_array(value, py).unwrap();

                unsafe { ejdb2_sys::jbl_set_nested(jbl, std::ptr::null(), nested) };

                unsafe { ejdb2_sys::jbl_destroy(&mut nested) };
            } else if value.is_instance::<pyo3::types::PyFloat>().unwrap() {
                let val = value.downcast::<pyo3::types::PyFloat>().unwrap();
                unsafe { ejdb2_sys::jbl_set_f64(jbl, key_str.as_ptr(), val.value()) };
            } else if value.is_instance::<pyo3::types::PyInt>().unwrap() {
                let val = value
                    .downcast::<pyo3::types::PyInt>()
                    .unwrap()
                    .to_object(py)
                    .extract::<i64>(py)
                    .unwrap();
                unsafe { ejdb2_sys::jbl_set_int64(jbl, key_str.as_ptr(), val) };
            } else if value.is_instance::<pyo3::types::PyBool>().unwrap() {
                let val = value.downcast::<pyo3::types::PyBool>().unwrap();
                unsafe { ejdb2_sys::jbl_set_bool(jbl, key_str.as_ptr(), val.is_true()) };
            } else if value.is_instance::<pyo3::types::PyDict>().unwrap() {
                let mut nested = <pyo3::types::PyAny>::to_jbl_object(value, py).unwrap();

                unsafe { ejdb2_sys::jbl_set_nested(jbl, key_str.as_ptr(), nested) };

                unsafe { ejdb2_sys::jbl_destroy(&mut nested) };
            } else {
                return Err(0);
            }
        }

        Ok(jbl)
    }

    fn to_jbl_array(
        array: &pyo3::types::PyAny,
        py: pyo3::prelude::Python<'_>,
    ) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc> {
        let mut jbl: ejdb2_sys::JBL = std::ptr::null_mut();
        let rc = unsafe { ejdb2_sys::jbl_create_empty_object(&mut jbl) };

        if rc != 0 {
            unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
            return Err(rc);
        }

        if array.is_instance::<pyo3::types::PyList>().unwrap() {
            for value in array.downcast::<pyo3::types::PyList>().unwrap().iter() {
                if value.is_instance::<pyo3::types::PyString>().unwrap() {
                    let val_str = std::ffi::CString::new(
                        value
                            .downcast::<pyo3::types::PyString>()
                            .unwrap()
                            .to_string(),
                    )
                    .unwrap();
                    unsafe { ejdb2_sys::jbl_set_string(jbl, std::ptr::null(), val_str.as_ptr()) };
                } else if value.is_instance::<pyo3::types::PyList>().unwrap()
                    || value.is_instance::<pyo3::types::PySet>().unwrap()
                {
                    let mut nested = <pyo3::types::PyAny>::to_jbl_array(value, py).unwrap();

                    unsafe { ejdb2_sys::jbl_set_nested(jbl, std::ptr::null(), nested) };

                    unsafe { ejdb2_sys::jbl_destroy(&mut nested) };
                } else if value.is_instance::<pyo3::types::PyFloat>().unwrap() {
                    let val = value.downcast::<pyo3::types::PyFloat>().unwrap();
                    unsafe { ejdb2_sys::jbl_set_f64(jbl, std::ptr::null(), val.value()) };
                } else if value.is_instance::<pyo3::types::PyInt>().unwrap() {
                    let val = value
                        .downcast::<pyo3::types::PyInt>()
                        .unwrap()
                        .to_object(py)
                        .extract::<i64>(py)
                        .unwrap();
                    unsafe { ejdb2_sys::jbl_set_int64(jbl, std::ptr::null(), val) };
                } else if value.is_instance::<pyo3::types::PyBool>().unwrap() {
                    let val = value.downcast::<pyo3::types::PyBool>().unwrap();
                    unsafe { ejdb2_sys::jbl_set_bool(jbl, std::ptr::null(), val.is_true()) };
                } else if value.is_instance::<pyo3::types::PyDict>().unwrap() {
                    let mut nested = <pyo3::types::PyAny>::to_jbl_object(value, py).unwrap();

                    unsafe { ejdb2_sys::jbl_set_nested(jbl, std::ptr::null(), nested) };

                    unsafe { ejdb2_sys::jbl_destroy(&mut nested) };
                } else {
                    return Err(0);
                }
            }
        } else if array.is_instance::<pyo3::types::PySet>().unwrap() {
            for value in array.downcast::<pyo3::types::PySet>().unwrap().iter() {
                if value.is_instance::<pyo3::types::PyString>().unwrap() {
                    let val_str = std::ffi::CString::new(
                        value
                            .downcast::<pyo3::types::PyString>()
                            .unwrap()
                            .to_string(),
                    )
                    .unwrap();
                    unsafe { ejdb2_sys::jbl_set_string(jbl, std::ptr::null(), val_str.as_ptr()) };
                } else if value.is_instance::<pyo3::types::PyList>().unwrap() {
                    let mut nested = <pyo3::types::PyAny>::to_jbl_array(value, py).unwrap();

                    unsafe { ejdb2_sys::jbl_set_nested(jbl, std::ptr::null(), nested) };

                    unsafe { ejdb2_sys::jbl_destroy(&mut nested) };
                } else if value.is_instance::<pyo3::types::PyFloat>().unwrap() {
                    let val = value.downcast::<pyo3::types::PyFloat>().unwrap();
                    unsafe { ejdb2_sys::jbl_set_f64(jbl, std::ptr::null(), val.value()) };
                } else if value.is_instance::<pyo3::types::PyInt>().unwrap() {
                    let val = value
                        .downcast::<pyo3::types::PyInt>()
                        .unwrap()
                        .to_object(py)
                        .extract::<i64>(py)
                        .unwrap();
                    unsafe { ejdb2_sys::jbl_set_int64(jbl, std::ptr::null(), val) };
                } else if value.is_instance::<pyo3::types::PyBool>().unwrap() {
                    let val = value.downcast::<pyo3::types::PyBool>().unwrap();
                    unsafe { ejdb2_sys::jbl_set_bool(jbl, std::ptr::null(), val.is_true()) };
                } else if value.is_instance::<pyo3::types::PyDict>().unwrap() {
                    let mut nested = <pyo3::types::PyAny>::to_jbl_object(value, py).unwrap();

                    unsafe { ejdb2_sys::jbl_set_nested(jbl, std::ptr::null(), nested) };

                    unsafe { ejdb2_sys::jbl_destroy(&mut nested) };
                } else {
                    return Err(0);
                }
            }
        } else {
            return Err(0);
        }
        Ok(jbl)
    }
}

impl EJDBSerializable<pyo3::Py<PyAny>> for pyo3::types::PyAny {
    fn from_jbl(
        jbl: ejdb2_sys::JBL,
        py: pyo3::prelude::Python<'_>,
    ) -> Result<pyo3::Py<PyAny>, ejdb2_sys::iwrc> {
        let jbl_type = unsafe { ejdb2_sys::jbl_type(jbl) };

        match jbl_type {
            ejdb2_sys::jbl_type_t_JBV_OBJECT => {
                let map = <pyo3::types::PyAny>::from_jbl_object(jbl, py).unwrap();
                return Ok(map);
            }
            ejdb2_sys::jbl_type_t_JBV_ARRAY => {
                let arr = <pyo3::types::PyAny>::from_jbl_array(jbl, py).unwrap();
                return Ok(arr);
            }
            _ => {
                println!("only object and array types are serializable.");
                return Err(0);
            }
        };
    }

    fn to_jbl(&self, py: pyo3::prelude::Python<'_>) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc> {
        if self.is_instance::<pyo3::types::PyDict>().unwrap() {
            let nested = <pyo3::types::PyAny>::to_jbl_object(self, py).unwrap();
            return Ok(nested);
        } else if self.is_instance::<pyo3::types::PyList>().unwrap() {
            let nested = <pyo3::types::PyAny>::to_jbl_array(self, py).unwrap();
            return Ok(nested);
        } else if self.is_instance::<pyo3::types::PySet>().unwrap() {
            let nested = <pyo3::types::PyAny>::to_jbl_array(self, py).unwrap();
            return Ok(nested);
        } else {
            return Err(0);
        }
    }
}

fn err_to_str(mut rc: ejdb2_sys::iwrc) -> &'static str {
    let error_code = unsafe { ejdb2_sys::iwrc_strip_errno(&mut rc) };

    match error_code {
        ejdb2_sys::iw_ecode_IW_OK => "No error.",
        ejdb2_sys::iw_ecode_IW_ERROR_FAIL => "Unspecified error.",
        ejdb2_sys::iw_ecode_IW_ERROR_ERRNO => "Error with expected errno status set.",
        ejdb2_sys::iw_ecode_IW_ERROR_IO_ERRNO => "IO error with expected errno status set.",
        ejdb2_sys::iw_ecode_IW_ERROR_AGAIN => "Again",
        ejdb2_sys::iw_ecode_IW_ERROR_NOT_EXISTS => "Resource is not exists.",
        ejdb2_sys::iw_ecode_IW_ERROR_READONLY => "Resource is readonly.",
        ejdb2_sys::iw_ecode_IW_ERROR_ALREADY_OPENED => "Resource is already opened.",
        ejdb2_sys::iw_ecode_IW_ERROR_THREADING => "Threading error.",
        ejdb2_sys::iw_ecode_IW_ERROR_THREADING_ERRNO => "Threading error with errno status set.",
        ejdb2_sys::iw_ecode_IW_ERROR_ASSERTION => "Generic assertion error.",
        ejdb2_sys::iw_ecode_IW_ERROR_INVALID_HANDLE => "Invalid HANDLE value.",
        ejdb2_sys::iw_ecode_IW_ERROR_OUT_OF_BOUNDS => "Invalid bounds specified.",
        ejdb2_sys::iw_ecode_IW_ERROR_NOT_IMPLEMENTED => "Method is not implemented.",
        ejdb2_sys::iw_ecode_IW_ERROR_ALLOC => "Memory allocation failed.",
        ejdb2_sys::iw_ecode_IW_ERROR_INVALID_STATE => "Illegal state error.",
        ejdb2_sys::iw_ecode_IW_ERROR_NOT_ALIGNED => "Argument is not aligned properly.",
        ejdb2_sys::iw_ecode_IW_ERROR_FALSE => "Request rejection/false response.",
        ejdb2_sys::iw_ecode_IW_ERROR_INVALID_ARGS => "Invalid function arguments.",
        ejdb2_sys::iw_ecode_IW_ERROR_OVERFLOW => "Overflow.",
        ejdb2_sys::iw_ecode_IW_ERROR_INVALID_VALUE => "Invalid value.",
        ejdb2_sys::iw_ecode_IW_ERROR_UNEXPECTED_RESPONSE => {
            "Unexpected response (IW_ERROR_UNEXPECTED_RESPONSE)"
        }
        ejdb2_sys::iw_ecode_IW_ERROR_NOT_ALLOWED => "Action is not allowed. (IW_ERROR_NOT_ALLOWED)",
        ejdb2_sys::iw_ecode_IW_ERROR_UNSUPPORTED => "Unsupported opration. (IW_ERROR_UNSUPPORTED)",
        _ => "Unknown error code",
    }
}

#[pymethods]
impl EJDB {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(EJDB {
            db: std::ptr::null_mut(),
        })
    }

    #[staticmethod]
    fn init() -> PyResult<ejdb2_sys::iwrc> {
        let rc = unsafe { ejdb2_sys::ejdb_init() };

        if rc != 0 {
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }

        Ok(rc)
    }

    pub fn open(&mut self, path: &str) -> PyResult<ejdb2_sys::iwrc> {
        let path_cstr = std::ffi::CString::new(path).expect("CString::new failed");
        let opts = ejdb2_sys::EJDB_OPTS {
            kv: ejdb2_sys::IWKV_OPTS {
                path: path_cstr.as_ptr(),
                random_seed: 0,
                fmt_version: 0,
                oflags: 0,
                file_lock_fail_fast: false,
                wal: ejdb2_sys::IWKV_WAL_OPTS {
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
            http: ejdb2_sys::EJDB_HTTP {
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

        let rc = unsafe { ejdb2_sys::ejdb_open(&opts, &mut self.db) };

        if rc != 0 {
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }

        Ok(0)
    }

    pub fn put_new(
        &self,
        py: pyo3::prelude::Python<'_>,
        collection: &str,
        json: &pyo3::types::PyAny,
    ) -> PyResult<i64> {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let mut jbl: ejdb2_sys::JBL;

        if json.is_instance::<pyo3::types::PyString>().unwrap() {
            let py_str = json.downcast::<pyo3::types::PyString>().unwrap();

            jbl = match py_str.to_str().unwrap().to_jbl(py) {
                Result::Ok(val) => val,
                Result::Err(err) => {
                    return Err(pyo3::exceptions::PyValueError::new_err(err_to_str(err)));
                }
            };
        } else {
            jbl = match json.to_jbl(py) {
                Result::Ok(val) => val,
                Result::Err(err) => {
                    return Err(pyo3::exceptions::PyValueError::new_err(err_to_str(err)));
                }
            };
        }
        let mut id: i64 = 0;
        let rc4 =
            unsafe { ejdb2_sys::ejdb_put_new(self.db, collection_str.as_ptr(), jbl, &mut id) };

        if rc4 != 0 {
            unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc4)));
        }
        Ok(id)
    }

    pub fn put(
        &self,
        py: pyo3::prelude::Python<'_>,
        collection: &str,
        json: &pyo3::types::PyAny,
        id: i64,
    ) -> PyResult<ejdb2_sys::iwrc> {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let mut jbl: ejdb2_sys::JBL;

        if json.is_instance::<pyo3::types::PyString>().unwrap() {
            let py_str = json.downcast::<pyo3::types::PyString>().unwrap();
            jbl = match py_str.to_str().unwrap().to_jbl(py) {
                Result::Ok(val) => val,
                Result::Err(err) => {
                    println!("json error: {}", err_to_str(err));
                    return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(err)));
                }
            };
        } else {
            jbl = match json.to_jbl(py) {
                Result::Ok(val) => val,
                Result::Err(err) => {
                    return Err(pyo3::exceptions::PyValueError::new_err(err_to_str(err)));
                }
            };
        }

        let rc4 = unsafe { ejdb2_sys::ejdb_put(self.db, collection_str.as_ptr(), jbl, id) };

        if rc4 != 0 {
            println!("failed to put: {}", err_to_str(rc4));
            unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc4)));
        }
        Ok(0)
    }

    pub fn close(&mut self) -> PyResult<ejdb2_sys::iwrc> {
        let rc = unsafe { ejdb2_sys::ejdb_close(&mut self.db) };
        if rc != 0 {
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }
        self.db = std::ptr::null_mut();
        Ok(0)
    }

    pub fn patch(
        &self,
        py: pyo3::prelude::Python<'_>,
        collection: &str,
        json: &pyo3::types::PyAny,
        id: i64,
    ) -> PyResult<ejdb2_sys::iwrc> {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let mut jbl: ejdb2_sys::JBL;
        if json.is_instance::<pyo3::types::PyString>().unwrap() {
            let py_str = json.downcast::<pyo3::types::PyString>().unwrap();
            jbl = match py_str.to_str().unwrap().to_jbl(py) {
                Result::Ok(val) => val,
                Result::Err(err) => {
                    println!("json error: {}", err_to_str(err));
                    return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(err)));
                }
            };
        } else {
            jbl = match json.to_jbl(py) {
                Result::Ok(val) => val,
                Result::Err(err) => {
                    println!("json error: {}", err_to_str(err));
                    return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(err)));
                }
            };
        }
        let rc = unsafe { ejdb2_sys::ejdb_patch_jbl(self.db, collection_str.as_ptr(), jbl, id) };
        if rc != 0 {
            println!("failed to patch {} {}", id, rc);
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }
        unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
        Ok(0)
    }

    pub fn get(
        &self,
        py: pyo3::prelude::Python<'_>,
        collection: &str,
        id: i64,
    ) -> PyResult<Py<PyAny>> {
        let mut jbl: ejdb2_sys::JBL = std::ptr::null_mut();
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let rc = unsafe { ejdb2_sys::ejdb_get(self.db, collection_str.as_ptr(), id, &mut jbl) };
        if rc != 0 {
            println!("failed to get {} {}", id, rc);
            unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        } else if jbl == std::ptr::null_mut() {
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(0)));
        }

        let result = match <pyo3::types::PyAny>::from_jbl(jbl, py) {
            Result::Ok(val) => {
                unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
                val
            }
            Result::Err(err) => {
                unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
                return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(err)));
            }
        };
        Ok(result)
    }

    pub fn info(&self, py: pyo3::prelude::Python<'_>) -> PyResult<Py<PyAny>> {
        let mut jbl2: ejdb2_sys::JBL = std::ptr::null_mut();

        let rc4 = unsafe { ejdb2_sys::ejdb_get_meta(self.db, &mut jbl2) };

        if rc4 != 0 {
            println!("failed to get db meta {}", rc4);
            unsafe { ejdb2_sys::jbl_destroy(&mut jbl2) };
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc4)));
        }

        let result = match <pyo3::types::PyAny>::from_jbl(jbl2, py) {
            Result::Ok(val) => {
                unsafe { ejdb2_sys::jbl_destroy(&mut jbl2) };
                val
            }
            Result::Err(err) => {
                unsafe { ejdb2_sys::jbl_destroy(&mut jbl2) };
                return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(err)));
            }
        };
        Ok(result)
    }

    pub fn del(&self, collection: &str, id: i64) -> PyResult<ejdb2_sys::iwrc> {
        let collection_str = std::ffi::CString::new(collection).unwrap();

        let rc4 = unsafe { ejdb2_sys::ejdb_del(self.db, collection_str.as_ptr(), id) };

        if rc4 != 0 {
            println!("failed to del {} {}", id, rc4);
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc4)));
        }

        Ok(0)
    }

    pub fn rename_collection(
        &self,
        old_collection_name: &str,
        new_collection_name: &str,
    ) -> PyResult<ejdb2_sys::iwrc> {
        let old_collection_name_str = std::ffi::CString::new(old_collection_name).unwrap();
        let new_collection_name_str = std::ffi::CString::new(new_collection_name).unwrap();

        let rc = unsafe {
            ejdb2_sys::ejdb_rename_collection(
                self.db,
                old_collection_name_str.as_ptr(),
                new_collection_name_str.as_ptr(),
            )
        };

        if rc != 0 {
            println!("failed to rename collection {} {}", old_collection_name, rc);
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }

        Ok(0)
    }

    pub fn ensure_index(
        &self,
        collection: &str,
        path: &str,
        mode: ejdb2_sys::ejdb_idx_mode_t,
    ) -> PyResult<ejdb2_sys::iwrc> {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let path_str = std::ffi::CString::new(path).unwrap();

        let rc = unsafe {
            ejdb2_sys::ejdb_ensure_index(self.db, collection_str.as_ptr(), path_str.as_ptr(), mode)
        };

        if rc != 0 {
            println!(
                "failed to ensure index for collection {} {} {} {}",
                collection, path, mode, rc
            );
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }
        Ok(0)
    }

    pub fn remove_index(
        &self,
        collection: &str,
        path: &str,
        mode: ejdb2_sys::ejdb_idx_mode_t,
    ) -> PyResult<ejdb2_sys::iwrc> {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let path_str = std::ffi::CString::new(path).unwrap();

        let rc = unsafe {
            ejdb2_sys::ejdb_remove_index(self.db, collection_str.as_ptr(), path_str.as_ptr(), mode)
        };

        if rc != 0 {
            println!(
                "failed to remove index for collection {} {} {} {}",
                collection, path, mode, rc
            );
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }
        Ok(0)
    }

    pub fn remove_collection(&self, collection: &str) -> PyResult<ejdb2_sys::iwrc> {
        let collection_str = std::ffi::CString::new(collection).unwrap();
        let rc = unsafe { ejdb2_sys::ejdb_remove_collection(self.db, collection_str.as_ptr()) };

        if rc != 0 {
            println!("failed to remove collection {} {}", collection, rc);
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }

        Ok(0)
    }

    pub fn online_backup(&self, filename: &str) -> PyResult<u64> {
        let mut ts: u64 = 0;
        let filename_str = std::ffi::CString::new(filename).unwrap();
        let rc = unsafe { ejdb2_sys::ejdb_online_backup(self.db, &mut ts, filename_str.as_ptr()) };

        if rc != 0 {
            println!("failed to backup {}", rc);
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }

        return Ok(ts);
    }

    pub fn debug(&self, dict: &pyo3::types::PyAny) -> PyResult<i32> {
        /*
        for (key, value) in dict {
            println!("key {:?} {:?} value {:?} {:?}", key, key.get_type().name().unwrap(), value, value.get_type().name().unwrap());
        }*/

        println!("{:?}", dict.get_type().name().unwrap());

        Ok(0)
    }

    pub fn exec(
        &self,
        py: pyo3::prelude::Python<'_>,
        q: &EJDBQuery,
        f: & pyo3::types::PyList,
    ) -> PyResult<ejdb2_sys::iwrc> {

        let mut ctx = ExecCtxWrapper{f:f, py:py};

        let callback_ptr: *mut std::ffi::c_void = &mut ctx as *mut _ as *mut std::ffi::c_void;

        let mut ux = ejdb2_sys::EJDB_EXEC {
            db: self.db,
            cnt: 0,
            limit: 0,
            log: std::ptr::null_mut(),
            opaque: callback_ptr,
            pool: std::ptr::null_mut(),
            skip: 0,
            visitor: Some(document_visitor),
            q: q.q,
        };

        let rc3 = unsafe { ejdb2_sys::ejdb_exec(&mut ux) };
        println!("after exec");
        if rc3 != 0 {
            println!("unable to exec {}", rc3);
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc3)));
        }

        Ok(0)
    }
}

struct ExecCtxWrapper<'a> {
    pub f: &'a pyo3::types::PyList,
    pub py: pyo3::prelude::Python<'a>
}

unsafe extern "C" fn document_visitor(
    ctx: *mut ejdb2_sys::_EJDB_EXEC,
    doc: ejdb2_sys::EJDB_DOC,
    _step: *mut i64,
) -> ejdb2_sys::iwrc {

    let data: &mut ExecCtxWrapper= &mut *((*ctx).opaque as *mut ExecCtxWrapper);

    
    let result = match <pyo3::types::PyAny>::from_jbl((*doc).raw,data.py) {
        Result::Ok(val) => val,
        Result::Err(err) => {
            return err;
        }
    };
    let v = vec![(*doc).id.into_py(data.py),  result];
    data.f.append(pyo3::types::PyTuple::new(data.py, v)).unwrap();
    0
}

#[pyclass]
pub struct EJDBQuery {
    q: ejdb2_sys::JQL,
    query: String,
    collection: String,
}

unsafe impl Send for EJDBQuery {}
unsafe impl Sync for EJDBQuery {}

pub trait SetPlaceholder<T> {
    fn set_placeholder(&self, placeholder: &str, index: i32, val: T)
        -> Result<(), ejdb2_sys::iwrc>;
}

#[pymethods]
impl EJDBQuery {
    #[new]
    pub fn new(collection: &str, query: &str) -> PyResult<Self> {
        Ok(EJDBQuery {
            q: std::ptr::null_mut(),
            query: String::from(query),
            collection: String::from(collection),
        })
    }

    pub fn init(&mut self) -> PyResult<ejdb2_sys::iwrc> {
        let collection_str = std::ffi::CString::new(self.collection.as_str()).unwrap();
        let query_str = std::ffi::CString::new(self.query.as_str()).unwrap();
        let rc = unsafe {
            ejdb2_sys::jql_create(&mut self.q, collection_str.as_ptr(), query_str.as_ptr())
        };

        if rc != 0 {
            unsafe { ejdb2_sys::jql_destroy(&mut self.q) };
            self.q = std::ptr::null_mut();
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }
        Ok(0)
    }

    pub fn limit(&self) -> PyResult<i64> {
        let mut out: i64 = 0;
        let rc = unsafe { ejdb2_sys::jql_get_limit(self.q, &mut out) };

        if rc != 0 {
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }

        return Ok(out);
    }

    pub fn set_placeholder_json(
        &self,
        py: pyo3::prelude::Python<'_>,
        placeholder: &str,
        index: i32,
        val: &pyo3::types::PyAny,
    ) -> PyResult<ejdb2_sys::iwrc> {
        let mut jbl: ejdb2_sys::JBL = std::ptr::null_mut();

        if val.is_instance::<pyo3::types::PyString>().unwrap() {
            let py_str = val.downcast::<pyo3::types::PyString>().unwrap();
            let value_str = std::ffi::CString::new(py_str.to_str().unwrap()).unwrap();

            let rc = unsafe { ejdb2_sys::jbl_from_json(&mut jbl, value_str.as_ptr()) };
            if rc != 0 {
                println!("can't convert str to json {}", rc);
                unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
                return Err(pyo3::exceptions::PyValueError::new_err(err_to_str(rc)));
            }
        } else {
            jbl = val.to_jbl(py).unwrap();
        }
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();

        let rc2 =
            unsafe { ejdb2_sys::jql_set_json_jbl(self.q, placeholder_str.as_ptr(), index, jbl) };
        if rc2 != 0 {
            println!("failed to set placeholder {} to json {}", placeholder, rc2);
            unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc2)));
        }
        unsafe { ejdb2_sys::jbl_destroy(&mut jbl) };
        Ok(0)
    }

    pub fn set_regexp(
        &self,
        placeholder: &str,
        index: i32,
        regexp: &str,
    ) -> PyResult<ejdb2_sys::iwrc> {
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();
        let regexp_str = std::ffi::CString::new(regexp).unwrap();

        let rc = unsafe {
            ejdb2_sys::jql_set_regexp(self.q, placeholder_str.as_ptr(), index, regexp_str.as_ptr())
        };

        if rc != 0 {
            println!("failed to set placeholder {} to regexp {}", placeholder, rc);
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }
        Ok(0)
    }

    pub fn set_null(&self, placeholder: &str, index: i32) -> PyResult<ejdb2_sys::iwrc> {
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();

        let rc = unsafe { ejdb2_sys::jql_set_null(self.q, placeholder_str.as_ptr(), index) };
        if rc != 0 {
            println!("failed to set placeholder {} to null {}", placeholder, rc);
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }
        Ok(0)
    }

    fn set_placeholder(
        &self,
        py: pyo3::prelude::Python<'_>,
        placeholder: &str,
        index: i32,
        val: &pyo3::types::PyAny,
    ) -> PyResult<ejdb2_sys::iwrc> {
        let placeholder_str = std::ffi::CString::new(placeholder).unwrap();
        let mut rc: ejdb2_sys::iwrc = 0;
        if val.is_instance::<pyo3::types::PyInt>().unwrap() {
            let int_val = val
                .downcast::<pyo3::types::PyInt>()
                .unwrap()
                .to_object(py)
                .extract::<i64>(py)
                .unwrap();

            rc =
                unsafe { ejdb2_sys::jql_set_i64(self.q, placeholder_str.as_ptr(), index, int_val) };
        } else if val.is_instance::<pyo3::types::PyFloat>().unwrap() {
            let float_val = val.downcast::<pyo3::types::PyFloat>().unwrap();

            rc = unsafe {
                ejdb2_sys::jql_set_f64(self.q, placeholder_str.as_ptr(), index, float_val.value())
            };
        } else if val.is_instance::<pyo3::types::PyBool>().unwrap() {
            let bool_val = val.downcast::<pyo3::types::PyBool>().unwrap();

            rc = unsafe {
                ejdb2_sys::jql_set_bool(self.q, placeholder_str.as_ptr(), index, bool_val.is_true())
            };
        } else if val.is_instance::<pyo3::types::PyString>().unwrap() {
            let py_str = val.downcast::<pyo3::types::PyString>().unwrap();
            let value_str = std::ffi::CString::new(py_str.to_str().unwrap()).unwrap();

            rc = unsafe {
                ejdb2_sys::jql_set_str(self.q, placeholder_str.as_ptr(), index, value_str.as_ptr())
            };
        }

        if rc != 0 {
            println!("failed to set placeholder {}", placeholder);
            return Err(pyo3::exceptions::PyIOError::new_err(err_to_str(rc)));
        }
        Ok(0)
    }
}

impl Drop for EJDB {
    fn drop(&mut self) {
        unsafe {
            ejdb2_sys::ejdb_close(&mut self.db);
        }
        self.db = std::ptr::null_mut();
        println!("db closed");
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn ejdb2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EJDB>()?;
    m.add_class::<EJDBQuery>()?;
    Ok(())
}
