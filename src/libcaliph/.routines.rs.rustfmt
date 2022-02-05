/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # Routines Module
//! Provides the functions needed to calibrate a pH meter, and to perform the conversion of a measurement with a known calibration.

use super::fit;
use super::{PH10_STATIC, PH4_STATIC, TEMP_STATIC};
use float_cmp::ApproxEq;
use splines::{Interpolation, Key, Spline};

/// Calibration struct as a convenience wrapper.
///
/// This includes optional elements for goodness of fit variables. The calibration model is linear, i.e. $`y  = m x + c`$
pub struct Calibration<F> {
    ///
    pub slope: F,
    pub offset: F,
    pub rms: Option<F>,
    pub rsq: Option<F>,
}

/// Implements ApproxEq trait for Calibration struct
impl<'a, M: Copy + Default, F: Copy + ApproxEq<Margin = M>> ApproxEq for &'a Calibration<F> {
    type Margin = M;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.slope.approx_eq(other.slope, margin) && self.offset.approx_eq(other.offset, margin)
    }
}

impl<F> Calibration<F>
where
    F: Copy,
{
    pub fn new(slope: F, offset: F, rms: Option<F>, rsq: Option<F>) -> Calibration<F> {
        Calibration {
            slope,
            offset,
            rms,
            rsq,
        }
    }
    /// Modifies the slope
    pub fn with_slope(&self, slope: F) -> Calibration<F> {
        Calibration {
            slope,
            offset: self.offset,
            rms: self.rms,
            rsq: self.rsq,
        }
    }

    // Modifies with offset
    pub fn with_offset(&self, offset: F) -> Calibration<F> {
        Calibration {
            slope: self.slope,
            offset,
            rms: self.rms,
            rsq: self.rsq,
        }
    }
}

impl<F> Default for Calibration<F>
where
    F: Default,
{
    fn default() -> Self {
        Calibration {
            slope: F::default(),
            offset: F::default(),
            rms: None,
            rsq: None,
        }
    }
}

/// Calculates the calibration values at give temperature for the measured pH values
pub fn ph_calibration(ph_measured: &[f64; 2], temperature: &f64) -> Calibration<f64> {
    let ph4_cal = interp_ph4(temperature).unwrap_or(4.01);
    let ph10_cal = interp_ph10(temperature).unwrap_or(10.01);

    let ph_cal = [ph4_cal, ph10_cal];

    let calibration = fit::fit(ph_measured, &ph_cal);
    let fit_eval = fit::evaluate(ph_measured, &ph_cal, &calibration);

    Calibration::new(
        calibration[0],
        calibration[1],
        Some(fit_eval[0]),
        Some(fit_eval[1]),
    )
}

/// Converts the measured pH to a calibrated one using a known calibration
pub fn ph_convert(ph_measured: &f64, calibration: &[f64; 2]) -> f64 {
    fit::predict(ph_measured, calibration)
}

/// Interpolates the temperature dependence of a pH 4.01 buffer solution to give an arbitrary pH value between 5 to 95˚C
pub fn interp_ph4(temperature: &f64) -> Option<f64> {
    let pairs_iter = TEMP_STATIC.iter().zip(PH4_STATIC.iter());
    let zipped_points: Vec<_> = pairs_iter
        .map(|(x, y)| Key::new(*x, *y, Interpolation::Linear))
        .collect();

    let spline = Spline::from_vec(zipped_points);

    spline.sample(*temperature)
}

/// Interpolates the temperature dependence of a pH 10.01 buffer solution to give an arbitrary pH value between 5 to 95˚C
pub fn interp_ph10(temperature: &f64) -> Option<f64> {
    let pairs_iter = TEMP_STATIC.iter().zip(PH10_STATIC.iter());
    let zipped_points: Vec<_> = pairs_iter
        .map(|(x, y)| Key::new(*x, *y, Interpolation::Linear))
        .collect();

    let spline = Spline::from_vec(zipped_points);

    spline.sample(*temperature)
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::routines::Calibration;

    use super::ph_calibration;

    #[test]
    fn test_ph_calibration() {
        let temperature = 21.0;
        let ph_measured = [3.75, 9.49];
        let res = ph_calibration(&ph_measured, &temperature);
        let slope = 1.053658536585366;
        let offset = 0.05078048780487787;
        let test_calib = Calibration::default().with_slope(slope).with_offset(offset);

        assert!(approx_eq!(&Calibration<f64>, &res, &test_calib))
    }
}
