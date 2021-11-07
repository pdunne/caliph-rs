/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! Command line tool to correct a pH measurement using a calibration model.
//!
//! Examples:
//!
//! Assuming the `calibration.ph` file exists:
//! ```console
//! conph 3.5
//! ```
//!
//! Custom calibration settings fof the slope and offset:
//! `-c` sets it to custom, `-s VAL` is for the slope, `-o VAL` is for the offset
//! ```console
//! conph 3.5 0 -c -s 1.1 -o 0.02
//! ```
//!
//! Boolean flat to save the calibration to `calibration.ph` in the current directory:
//! ```console
//! caliph 3.97 10.2 -t 22.3 -s
//! ```
extern crate common;
use anyhow::Result;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

use common::args::ConvArgs;
use common::routines::ph_convert;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    // Parse CLI
    let args = ConvArgs::parse();

    let ph_measured = args.ph;
    let calibration = if args.custom == false {
        let file = File::open("calibration.ph")?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        // Fragile, assumes two floats separated by a tab/whitespace
        let cal_vec = contents
            .split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok())
            .collect::<Vec<_>>();
        [cal_vec[0], cal_vec[1]]
    } else {
        args.calibration.unwrap()
    };

    let ph_correct = ph_convert(&ph_measured, &calibration);

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    writeln!(&mut stdout, "\n---------------")?;
    stdout.set_color(ColorSpec::new().set_bold(true))?;
    writeln!(&mut stdout, "  Converting")?;
    stdout.reset()?;
    writeln!(&mut stdout, "---------------")?;

    stdout.set_color(ColorSpec::new().set_bold(true))?;
    writeln!(&mut stdout, "Input\t{}", ph_measured)?;
    writeln!(&mut stdout, "Output\t{:.4}", ph_correct)?;
    stdout.reset()?;
    writeln!(&mut stdout, "---------------\n")?;

    Ok(())
}
