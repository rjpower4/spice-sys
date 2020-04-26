use spice_sys::*;
use std::ffi::CString;
use std::path::PathBuf;

#[test]
fn test_load_unload_kernel() {
    let spk_path: PathBuf = ["thirdparty", "testkernels", "de430.bsp"].iter().collect();
    let lsk_path: PathBuf = ["thirdparty", "testkernels", "naif0012.lsk"]
        .iter()
        .collect();
    let spk_kern_name = CString::new(spk_path.as_path().to_str().unwrap()).unwrap();
    let lsk_kern_name = CString::new(lsk_path.as_path().to_str().unwrap()).unwrap();
    let kern_type_all = CString::new("all").unwrap();
    let mut k = 0;

    unsafe {
        // Load the SPK
        furnsh_c(spk_kern_name.as_ptr());

        // Ensure that it was loaded
        ktotal_c(kern_type_all.as_ptr(), &mut k);
        assert_eq!(k, 1);

        // Load the LSK
        furnsh_c(lsk_kern_name.as_ptr());

        // Ensure that it was loaded
        ktotal_c(kern_type_all.as_ptr(), &mut k);
        assert_eq!(k, 2);

        // Unload the SPK
        unload_c(spk_kern_name.as_ptr());

        // Ensure that it was unloaded
        ktotal_c(kern_type_all.as_ptr(), &mut k);
        assert_eq!(k, 1);

        // Unload the LSK
        unload_c(lsk_kern_name.as_ptr());

        // Ensure that it was unloaded
        ktotal_c(kern_type_all.as_ptr(), &mut k);
        assert_eq!(k, 0);
    }
}
