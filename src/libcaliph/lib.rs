/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! A simple tool to calibrate and convert pH measurements using a two point method.
//! This module contains all the routines necessary to calculate the correction factors needed to calibrate a pH meter.
//!
//!## About
//!
//!This project contains two binaries:
//!
//!`caliph`, for calibrating a pH electrode using a two point pH method.
//!
//!`conph` for converting measured pH values to calibrated ones.
//!
//!<!-- ## How it works
//!
//!Text here -->
//!
//!## Demo
//!
//!Once installed (see below), the two methods available are:
//!
//!### Calibration
//!
//!When the temperature is 25˚C during the measurement:
//!
//!```console
//!$ caliph 3.97 10.2
//!
//!-----------------
//!  Calibrating
//!-----------------
//!Slope   0.96308
//!Offset  0.18657
//!-----------------
//!
//!```
//!
//!Optional temperature argument:
//!
//!```console
//!$ caliph 3.97 10.2 -t 22.3
//!
//!-----------------
//!  Calibrating
//!-----------------
//!Slope   0.96828
//!Offset  0.16052
//!-----------------
//!```
//!
//!Boolean flat to save the calibration to `calibration.ph` in the current directory:
//!
//!```console
//!$ caliph 3.97 10.2 -t 22.3 -s
//!
//!-----------------
//!  Calibrating
//!-----------------
//!Slope   0.96828
//!Offset  0.16052
//!-----------------
//!
//!Saved to calibration.ph
//!```
//!
//!## Conversion
//!
//!Assuming the `calibration.ph` file exists:
//!
//!```console
//!$ conph 3.5
//!
//!---------------
//!  Converting
//!---------------
//!Input   3.5
//!Output  3.5495
//!---------------
//!
//!```
//!
//!Custom calibration settings fof the slope and offset:
//!
//!`-c` sets it to custom, `-s VAL` is for the slope, `-o VAL` is for the offset
//!
//!```console
//!$ conph 3.5 -c -s 1.1 -o 0.02
//!
//!---------------
//!  Converting
//!---------------
//!Input   3.5
//!Output  3.8700
//!---------------
//!
//!```
//!
//!## Installing
//!
//!The latest version of can be installed or updated with `cargo install`:
//!
//!```sh
//!cargo install caliph
//!```
//!
//!or
//!
//!```sh
//!cargo install  --git https://github.com/pdunne/caliph-rs
//!```
//!
//!Binary releases will also be made available on the github page.
//!
//!## Compiling
//!
//!Follow these instructions to compile `cargo-outdated`, then skip down to Installation.
//!
//! 1. Ensure you have current version of `cargo` and [Rust](https://www.rust-lang.org) installed
//! 2. Clone the project `$ git clone https://github.com/kbknapp/cargo-outdated && cd cargo-outdated`
//! 3. Build the project `$ cargo build --release`
//! 4. Once complete, the binary will be located at `target/release/cargo-outdated`
//!
//!### Options
//!
//!For `caliph`:
//!
//!```text
//!caliph 0.1.3
//!Peter Dunne
//!Calculates corrections from 2 point pH calibration
//!
//!USAGE:
//!    caliph [FLAGS] [OPTIONS] <ph4> <ph10>
//!
//!FLAGS:
//!    -h, --help       Prints help information
//!    -s, --store      Store calibration to file calib.ph
//!    -V, --version    Prints version information
//!
//!OPTIONS:
//!    -t, --temperature <temperature>    temperature of measurement
//!
//!ARGS:
//!    <ph4>     pH measured for pH 4.01 buffer solution
//!    <ph10>    pH measured for pH 10.01 buffer solution
//!```
//!
//!and for `conph`
//!
//!```text
//!conph 0.1.3
//!Peter Dunne
//!Corrects pH measurement with calibration
//!
//!USAGE:
//!    conph [FLAGS] [OPTIONS] <ph>
//!
//!FLAGS:
//!    -c, --custom     Custom Input
//!    -h, --help       Prints help information
//!    -V, --version    Prints version information
//!
//!OPTIONS:
//!    -o, --offset <offset>              Offset
//!    -s, --slope <slope>                Slope
//!    -t, --temperature <temperature>    Temperature of measurement
//!
//!ARGS:
//!    <ph>    pH measured
//!```
//!
//!## License
//!
//!`calpih-rs` is released under the terms of the Mozilla Public
//!License, v. 2.0. See the LICENSE.
//!
//! For the `caliph` calibration binary, the two imports needed in its main function
//!  are:
//! ```
//! use libcaliph::args::CalibArgs;
//! use libcaliph::routines::ph_calibration;
//! ```
//!
//! While for `conph` the pH converter, the two imports needed in it's main function are:
//! ```
//! use libcaliph::args::CalibArgs;
//! use libcaliph::routines::ph_convert;
//! ```
pub mod args;
pub mod fit;
pub mod routines;
pub mod stats;

/// Temperature points for pH buffer solutions dependent curves.
///
/// These are kept in the stack for the lifetime of the program
static TEMP_STATIC: [f64; 20] = [
    0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0,
    80.0, 85.0, 90.0, 95.0,
];

/// 4.01 pH buffer solutions temperature dependence
///
/// This is in the stack for the lifetime of the program
static PH4_STATIC: [f64; 20] = [
    4.01, 4., 4., 4., 4., 4.01, 4.02, 4.03, 4.04, 4.05, 4.06, 4.07, 4.09, 4.11, 4.12, 4.14, 4.16,
    4.17, 4.19, 4.2,
];

/// 4.01 pH buffer solutions temperature dependence
///
/// This is in the stack for the lifetime of the program
static PH10_STATIC: [f64; 20] = [
    10.32, 10.25, 10.18, 10.12, 10.06, 10.01, 9.96, 9.92, 9.88, 9.85, 9.82, 9.79, 9.77, 9.76, 9.75,
    9.74, 9.73, 9.74, 9.75, 9.76,
];
