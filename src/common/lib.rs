/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

pub mod args;
pub mod fit;
pub mod routines;
pub mod stats;

static TEMP_STATIC: [f64; 20] = [
    0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0,
    80.0, 85.0, 90.0, 95.0,
];

static PH4_STATIC: [f64; 20] = [
    4.01, 4., 4., 4., 4., 4.01, 4.02, 4.03, 4.04, 4.05, 4.06, 4.07, 4.09, 4.11, 4.12, 4.14, 4.16,
    4.17, 4.19, 4.2,
];

static PH10_STATIC: [f64; 20] = [
    10.32, 10.25, 10.18, 10.12, 10.06, 10.01, 9.96, 9.92, 9.88, 9.85, 9.82, 9.79, 9.77, 9.76, 9.75,
    9.74, 9.73, 9.74, 9.75, 9.76,
];
