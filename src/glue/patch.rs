use super::hell_ffi;

pub struct SurgePatch {
    ptr: *mut hell_ffi::SurgePatch,
}

impl SurgePatch {
    pub fn new() -> Self {
        unsafe {
            let ptr = hell_ffi::create_patch();
            assert!(!ptr.is_null(), "a surge burnt the bridge down (failed to create the patch)."); // the?
            Self { ptr }
        }
    }

    pub fn load_xml(&mut self, data: &[u8], preset: bool) {
        let size = data.len() as i32;
        let data = data.as_ptr() as *const std::ffi::c_void;

        unsafe { (*self.ptr).load_xml(data, size, preset); }
    }
}

impl Drop for SurgePatch {
    fn drop(&mut self) {
        unsafe {
            hell_ffi::destroy_patch(self.ptr);
        }
    }
}
