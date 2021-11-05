extern crate common;

use anyhow::Result;
use common::args::CalibArgs;
use common::routines::ph_calibration;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let args = CalibArgs::parse();

    //     let temperature = 21.0;
    //     let ph_measured = [3.75, 9.49];

    let temperature = args.temperature;

    let ph_measured = [args.ph4, args.ph10];

    let calibration = ph_calibration(&ph_measured, &temperature);
    println!("Calib is {:?}", calibration.model);
    println!("RMS is {}", calibration.rms.unwrap());
    println!("R-Squared is {}", calibration.rsq.unwrap());

    if args.store == true {
        println!("Saving to calibration.ph");
        let mut file = File::create("calibration.ph")?;
        write!(file, "{}\t{}\n", calibration.model[0], calibration.model[1])?;
    }

    Ok(())
}
