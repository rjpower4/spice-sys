use libc::{c_char, c_double, c_float, c_int, c_long, c_short, c_void, uintptr_t};

// Definitions corresponding to the defs in CSPICE
type SpiceLong = c_long;
type SpiceInt = c_int; // this may be wrong for windows...
type SpiceShort = c_short;
type SpiceChar = c_char;
type SpiceDouble = c_double;
type SpiceFloat = c_float;
type SpiceBoolean = c_int;

#[repr(C)]
pub struct SpiceCellC {
    _private: [u8; 0],
}

// not that the field names after fwdptr have been changed
// by adding an '_' to avoid `isize` as it's a keyword.
#[repr(C)]
pub struct SpiceDLADescrC {
    bwdptr: SpiceInt,
    fwdptr: SpiceInt,
    i_base: SpiceInt,
    i_size: SpiceInt,
    d_base: SpiceInt,
    d_size: SpiceInt,
    c_base: SpiceInt,
    c_size: SpiceInt,
}

/// Number of coordinate system parameters in DSK descriptor
const SPICE_DSK_NSYPAR: uintptr_t  = 10;

/// DSK segment descriptor
#[repr(C)]
pub struct SpiceDSKDescrC {
    surfce: SpiceInt,
    center: SpiceInt,
    dclass: SpiceInt,
    dtype: SpiceInt,
    frmcde: SpiceInt,
    corsys: SpiceInt,
    corpar: [SpiceInt; SPICE_DSK_NSYPAR], // Note this array length is hard-coded in CSPICE
    co1min: SpiceDouble,
    co1max: SpiceDouble,
    co2min: SpiceDouble,
    co2max: SpiceDouble,
    co3min: SpiceDouble,
    co3max: SpiceDouble,
    start: SpiceDouble,
    stop: SpiceDouble,
}



#[link(name = "cspice")]
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
    pub fn ktotal_c(kind: *const c_char, count: *mut c_int);

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
    pub fn bodfnd_c(body: c_int, item: *const c_char) -> SpiceBoolean;

    /// retrieves from the kernel pool the values of an item associated with a body.
    pub fn bodvrd_c(
        bodynm: *const c_char,
        item: *const c_char,
        maxn: c_int,
        dim: *mut c_int,
        values: *mut c_double,
    );

    /// returns the 3x3 matrix rotating a position vector one frame to another.
    pub fn pxform_c(
        from: *const c_char,
        to: *const c_char,
        et: c_double,
        rotate: *mut *mut c_double,
    );

    /// returns the 6x6 matrix rotating a state vector from one frame to another.
    pub fn sxform_c(
        from: *const c_char,
        to: *const c_char,
        et: c_double,
        xform: *mut *mut c_double,
    );

    /// finds the set of reference frame class ID codes of all frames in a binary PCK file.
    pub fn pckfrm_c(pck: *const c_char, ids: *mut SpiceCellC);

    /// finds the coverage window for a reference frame in a binary PCK file.
    pub fn pckcov_c(pck: *const c_char, idcode: c_int, cover: *mut SpiceCellC);

    /// returns the 3x3 matrix rotating a position vector from one frame at a
    /// specified epoch to another frame at a different epoch.
    pub fn pxfrm2_c(
        from: *const c_char,
        to: *const c_char,
        etfrom: c_double,
        ett0: c_double,
        rotate: *mut *mut c_double,
    );

    /// returns the state of a target body relative to an observing body.
    pub fn spkezr_c(
        targ: *const c_char,
        et: c_double,
        reff: *const c_char,
        abcorr: *const c_char,
        obs: *const c_char,
        starg: *mut c_double,
        lt: *mut c_double,
    );

    /// returns the position of a target body relative to an observing body.
    pub fn spkpos_c(
        targ: *const c_char,
        et: c_double,
        reff: *const c_char,
        abcorr: *const c_char,
        obs: *const c_char,
        ptarg: *mut c_double,
        lt: *mut c_double,
    );

    /// returns the state of a target body relative to a constant-position observer location.
    pub fn spkcpo_c(
        target: *const c_char,
        et: c_double,
        outref: *const c_char,
        refloc: *const c_char,
        abcorr: *const c_char,
        obspos: *const c_double,
        obsctr: *const c_char,
        obsref: *const c_char,
        state: *mut c_double,
        lt: *mut c_double,
    );

    /// returns the state of a constant-position target location relative to an observing body.
    pub fn spkcpt_c(
        trgpos: *const c_double,
        trgctr: *const c_char,
        trgref: *const c_char,
        et: c_double,
        outref: *const c_char,
        refloc: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        state: *mut c_double,
        lt: *mut c_double,
    );

    /// returns the state of a target body relative to a constant-velocity observer location.
    pub fn spkcvo_c(
        target: *const c_char,
        et: c_double,
        outref: *const c_char,
        refloc: *const c_char,
        abcorr: *const c_char,
        obssta: *const c_double,
        obsepc: c_double,
        obsctr: *const c_char,
        obsref: *const c_char,
        state: *mut c_double,
        lt: *mut c_double,
    );

    /// returns the state of a constant-velocity target location relative to an observing body.
    pub fn spkcvt_c(
        trgsta: *const c_double,
        trgepc: c_double,
        trgctr: *const c_char,
        trgref: *const c_char,
        et: c_double,
        outref: *const c_char,
        refloc: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        state: *mut c_double,
        lt: *mut c_double,
    );

    /// finds the set of ID codes for all objects in a specified SPK file.
    pub fn spkobj_c(spk: *const c_char, ids: *mut SpiceCellC);

    /// finds the coverage window for a specified object in a specified SPK file.
    pub fn spkcov_c(spk: *const c_char, idcode: c_int, cover: *mut SpiceCellC);

    /// finds the set of ID codes for all objects in a specified CK file.
    pub fn ckobj_c(ck: *const c_char, ids: *mut SpiceCellC);

    /// finds the coverage window for a specified object in a specified CK file.
    pub fn ckcov_c(
        ck: *const c_char,
        idcode: c_int,
        needav: SpiceBoolean,
        level: *const c_char,
        tol: c_double,
        timsys: *const c_char,
        cover: *mut SpiceCellC,
    );

    /// gets pointing for a specified CK ID at a specified SCLK time.
    pub fn ckgp_c(
        inst: c_int,
        sclkdp: c_double,
        tol: c_double,
        reff: *const c_char,
        cmat: *const *const c_double,
        clkout: *mut c_double,
        founnd: *mut SpiceBoolean,
    );

    /// gets pointing and angular velocity for a specified CK ID at a specified SCLK time.
    pub fn ckgpav_c(
        inst: c_int,
        sclkdp: c_double,
        tol: c_double,
        reff: *const c_char,
        cmat: *mut *mut c_double,
        av: *mut c_double,
        clkout: *mut c_double,
        found: *mut SpiceBoolean,
    );

    /// returns the field-of-view (FOV) configuration for a specified instrument.
    pub fn getfov_c(
        instid: c_int,
        room: c_int,
        shapelen: c_int,
        framelen: c_int,
        shape: *mut c_char,
        frame: *mut c_char,
        bsight: *mut c_double,
        n: *mut c_int,
        bounds: *mut *mut c_double,
    );

    /// returns the double precision value of a kernel variable from the kernel pool.
    pub fn gdpool_c(
        name: *const c_char,
        start: c_int,
        room: c_int,
        n: *mut c_int,
        values: *mut c_double,
        found: *mut SpiceBoolean,
    );

    /// returns the integer value of a kernel variable from the kernel pool.
    pub fn gipool_c(
        name: *const c_char,
        start: c_int,
        room: c_int,
        n: *mut c_int,
        ivals: *mut c_int,
        found: *mut SpiceBoolean,
    );

    /// returns the character value of a kernel variable from the kernel pool.
    pub fn gcpool_c(
        name: *const c_char,
        start: c_int,
        room: c_int,
        n: *mut c_int,
        cvals: *mut c_void,
        found: *mut SpiceBoolean,
    );

    /// maps an array of planetocentric longitude/latitude coordinate pairs to
    /// surface points on a body, modeled as an ellipsoid or a digital shape (DSK).
    pub fn latsrf_c(
        method: *const c_char,
        target: *const c_char,
        et: c_double,
        fixref: *const c_char,
        npts: c_int,
        lonlat: *const *const c_double,
        srfpts: *mut *mut c_double,
    );

    /// maps an array of surface points on a body, modeled as an ellipsoid or a
    /// digital shape (DSK), to the corresponding outward surface normal vectors.
    pub fn srfnrm_c(
        method: *const c_char,
        target: *const c_char,
        et: c_double,
        fixref: *const c_char,
        npts: c_int,
        srfpts: *const *const c_double,
        normls: *mut *mut c_double,
    );

    /// returns plate model size parameters (plate count and vertex count) for a
    /// type 2 DSK segment.
    pub fn dskz02_c(handle: c_int, dladsc: *const SpiceDLADescrC, nv: *mut c_int, np: *mut c_int);

    /// returns triangular plates from a type 2 DSK segment.
    pub fn dskp02_c(
        handle: c_int,
        dladsc: *const SpiceDLADescrC,
        start: c_int,
        room: c_int,
        n: *mut c_int,
        plates: *mut *mut c_int,
    );

    /// returns vertices from a type 2 DSK segment.
    pub fn dskv02_c(
        handle: c_int,
        dladsc: *const SpiceDLADescrC,
        start: c_int,
        room: c_int,
        n: *mut c_int,
        vrtces: *mut *mut c_double,
    );

    /// returns the set of body ID codes of all objects for which data are
    /// provided in a DSK file.
    pub fn dskobj_c(dsk: *const c_char, bodids: *mut SpiceCellC);

    /// returns the set of surface ID codes for all surfaces associated with a body
    ///in a DSK file.
    pub fn dsksrf_c(dsk: *const c_char, bodyid: c_int, srfids: *mut SpiceCellC);

    /// translates the NAIF integer code of a body into a common name for that body.
    pub fn bodc2n_c(code: c_int, lenout: c_int, name: *mut c_char, found: *mut SpiceBoolean);

    /// translates the name of a body or object to the corresponding NAIF integer ID code.
    pub fn bodn2c_c(name: *const c_char, code: *mut c_int, found: *mut SpiceBoolean);

    /// translates a surface ID code, together with a body name, to the corresponding surface name.
    pub fn srfcss_c(
        code: c_int,
        bodstr: *const c_char,
        srflen: c_int,
        srfstr: *mut c_char,
        isname: *mut SpiceBoolean,
    );

    /// translates a surface string, together with a body name, to the corresponding surface ID code.
    pub fn srfs2c_c(
        srfstr: *const c_char,
        bodstr: *const c_char,
        code: *mut c_int,
        found: *mut SpiceBoolean,
    );

    /// translates a surface ID code, together with a body ID code, to the corresponding surface name.
    pub fn srfc2s_c(
        code: c_int,
        bodyid: c_int,
        srflen: c_int,
        srfstr: *mut c_char,
        isname: *mut SpiceBoolean,
    );

    /// translates a surface string, together with a body ID code, to the corresponding surface ID code.
    pub fn srfscc_c(
        srfstr: *const c_char,
        bodyid: c_int,
        code: *mut c_int,
        found: *mut SpiceBoolean,
    );

    /// converts from rectangular to planetocentric coordinates.
    pub fn reclat_c(
        rectan: *const c_double,
        radius: *mut c_double,
        longitude: *mut c_double,
        latitude: *mut c_double,
    );

    /// converts from planetocentric to rectangular coordinates.
    pub fn latrec_c(
        radius: c_double,
        longitude: c_double,
        latitude: c_double,
        rectan: *mut c_double,
    );

    /// converts from planetocentric lat/lon of a surface point on a body to rectangular coordinates.
    pub fn srfrec_c(body: c_int, longitude: c_double, latitude: c_double, rectan: *mut c_double);

    /// converts from rectangular to geodetic coordinates.
    pub fn recgeo_c(
        rectan: *const c_double,
        re: c_double,
        f: c_double,
        lon: *mut c_double,
        lat: *mut c_double,
        alt: *mut c_double,
    );

    /// converts from geodetic to rectangular coordinates.
    pub fn georec_c(
        lon: c_double,
        lat: c_double,
        alt: c_double,
        re: c_double,
        f: c_double,
        rectan: *mut c_double,
    );

    /// converts from rectangular to planetographic coordinates.
    pub fn recpgr_c(
        body: *const c_char,
        rectan: *const c_double,
        re: c_double,
        f: c_double,
        lon: *mut c_double,
        lat: *mut c_double,
        alt: *mut c_double,
    );

    /// converts from planetographic to rectangular coordinates.
    pub fn pgrrec_c(
        body: *const c_char,
        lon: c_double,
        lat: c_double,
        alt: c_double,
        re: c_double,
        f: c_double,
        rectan: *mut c_double,
    );

    /// computes the surface intercept point of the ray on a body, modeled as an
    /// ellipsoid or a digital shape (DSK), at a specified epoch.
    pub fn sincpt_c(
        method: *const c_char,
        target: *const c_char,
        et: c_double,
        fixref: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        dref: *const c_char,
        dvec: *const c_double,
        spoint: *mut c_double,
        trgepc: *mut c_double,
        srfvec: *mut c_double,
        found: *mut SpiceBoolean,
    );

    /// computes ray-surface intercepts for a set of rays, using data provided
    /// by multiple loaded DSK segments.
    pub fn dskxv_c(
        pri: SpiceBoolean,
        target: *const c_char,
        nsurf: c_int,
        srflst: *const c_int,
        et: c_double,
        fixref: *const c_char,
        nrays: c_int,
        vtxarr: *const *const c_double,
        dirarr: *const *const c_double,
        xptarr: *mut *mut c_double,
        fndarr: *mut SpiceBoolean,
    );

    /// computes a ray-surface intercept using data provided by multiple loaded
    /// DSK segments and returns information about the source of the data
    /// defining the surface on which the intercept was found.
    pub fn dskxsi_c(
        pri: SpiceBoolean,
        target: *const c_char,
        nsurf: c_int,
        srflst: *const c_int,
        et: c_double,
        fixref: *const c_char,
        vertex: *const c_double,
        raydir: *const c_double,
        maxd: c_int,
        maxi: c_int,
        xpt: *mut c_double,
        handle: *mut c_int,
        dladsc: *mut SpiceDLADescrC,
        dskdsc: *mut SpiceDSKDescrC,
        dc: *mut c_double,
        ic: *mut c_int,
        found: *mut SpiceBoolean,
    );

    /// computes the sub-observer point on a body, modeled as an ellipsoid or
    /// a digital shape (DSK), at a particular epoch.
    pub fn subpnt_c(
        method: *const c_char,
        target: *const c_char,
        et: c_double,
        fixref: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        spoint: *mut c_double,
        trgepc: *mut c_double,
        srfvec: *mut c_double,
    );

    /// computes the sub-solar point on a body, modeled as an ellipsoid or a
    /// digital shape (DSK), as seen by an observer at a particular epoch.
    pub fn subslr_c(
        method: *const c_char,
        target: *const c_char,
        et: c_double,
        fixref: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        spoint: *mut c_double,
        trgepc: *mut c_double,
        srfvec: *mut c_double,
    );

    /// computes the illumination angles at a specified surface point of a
    /// target body, modeled as an ellipsoid or a digital shape (DSK), as
    /// seen from an observer body, illuminated by the Sun.
    pub fn ilumin_c(
        method: *const c_char,
        target: *const c_char,
        et: c_double,
        fixref: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        spoint: *const c_double,
        trgepc: *mut c_double,
        srfvec: *mut c_double,
        phase: *mut c_double,
        incdnc: *mut c_double,
        emissn: *mut c_double,
    );

    /// computes the illumination angles at a specified surface point of a
    /// target body, modeled as an ellipsoid or a digital shape (DSK), as
    /// seen from an observer body, illuminated by a user specified body.
    pub fn illumg_c(
        method: *const c_char,
        target: *const c_char,
        ilusrc: *const c_char,
        et: c_double,
        fixref: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        spoint: *const c_double,
        trgepc: *mut c_double,
        srfvec: *mut c_double,
        phase: *mut c_double,
        incdnc: *mut c_double,
        emissn: *mut c_double,
    );

    /// computes the illumination angles at a specified surface point of a
    /// target body, modeled as an ellipsoid or a digital shape (DSK), as
    /// seen from an observer body, illuminated by a user specified body,
    /// with flags indicating whether the point is visible from the observer
    /// and whether it is illuminated.
    pub fn illumf_c(
        method: *const c_char,
        target: *const c_char,
        ilusrc: *const c_char,
        et: c_double,
        fixref: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        spoint: *const c_double,
        trgepc: *mut c_double,
        srfvec: *mut c_double,
        phase: *mut c_double,
        incdnc: *mut c_double,
        visibl: *mut SpiceBoolean,
        lit: *mut SpiceBoolean,
    );

    /// computes the apparent phase angle between the centers of target,
    /// observer, and illuminator ephemeris objects.
    pub fn phaseq_c(
        et: c_double,
        target: *const c_char,
        illmn: *const c_char,
        obsrvr: *const c_char,
        abcorr: *const c_char,
    ) -> c_double;

    /// computes limb points on a body, modeled as an ellipsoid or a
    /// digital shape (DSK).
    pub fn limbpt_c(
        method: *const c_char,
        target: *const c_char,
        et: c_double,
        fixref: *const c_char,
        abcorr: *const c_char,
        corloc: *const c_char,
        obsrvr: *const c_char,
        refvec: *const c_double,
        rolstp: c_double,
        ncuts: c_int,
        schstp: c_double,
        soltol: c_double,
        maxn: c_int,
        npts: *mut c_int,
        points: *mut *mut c_double,
        epochs: *mut c_double,
        tangts: *mut *mut c_double,
    );

    /// computes umbral or penumbral terminator points on a body, modeled as
    /// an ellipsoid or a digital shape (DSK).
    pub fn termpt_c(
        method: *const c_char,
        ilusrc: *const c_char,
        target: *const c_char,
        et: c_double,
        fixref: *const c_char,
        abcorr: *const c_char,
        corloc: *const c_char,
        obsrvr: *const c_char,
        refvec: *const c_double,
        rolstp: c_double,
        ncuts: c_int,
        schstp: c_double,
        soltol: c_double,
        maxn: c_int,
        npts: *mut c_int,
        points: *mut *mut c_double,
        epochs: *mut c_double,
        trmvcs: *mut *mut c_double,
    );

    /// determines the state of an orbiting body from a set orbital elements.
    pub fn conics_c(elts: *const c_double, et: c_double, state: *mut c_double);

    /// determines the set of orbital elements corresponding to the state of a
    /// body.
    pub fn oscelt_c(state: *const c_double, et: c_double, mu: c_double, elts: *mut c_double);

    /// determines if a specified ray is within the FOV of a specified
    /// instrument at a given time.
    pub fn fovray_c(
        inst: *const c_char,
        raydir: *const c_double,
        rframe: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        et: *const c_double,
        visible: *mut SpiceBoolean,
    );

    /// determines if a specified ephemeris object is within the FOV of a
    /// specified instrument at a given time.
    pub fn fovtrg_c(
        inst: *const c_char,
        target: *const c_char,
        tshape: *const c_char,
        tframe: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        et: *const c_double,
        visible: *mut SpiceBoolean,
    );

    /// determines the occultation condition (not occulted, partially,
    /// etc.) of one target relative to another target as seen by an observer at
    /// a given time, with targets modeled as points, ellipsoids, or digital
    /// shapes (DSK).
    pub fn occult_c(
        targ1: *const c_char,
        shape1: *const c_char,
        frame1: *const c_char,
        targ2: *const c_char,
        shape2: *const c_char,
        frame2: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        et: c_double,
        ocltid: *mut c_int,
    );

    /// determines time intervals when a specified constraint on observer-target
    /// distance is met.
    pub fn gfdist_c(
        target: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        relate: *const c_char,
        refval: c_double,
        adjust: c_double,
        step: c_double,
        nintvls: c_int,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when a specified constraint on the observed
    /// phase, solar incidence, or emission angle at a surface point is met.
    pub fn gfilum_c(
        method: *const c_char,
        angtyp: *const c_char,
        target: *const c_char,
        illmn: *const c_char,
        fixref: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        spoint: *const c_double,
        relate: *const c_char,
        refval: c_double,
        adjust: c_double,
        step: c_double,
        nintvls: c_int,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when a specified constraint on the phase angle
    /// between the illuminator, target, and observer body centers is met.
    pub fn gfpa_c(
        target: *const c_char,
        illmn: *const c_char,
        frame: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        relate: *const c_char,
        refval: c_double,
        adjust: c_double,
        step: c_double,
        nintvls: c_int,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when a coordinate of an observer-target
    /// position vector satisfies a numerical constraint.
    pub fn gfposc_c(
        target: *const c_char,
        frame: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        crdsys: *const c_char,
        coord: *const c_char,
        relate: *const c_char,
        refval: c_double,
        adjust: c_double,
        step: c_double,
        nintvls: c_int,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when a specified constraint on the observer-
    /// target range rate is met.
    pub fn gfrr_c(
        target: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        relate: *const c_char,
        refval: c_double,
        adjust: c_double,
        step: c_double,
        nintvls: c_int,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when the angular separation between the
    /// position vectors of two target bodies relative to an observer satisfies
    /// a numerical relationship.
    pub fn gfsep_c(
        targ1: *const c_char,
        shape1: *const c_char,
        frame1: *const c_char,
        targ2: *const c_char,
        shape2: *const c_char,
        frame2: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        relate: *const c_char,
        refval: c_double,
        adjust: c_double,
        step: c_double,
        nintvls: c_int,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when a coordinate of a ray-surface intercept
    /// position vector satisfies a numerical constraint.
    pub fn gfsntc_c(
        target: *const c_char,
        fixref: *const c_char,
        method: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        dref: *const c_char,
        dvec: *const c_double,
        crdsys: *const c_char,
        coord: *const c_char,
        relate: *const c_char,
        refval: c_double,
        adjust: c_double,
        step: c_double,
        nintvls: c_int,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when a coordinate of a sub-observer point
    /// position vector satisfies a numerical constraint.
    pub fn gfsubc_c(
        target: *const c_char,
        fixref: *const c_char,
        method: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        crdsys: *const c_char,
        coord: *const c_char,
        relate: *const c_char,
        refval: c_double,
        adjust: c_double,
        step: c_double,
        nintvls: c_int,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when a specified ray intersects the space
    /// bounded by the field-of-view (FOV) of a specified instrument.
    pub fn gfrfov_c(
        inst: *const c_char,
        raydir: *const c_double,
        rframe: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        step: c_double,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when a specified ephemeris object intersects
    /// the space bounded by the field-of-view (FOV) of a specified instrument.
    pub fn gftfov_c(
        inst: *const c_char,
        target: *const c_char,
        tshape: *const c_char,
        tframe: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        step: c_double,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// determines time intervals when an observer sees one target occulted by
    /// another, with targets modeled as points, ellipsoids, or digital shapes
    /// (DSK).
    pub fn gfoclt_c(
        occtyp: *const c_char,
        front: *const c_char,
        fshape: *const c_char,
        fframe: *const c_char,
        back: *const c_char,
        bshape: *const c_char,
        bframe: *const c_char,
        abcorr: *const c_char,
        obsrvr: *const c_char,
        step: c_double,
        cnfine: *mut SpiceCellC,
        result: *mut SpiceCellC,
    );

    /// converts from rectangular coordinates to range, right ascension, and
    /// declination.
    pub fn recrad_c(
        rectan: *const c_double,
        range: *mut c_double,
        ra: *mut c_double,
        dec: *mut c_double,
    );

    /// converts from range, right ascension, and declination to rectangular
    /// coordinates.
    pub fn radrec_c(range: c_double, ra: c_double, dec: c_double, rectan: *mut c_double);

    /// converts from rectangular to spherical coordinates.
    pub fn recsph_c(
        rectan: *const c_double,
        r: *mut c_double,
        colat: *mut c_double,
        lons: *mut c_double,
    );

    /// converts from spherical to rectangular coordinates.
    pub fn sphrec_c(r: c_double, colat: c_double, lons: c_double, rectan: *mut c_double);

    /// converts from rectangular to cylindrical coordinates.
    pub fn reccyl_c(
        rectan: *const c_double,
        r: *mut c_double,
        lon: *mut c_double,
        z: *mut c_double,
    );

    /// converts from cylindrical to rectangular coordinates.
    pub fn cylrec_c(r: c_double, lon: c_double, z: c_double, rectan: *mut c_double);

    /// converts from spherical to cylindrical coordinates.
    pub fn sphcyl_c(
        radius: c_double,
        colat: c_double,
        slon: c_double,
        r: *mut c_double,
        lon: *mut c_double,
        z: *mut c_double,
    );

    /// converts from cylindrical to spherical coordinates.
    pub fn cylsph_c(
        r: c_double,
        lonc: c_double,
        z: c_double,
        radius: *mut c_double,
        colat: *mut c_double,
        lon: *mut c_double,
    );

    /// converts from spherical to latitudinal coordinates.
    pub fn sphlat_c(
        r: c_double,
        colat: c_double,
        lons: c_double,
        radius: *mut c_double,
        lon: *mut c_double,
        lat: *mut c_double,
    );

    /// converts from latitudinal to spherical coordinates.
    pub fn latsph_c(
        radius: c_double,
        lon: c_double,
        lat: c_double,
        rho: *mut c_double,
        colat: *mut c_double,
        lons: *mut c_double,
    );

    /// converts from cylindrical to latitudinal coordinates.
    pub fn cyllat_c(
        r: c_double,
        lonc: c_double,
        z: c_double,
        radius: *mut c_double,
        lon: *mut c_double,
        lat: *mut c_double,
    );

    /// converts from latitudinal to cylindrical coordinates.
    pub fn latcyl_c(
        radius: c_double,
        lon: c_double,
        lat: c_double,
        r: *mut c_double,
        lonc: *mut c_double,
        z: *mut c_double,
    );

    /// transforms states between coordinate systems -- rectangular, cylindrical,
    /// latitudinal, spherical, geodetic, and planetographic.
    pub fn xfmsta_c(
        input_state: *const c_double,
        input_coord_sys: *const c_char,
        output_coord_sys: *const c_char,
        body: *const c_char,
        output_state: *mut c_double,
    );

    /// packs three scalar components into a vector.
    pub fn vpack_c(x: c_double, y: c_double, z: c_double, v: *mut c_double);

    /// unpacks three scalar components from a vector.
    pub fn vupack_c(v: *const c_double, x: *mut c_double, y: *mut c_double, z: *mut c_double);

    /// adds two 3D vectors.
    pub fn vadd_c(v1: *const c_double, v2: *const c_double, vout: *mut c_double);

    /// computes the difference between two 3D vectors.
    pub fn vsub_c(v1: *const c_double, v2: *const c_double, vout: *mut c_double);

    /// computes the cross product of two 3D vectors.
    pub fn vcrss_c(v1: *const c_double, v2: *const c_double, vout: *mut c_double);

    /// computes the dot product of two 3D vectors.
    pub fn vdot_c(v1: *const c_double, v2: *const c_double) -> c_double;

    /// returns the relative difference between two 3D vectors
    pub fn vrel_c(v1: *const c_double, v2: *const c_double) -> c_double;

    /// multiplies a scalar and a 3D vector.
    pub fn vscl_c(s: c_double, v1: *const c_double, vout: *mut c_double);

    /// negates a 3D vector.
    pub fn vminus_c(v1: *const c_double, vout: *mut c_double);

    /// makes one 3D vector equal to another.
    pub fn vequ_c(vin: *const c_double, vout: *mut c_double);

    /// indicates whether a 3D vector is the zero vector.
    pub fn vzero_c(v: *const c_double) -> SpiceBoolean;

    /// finds the separation angle between two 3D vectors.
    pub fn vsep_c(v1: *const c_double, v2: *const c_double) -> c_double;

    /// returns the distance between two 3D vectors.
    pub fn vdist_c(v1: *const c_double, v2: *const c_double) -> c_double;

    /// computes the magnitude of a 3D vector.
    pub fn vnorm_c(v1: *const c_double) -> c_double;

    /// finds the unit vector along a 3D vector.
    pub fn vhat_c(v1: *const c_double, vout: *mut c_double);

    /// computes the normalized cross product of two 3D vectors.
    pub fn ucrss_c(v1: *const c_double, v2: *const c_double, vout: *mut c_double);

    /// normalizes a 3D vector and return its magnitude.
    pub fn unorm_c(v1: *const c_double, vout: *mut c_double, vmag: *mut c_double);

    /// finds the component of a 3D vector that is perpendicular to a second 3D
    /// vector.
    pub fn vperp_c(a: *const c_double, b: *const c_double, p: *mut c_double);

    /// finds the projection of one 3D vector onto another 3D vector.
    pub fn vproj_c(a: *const c_double, b: *const c_double, p: *mut c_double);

    /// rotates a 3D vector about a specified axis 3D vector by a specified
    /// angle.
    pub fn vrotv_c(v: *const c_double, axis: *const c_double, theta: c_double, r: *mut c_double);

    /// transform a 3D vector to a new coordinate system rotated by an angle
    /// about X, Y, or Z.
    pub fn rotvec_c(v1: *const c_double, angle: c_double, iaxis: c_int, vout: *mut c_double);

    /// computes the vector linear combination a*v1 + b*v2 of two 3D vectors.
    pub fn vlcom_c(
        a: c_double,
        v1: *const c_double,
        b: c_double,
        v2: *const c_double,
        sum: *mut c_double,
    );

    /// computes the vector linear combination a*v1 + b*v2 + c*v3 of three 3D
    /// vectors.
    pub fn vlcom3_c(
        a: c_double,
        v1: *const c_double,
        b: c_double,
        v2: *const c_double,
        c: c_double,
        v3: *const c_double,
        sum: *mut c_double,
    );

    /// multiplies two 3x3 matrices.
    pub fn mxm_c(m1: *const *const c_double, m2: *const *const c_double, mout: *mut *mut c_double);

    /// multiplies a 3x3 matrix and the transpose of another 3x3 matrix.
    pub fn mxmt_c(m1: *const *const c_double, m2: *const *const c_double, mout: *mut *mut c_double);

    /// multiplies a 3x3 matrix with a 3D vector.
    pub fn mxv_c(m1: *const *const c_double, vin: *const c_double, vout: *mut c_double);

    /// multiplies the transpose of a 3x3 matrix and a 3x3 matrix.
    pub fn mtxm_c(m1: *const *const c_double, m2: *const *const c_double, mout: *mut *mut c_double);

    /// multiplies the transpose of a 3x3 matrix on the left with a 3D vector on
    /// the right.
    pub fn mtxv_c(m1: *const *const c_double, vin: *const c_double, vout: *mut c_double);

    /// multiplies the transpose of a 3D vector, a 3x3 matrix, and a 3D vector.
    pub fn vtmv_c(
        v1: *const c_double,
        matrix: *const *const c_double,
        v2: *const c_double,
    ) -> c_double;

    /// transposes a 3x3 matrix.
    pub fn xpose_c(m1: *const *const c_double, mout: *mut *mut c_double);

    /// sets one 3x3 matrix equal to another.
    pub fn mequ_c(m1: *const *const c_double, mout: *mut *mut c_double);

    /// computes the determinant of a 3x3 matrix.
    pub fn det_c(matrix: *const *const c_double);

    /// returns the trace of a 3x3 matrix.
    pub fn trace_c(matrix: *const *const c_double);

    /// calculates the 3x3 matrix for a rotation of an angle about the X, Y or Z
    /// axis.
    pub fn rotate_c(angle: c_double, iaxis: c_int, mout: *mut *mut c_double);

    /// applies a rotation of an angle about the X, Y, or Z axis to a matrix.
    pub fn rotmat_c(
        m1: *const *const c_double,
        angle: c_double,
        iaxis: c_int,
        mout: *mut *mut c_double,
    );

    /// builds the transformation to a frame based on two vectors.
    pub fn twovec_c(
        axdef: *const c_double,
        indexa: c_int,
        pldef: *const c_double,
        indexp: c_int,
        mout: *mut *mut c_double,
    );

    /// constructs a rotation matrix from a set of Euler angles.
    pub fn eul2m_c(
        angle3: c_double,
        angle2: c_double,
        angle1: c_double,
        axis3: c_int,
        axis2: c_int,
        axis1: c_int,
        r: *mut *mut c_double,
    );

    /// factors a matrix as a product of three rotations about specified axes.
    pub fn m2eul_c(
        r: *const *const c_double,
        axis3: c_int,
        axis2: c_int,
        axis1: c_int,
        angle3: *mut c_double,
        angle2: *mut c_double,
        angle1: *mut c_double,
    );

    /// computes the axis of the rotation given by a matrix and the angle about
    /// that axis.
    pub fn raxisa_c(matrix: *const *const c_double, axis: *mut c_double, angle: *mut c_double);

    /// construct a rotation matrix that rotates vectors by an angle about an
    /// axis.
    pub fn axisar_c(axis: *const c_double, angle: c_double, r: *mut *mut c_double);

    /// finds a unit quaternion corresponding to a specified rotation matrix.
    pub fn m2q_c(r: *const *const c_double, q: *mut c_double);

    /// find the rotation matrix corresponding to a specified unit quaternion.
    pub fn q2m_c(q: *const c_double, r: *mut *mut c_double);

    /// returns half the value of pi.
    pub fn halfpi_c() -> c_double;

    /// returns the value of pi.
    pub fn pi_c() -> c_double;

    /// returns twice the value of pi.
    pub fn twopi_c() -> c_double;

    /// returns the number of degrees per radian.
    pub fn dpr_c() -> c_double;

    /// returns the number of radians per degree.
    pub fn rpd_c() -> c_double;

    /// returns the number of seconds in a day.
    pub fn spd_c() -> c_double;

    /// returns the number of seconds per Julian year.
    pub fn jyear_c() -> c_double;

    /// returns the number of seconds per tropical year.
    pub fn tyear_c() -> c_double;

    /// returns the speed of light in vacuo (km/sec)
    pub fn clight_c() -> c_double;

    /// returns the Julian Date corresponding to Besselian date 1900.0.
    pub fn b1900_c() -> c_double;

    /// returns the Julian Date corresponding to Besselian date 1950.0.
    pub fn b1950_c() -> c_double;

    /// returns the Julian Date of 1899 DEC 31 12:00:00 (1900 JAN 0.5).
    pub fn j1900_c() -> c_double;

    /// returns the Julian Date of 1950 JAN 01 00:00:00 (1950 JAN 1.0).
    pub fn j1950_c() -> c_double;

    /// returns the Julian Date of 2000 JAN 01 12:00:00 (2000 JAN 1.5).
    pub fn j2000_c() -> c_double;

    /// returns the Julian Date of 2100 JAN 01 12:00:00 (2100 JAN 1.5).
    pub fn j2100_c() -> c_double;
}
