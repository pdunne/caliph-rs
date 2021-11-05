//!
//!
//!
//!
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
