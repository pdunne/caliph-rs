/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

use super::fit;
use super::{PH10_STATIC, PH4_STATIC, TEMP_STATIC};
use splines::{Interpolation, Key, Spline};

pub struct Calibration {
    pub model: [f64; 2],
    pub rms: Option<f64>,
    pub rsq: Option<f64>,
}

impl Calibration {
    pub fn new(model: [f64; 2], rms: Option<f64>, rsq: Option<f64>) -> Calibration {
        Calibration { model, rms, rsq }
    }
}

impl Default for Calibration {
    fn default() -> Self {
        Calibration {
            model: [1.0, 0.0],
            rms: None,
            rsq: None,
        }
    }
}

pub fn ph_calibration(ph_measured: &[f64; 2], temperature: &f64) -> Calibration {
    let ph4_cal = interp_ph4(temperature).unwrap();
    let ph10_cal = interp_ph10(temperature).unwrap();

    let ph_cal = [ph4_cal, ph10_cal];

    let calibration = fit::fit(ph_measured, &ph_cal);
    let fit_eval = fit::evaluate(ph_measured, &ph_cal, &calibration);

    Calibration::new(calibration, Some(fit_eval[0]), Some(fit_eval[1]))
}

pub fn ph_convert(ph_measured: &f64, calibration: &[f64; 2]) -> f64 {
    let ph_calibrated = fit::predict(ph_measured, calibration);
    ph_calibrated
}

pub fn interp_ph4(temperature: &f64) -> Option<f64> {
    let pairs_iter = TEMP_STATIC.iter().zip(PH4_STATIC.iter());
    let zipped_points: Vec<_> = pairs_iter
        .map(|(x, y)| Key::new(*x, *y, Interpolation::Linear))
        .collect();

    let spline = Spline::from_vec(zipped_points);

    let val = spline.sample(*temperature);
    val
}

pub fn interp_ph10(temperature: &f64) -> Option<f64> {
    let pairs_iter = TEMP_STATIC.iter().zip(PH10_STATIC.iter());
    let zipped_points: Vec<_> = pairs_iter
        .map(|(x, y)| Key::new(*x, *y, Interpolation::Linear))
        .collect();

    let spline = Spline::from_vec(zipped_points);

    let val = spline.sample(*temperature);
    val
}
