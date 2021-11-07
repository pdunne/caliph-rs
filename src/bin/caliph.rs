/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! Command line tool to calibrate a pH meter
//!
//! When the temperature is 25˚C during the measurement:
//! ```console
//! caliph 3.97 10.2
//! ```
//!
//! Optional temperature argument:
//! ```console
//! caliph 3.97 10.2 -t 22.3
//! ```
//!
//! Boolean flat to save the calibration to `calibration.ph` in the current directory:
//! ```console
//! caliph 3.97 10.2 -t 22.3 -s
//! ```
extern crate common;

use anyhow::Result;
use common::args::CalibArgs;
use common::routines::ph_calibration;
use std::fs::File;
use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> Result<()> {
    let args = CalibArgs::parse();

    let temperature = args.temperature;
    let ph_measured = [args.ph4, args.ph10];
    let calibration = ph_calibration(&ph_measured, &temperature);

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    writeln!(&mut stdout, "\n-----------------")?;
    stdout.set_color(ColorSpec::new().set_bold(true))?;
    writeln!(&mut stdout, "  Calibrating")?;
    stdout.reset()?;
    writeln!(&mut stdout, "-----------------")?;
    stdout.set_color(ColorSpec::new().set_bold(true))?;
    writeln!(&mut stdout, "Slope\t{:.5}", calibration.slope)?;
    println!("Offset\t{:.5}", calibration.offset);
    stdout.reset()?;
    writeln!(&mut stdout, "-----------------")?;

    if args.store == true {
        let mut file = File::create("calibration.ph")?;
        write!(file, "{}\t{}\n", calibration.slope, calibration.offset)?;
        println!("\nSaved to calibration.ph\n");
    }

    Ok(())
}
