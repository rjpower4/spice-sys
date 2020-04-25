use libc::{c_char, c_int, c_double};
use std::ffi::CString;

#[link(name="cspice")]
extern {
    fn furnsh_c(file: *const c_char);
    fn unload_c(file: *const c_char);

    fn str2et_c(string: *const c_char, et: *mut c_double);
    fn timout_c(et: c_double, pictur: *const c_char, lenout: c_int, output: *mut c_char);

    fn kclear_c();
    fn ktotal_c (kind: *const c_char, count: *mut c_int);
}


#[cfg(test)]
mod tests {
    use crate::*;
    use std::ffi::{CString};
    use libc::{c_int};

    #[test]
    fn load_unload() {
        let spk_name = CString::new("thirdparty/testkernels/de430.bsp").unwrap();
        let tls_name = CString::new("thirdparty/testkernels/naif0012.tls").unwrap();
        let spk = CString::new("SPK").unwrap();
        let all = CString::new("ALL").unwrap();
        let mut n: c_int = 0;

        unsafe {
            // Load the files
            furnsh_c(spk_name.as_ptr());
            furnsh_c(tls_name.as_ptr());

            // Count the number of SPK Kernels
            ktotal_c(spk.as_ptr(), &mut n);
            assert_eq!(n, 1);

            // Count the number of all kernels
            ktotal_c(all.as_ptr(), &mut n);
            assert_eq!(n, 2);


            // Unload the files
            unload_c(spk_name.as_ptr());
            unload_c(tls_name.as_ptr());

            // Count the number of all kernels after unload
            ktotal_c(all.as_ptr(), &mut n);
            assert_eq!(n, 0);
        }
    }
}
