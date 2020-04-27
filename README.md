# spice-sys
[![crate](https://img.shields.io/crates/v/spice-sys.svg)](https://crates.io/crates/spice-sys)
[![documentation](https://docs.rs/spice-sys/badge.svg)](https://docs.rs/spice-sys)
![Rust](https://github.com/rjpower4/spice-sys/workflows/Rust/badge.svg)

Rust bindings for the [NAIF spice toolkit](https://naif.jpl.nasa.gov/naif/toolkit.html).
This crate serves as a thin wrapper around the C implementation of the spice toolkit. 
Currently, it provides only the most common API calls as outlined [here](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/info/mostused.html). 
