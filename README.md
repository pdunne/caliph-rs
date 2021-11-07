# Caliph-RS

A simple tool to calibrate and convert pH measurements using a two point method.

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)
[![License: CC BY-SA 4.0](https://img.shields.io/badge/License-CC%20BY--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-sa/4.0/)

The two command line tools are `caliph`, and '`conph`.

## Calibration

Usage:

When the temperature is 25˚C during the measurement:
```console
caliph 3.97 10.2
```


Optional temperature argument:

```console
caliph 3.97 10.2 -t 22.3

```
Boolean flat to save the calibration to `calibration.ph` in the current directory:

```console
caliph 3.97 10.2 -t 22.3 -s
```
 
 ## Conversion
 
 Assuming the `calibration.ph` file exists:
 
```console
conph 3.5
```

Custom calibration settings fof the slope and offset:

`-c` sets it to custom, `-s VAL` is for the slope, `-o VAL` is for the offset
```console
conph 3.5 0 -c -s 1.1 -o 0.02
```


Boolean flat to save the calibration to `calibration.ph` in the current directory:

```console
caliph 3.97 10.2 -t 22.3 -s
```
