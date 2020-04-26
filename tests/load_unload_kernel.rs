use spice_sys::*;
use std::ffi::CString;
use std::path::PathBuf;

#[test]
fn test_load_unload_kernel() {
    let mut spk_path: PathBuf = ["thirdparty", "testkernels", "de430.bsp"].iter().collect();
    let mut tls_path: PathBuf = ["thirdparty", "testkernels", "naif0012.tls"].iter().collect();
    let spk_kern_name = CString::new(spk_path.as_path().to_str().unwrap()).unwrap();
    let tls_kern_name = CString::new(tls_path.as_path().to_str().unwrap()).unwrap();

    unsafe {
        // Load the kernels
        furnsh_c(spk_kern_name.as_ptr());
        furnsh_c(tls_kern_name.as_ptr());

        unload_c(spk_kern_name.as_ptr());
        unload_c(spk_kern_name.as_ptr());
    }
}
