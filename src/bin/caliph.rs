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

    // println!("RMS\t{:.3e}", calibration.rms.unwrap());
    // println!("R^2\t{:.04}", calibration.rsq.unwrap());
    // writeln!(&mut stdout, "-----------------")?;

    if args.store == true {
        let mut file = File::create("calibration.ph")?;
        write!(file, "{}\t{}\n", calibration.slope, calibration.offset)?;
        println!("\nSaved to calibration.ph\n");
    }

    Ok(())
}
