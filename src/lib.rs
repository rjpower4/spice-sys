use libc::{c_char, c_int, c_double};

#[repr(C)]
pub enum SpiceBooleanC {
    SpiceFalse,
    SpiceTrue
}

#[repr(C)] pub struct SpiceCellC { _private: [u8;0] }

#[link(name="cspice")]
extern "C" {
    /// loads an individual kernel or a collection of kernels.
    pub fn furnsh_c(file: *const c_char);

    /// unloads an individual kernel or a collection of kernels. 
    pub fn unload_c(file: *const c_char);

    /// converts a time string to ET seconds past J2000.
    pub fn str2et_c(string: *const c_char, et: *mut c_double);

    /// converts ET seconds past J2000 to a time string.
    pub fn timout_c(et: c_double, pictur: *const c_char, lenout: c_int, output: *mut c_char);

    /// converts an SCLK string to ET seconds past J2000.
    pub fn scs2e_c(sc: c_int, sclkch: *const c_char, et: *mut c_double);

    /// Clear the KEEPER subsystem: unload all kernels, clear the kernel pool, and
    /// re-initialize the subsystem.
    pub fn kclear_c();

    /// Return the current number of kernels that have been loaded via the KEEPER
    /// interface that are of a specified type. 
    pub fn ktotal_c (kind: *const c_char, count: *mut c_int);

    /// converts ET seconds past J2000 to SCLK string. 
    pub fn sce2s_c(sc: c_int, et: c_double, lenout: c_int, sclkch: *mut c_char);

    /// converts an encoded SCLK to ET seconds past J2000. 
    pub fn sct2e_c(sc: c_int, sclkdp: c_double, et: *mut c_double);

    /// converts ET seconds past J2000 to encoded SCLK
    pub fn sce2c_c(sc: c_int, et: c_double, sclkdp: *mut c_double);

    /// converts an SCLK string to encoded SCLK.
    pub fn scencd_c(sc: c_int, sclkch: *const c_char, sclkdp: *mut c_double);

    /// converts an encoded SCLK to SCLK string.
    pub fn scdecd_c(sc: c_int, sclkdp: c_double, lenout: c_int, sclkch: *mut c_char);

    /// determines whether values exist for some item for a body in the kernel pool.
    pub fn bodfnd_c(body: c_int, item: *const c_char) -> SpiceBooleanC;

    /// retrieves from the kernel pool the values of an item associated with a body.
    pub fn bodvrd_c(bodynm: *const c_char, item: *const c_char, maxn: c_int, dim: *mut c_int, values: *mut c_double);

    /// returns the 3x3 matrix rotating a position vector one frame to another.
    pub fn pxform_c(from: *const c_char, to: *const c_char, et: c_double, rotate: *mut *mut c_double);

    /// returns the 6x6 matrix rotating a state vector from one frame to another.
    pub fn sxform_c(from: *const c_char, to: *const c_char, et: c_double, xform: *mut *mut c_double);

    /// finds the set of reference frame class ID codes of all frames in a binary PCK file.
    pub fn pckfrm_c(pck: *const c_char, ids: *mut SpiceCellC);

    /// finds the coverage window for a reference frame in a binary PCK file. 
    pub fn pckcov_c(pck: *const c_char, idcode: c_int, cover: *mut SpiceCellC);

    /// returns the 3x3 matrix rotating a position vector from one frame at a
    /// specified epoch to another frame at a different epoch.
    pub fn pxfrm2_c(from: *const c_char, to: *const c_char, etfrom: c_double, ett0: c_double, rotate: *mut *mut c_double);
}
