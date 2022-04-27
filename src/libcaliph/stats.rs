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
    if values.is_empty() {
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

    covariance / length as f64
}

#[cfg(test)]
mod tests {
    use super::{covariance, mean, variance};
    use float_cmp::approx_eq;

    #[test]
    fn test_mean() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let result: f64 = mean(&values);

        assert!(approx_eq!(f64, 2.5_f64, result));
    }

    #[test]
    fn test_mean_empty() {
        let values: Vec<f64> = Vec::new();
        let result: f64 = mean(&values);

        assert!(approx_eq!(f64, 0.0_f64, result));
    }

    #[test]
    fn test_mean_single() {
        let values: Vec<f64> = vec![2.0];
        let result: f64 = mean(&values);

        assert!(approx_eq!(f64, 2.0_f64, result));
    }

    #[test]
    fn test_variance() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let result: f64 = variance(&values);

        assert!(approx_eq!(f64, 1.25_f64, result));
    }

    #[test]
    fn test_variance_empty() {
        let values: Vec<f64> = Vec::new();
        let result: f64 = variance(&values);

        assert!(approx_eq!(f64, 0.0_f64, result));
    }

    #[test]
    fn test_varaince_single() {
        let x: Vec<f64> = vec![2.0];
        let y: Vec<f64> = vec![1.0];
        let result: f64 = covariance(&x, &y);

        assert!(approx_eq!(f64, 0.0_f64, result));
    }

    #[test]
    fn test_covariance() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![3.0, 4.0, 5.0, 6.0];
        let result: f64 = covariance(&x, &y);

        assert!(approx_eq!(f64, 1.25_f64, result));
    }

    #[test]
    fn test_covariance_empty() {
        let values: Vec<f64> = Vec::new();
        let result: f64 = covariance(&values, &values);

        assert!(approx_eq!(f64, 0.0_f64, result));
    }

    #[test]
    #[should_panic(expected = "x and y must be of equal length.")]
    fn test_covariance_wrong_lengths() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![3.0, 4.0, 5.0];
        covariance(&x, &y);
    }
}
