/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

///! Read in command line arguments for `caliph` using clap
use clap::{App, Arg};

/// Command line arguments struct, infile, outfile, and silent (i.e. emit to stdout)
pub struct CalibArgs {
    /// pH measured for pH 4.01 buffer solution
    pub ph4: f64,
    /// pH measured for pH 10.01 buffer solution
    pub ph10: f64,
    /// temperature of measurement
    pub temperature: f64,
    /// Store calibration to file calib.ph
    pub store: bool,
}

impl CalibArgs {
    /// Parse command line arguments
    pub fn parse() -> Self {
        let matches = App::new("caliph")
            .author("Peter Dunne")
            .version("0.1.0")
            .about("Calculates 2D magnetic fields")
            .arg(
                Arg::with_name("ph4")
                    .help("pH measured for pH 4.01 buffer solution")
                    .index(1)
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("ph10")
                    .help("pH measured for pH 10.01 buffer solution")
                    .index(2)
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("temperature")
                    .help("temperature of measurement")
                    .short("t")
                    .long("temperature")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("store")
                    .short("s")
                    .long("store")
                    .help("Store calibration to file calib.ph"),
            )
            .get_matches();

        let ph4 = matches
            .value_of("ph4")
            .unwrap_or_default()
            .parse::<f64>()
            .unwrap();
        let ph10 = matches
            .value_of("ph10")
            .unwrap_or_default()
            .parse::<f64>()
            .unwrap();

        let temperature = if matches.is_present("temperature") {
            matches
                .value_of("temperature")
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap()
        } else {
            25.0_f64
        };

        let store = matches.is_present("store");

        Self {
            ph4,
            ph10,
            temperature,
            store,
        }
    }
}
