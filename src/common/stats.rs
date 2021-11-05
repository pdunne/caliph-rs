/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! # Stats Module
//! Provides simple stats formulae

/// Returns the mean of an array of floats
pub fn mean(values: &[f64]) -> f64 {
    let length: usize = values.len();
    if length == 0 {
        return 0_f64;
    } else if length == 1 {
        return values[0];
    }

    values.iter().sum::<f64>() / length as f64
}

/// Returns variance of an array of floats
pub fn variance(values: &[f64]) -> f64 {
    if values.len() == 0 {
        return 0f64;
    }
    let mean = mean(values);

    values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64
}

/// Returns covariance of two input arrays
pub fn covariance(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() {
        panic!("x and y must be of equal length.");
    }

    let length: usize = x.len();

    if length == 0 {
        return 0_f64;
    }

    let mean_x = mean(x);
    let mean_y = mean(y);

    let covariance: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum();

    return covariance / length as f64;
}
