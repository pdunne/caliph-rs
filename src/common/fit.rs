/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

///! # Linear Regression
///! Performs linear fit of two input arrays, x and y
use super::stats;

/// Linear regression of x,y data
pub fn fit(x: &[f64], y: &[f64]) -> [f64; 2] {
    let slope = stats::covariance(x, y) / stats::variance(x);
    let intercept = stats::mean(y) - slope * stats::mean(x);
    [slope, intercept]
}

/// Gives predicted value using `model` for a given x
pub fn predict(x: &f64, model: &[f64; 2]) -> f64 {
    x * model[0] + model[1]
}

/// Calculates RMS for a model
fn root_mean_squared_error(actual: &[f64], predicted: &[f64]) -> f64 {
    let length = actual.len();

    let sum_error_iter: f64 = predicted
        .iter()
        .zip(actual.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum();

    let mse = (sum_error_iter / length as f64).sqrt();
    mse
}

fn rsquared(y: &[f64], rms: &f64) -> f64 {
    1.0 - (rms / stats::variance(y))
}

/// Evaluates all data in a model
pub fn evaluate(x: &[f64], y: &[f64], model: &[f64; 2]) -> [f64; 2] {
    let y_predicted: Vec<f64> = x.iter().map(|y| predict(y, model)).collect();
    let rms = root_mean_squared_error(y, &y_predicted);
    [rms, rsquared(y, &rms)]
}
