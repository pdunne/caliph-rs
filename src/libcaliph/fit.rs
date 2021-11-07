/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! Provides methods to performs linear fit of two input arrays, x and y
use super::stats;

/// Linear regression of x,y data
/// Returns an array of [slope, offset]
/// ```
/// use crate::libcaliph::fit::fit;
/// use float_cmp::approx_eq;
/// let x = [1.0, 2.0, 3.0];
/// let y = [3.0, 5.0, 7.0];
///
/// let result = fit(&x, &y);
/// let comparison = [2.0, 1.0];
///
/// assert!(approx_eq!(f64, result[0], comparison[0]) &&  approx_eq!(f64, result[0], comparison[0]) );
/// ```
///
pub fn fit(x: &[f64], y: &[f64]) -> [f64; 2] {
    let slope = stats::covariance(x, y) / stats::variance(x);
    let intercept = stats::mean(y) - slope * stats::mean(x);
    [slope, intercept]
}

/// Gives predicted value using `model` for a given x
/// ```
/// use crate::libcaliph::fit::predict;
/// use float_cmp::approx_eq;
///
/// let model = [2.0, 3.5];
/// let x = 1.5;
/// let result = predict(&x, &model);
/// let comparison = 6.5;
/// assert!(approx_eq!(f64, result, comparison));
///```
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

    (sum_error_iter / length as f64).sqrt()
}

fn rsquared(y: &[f64], rms: &f64) -> f64 {
    1.0 - (rms / stats::variance(y))
}

/// Evaluates all data in a model, returning the root mean squared error (RMSE), and the R-Squared goodness of fit
///
/// ```
/// use crate::libcaliph::fit::{fit,evaluate};
/// use float_cmp::approx_eq;
///
/// let x = [1.05, 1.992, 3.03];
/// let y = [2.993, 4.92, 6.99];
///
/// let model = fit(&x, &y);
/// let result = evaluate(&x, &y, &model);
///
/// let comparison = [1.19675583971723e-2, 0.99550];
/// println!("{:.e}", result[0]);
/// println!("{}", result[1]);
/// assert!(approx_eq!(f64, result[0], comparison[0]) &&  approx_eq!(f64, result[0], comparison[0]) );
///```
pub fn evaluate(x: &[f64], y: &[f64], model: &[f64; 2]) -> [f64; 2] {
    let y_predicted: Vec<f64> = x.iter().map(|y| predict(y, model)).collect();
    let rms = root_mean_squared_error(y, &y_predicted);
    [rms, rsquared(y, &rms)]
}
