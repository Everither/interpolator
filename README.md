# Interpolator

Downsamples the input, then interpolates between these samples. Sounds like a bit crusher, but less harsh. Has the ability to preserve tonality of high frequencies even when downsampled heavily.

Built with the [NIH-plug](https://github.com/robbert-vdh/nih-plug) framework and the [VIZIA](https://github.com/vizia/vizia) UI library.

![interpolator](https://github.com/Everither/interpolator/assets/122586326/59d7b935-0f6b-41bf-b8bd-bc548e290c99)

## How It Works

The input undergoes a downsampling process. A pair of samples is selected periodically, the distance between these two samples is determined by the `Amount` parameter. Then, all samples between these two selected samples will be assigned values such that they create a linear line (or a cubic curve) from start to end. 

The plugin also optionally restricts the maximum tolerable error between the original input and the interpolated (or in other words, approximated) output. Whenever a linear line drifts too far away from the original input, or more precisely, whenever the error exceeds a certain amount, then the line will be redrawn with a smaller `Amount` such that the error does not exceed the constraint. The maximum tolerable error is determined by the `Tolerance` knob. 

## Parameters
`Amount` The amount of downsampling, measured in samples.

`Smooth` Similar to dithering in image processing, in this context, creating the illusion of decimal values using a combination of different integer values.
- When disabled, `Amount` will snap to the nearest integer and automation will sound discrete. 
- When enabled, `Amount` will stay as a decimal and automation will sound smooth and continuous.

`Linear / Cubic` Blends between linear interpolation and cubic interpolation.

`Tolerance` The maximum allowed error before recomputation. With low values, the tonality of high frequencies is preserved, as `Amount` essentially dynamically adjusts to minimize error.

## Installation

Download linear_interpolator_v0.1.1.vst3.zip from [Releases](https://github.com/Everither/linear-interpolator/releases/tag/0.1.0), unzip and place in your VST3 folder. 
