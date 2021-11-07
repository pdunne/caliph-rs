/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! This contains two submodules for parsing command line inputs using `clap` for both `caliph`, and `conph`
//!

mod args_caliph;
mod args_conph;

pub use args_caliph::CalibArgs;
pub use args_conph::ConvArgs;
