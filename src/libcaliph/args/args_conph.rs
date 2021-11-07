/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

///! Read in command line arguments for `conph` using clap
use clap::{App, Arg};

/// Command line arguments struct, infile, outfile, and silent (i.e. emit to stdout)
pub struct ConvArgs {
    /// pH measured
    pub ph: f64,
    /// Give custom calibration values insted of reading calibration.ph
    pub custom: bool,

    pub calibration: Option<[f64; 2]>,
}

impl ConvArgs {
    /// Parse command line arguments
    pub fn parse() -> Self {
        let matches = App::new("conph")
            .author("Peter Dunne")
            .version("0.1.3")
            .about("Corrects pH measurement with calibration")
            .arg(
                Arg::with_name("ph")
                    .help("pH measured")
                    .index(1)
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("temperature")
                    .help("Temperature of measurement")
                    .short("t")
                    .long("temperature")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("custom")
                    .short("c")
                    .long("custom")
                    .help("Custom Input")
                    .requires_all(&["slope", "offset"]),
            )
            .arg(
                Arg::with_name("slope")
                    .short("s")
                    .long("slope")
                    .help("Slope")
                    .takes_value(true)
                    .requires_all(&["custom", "offset"]),
            )
            .arg(
                Arg::with_name("offset")
                    .short("o")
                    .long("offset")
                    .help("Offset")
                    .takes_value(true)
                    .requires_all(&["custom", "slope"]),
            )
            .get_matches();

        let ph = matches
            .value_of("ph")
            .unwrap_or_default()
            .parse::<f64>()
            .unwrap();

        let custom = matches.is_present("custom");

        let calibration = if custom {
            let slope = matches
                .value_of("slope")
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap();

            let offset = matches
                .value_of("offset")
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap();

            Some([slope, offset])
        } else {
            None
        };

        Self {
            ph,
            custom,
            calibration,
        }
    }
}
