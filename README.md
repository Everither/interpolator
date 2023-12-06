# Linear Interpolator

Downsamples the input, then linearly interpolates between these samples. Sounds like a bit crusher, but less harsh. 

Built with the [NIH-plug](https://github.com/robbert-vdh/nih-plug) framework and the [VIZIA](https://github.com/vizia/vizia) UI library.

![image](https://github.com/Everither/linear-interpolator/assets/122586326/622cbd5e-39dc-4185-a474-68f00efd0ca2)

## How It Works

The amplitude of each individual sample point determines how far it gets forward time shifted.

For example, an array of 10 sample points gets processed by the algorithm, with indexes 0 to 9.
Each sample has an amplitude between -1 and 1.

To process the first sample (the sample at index 0) the absolute value of its amplitude (basically dropping the negative sign if there is one) is multiplied by some constant (determined by the parameter `Amount`) then rounded down to an integer.

Suppose the first sample (the sample at index 0) has amplitude 0.3, and `Amount` is set to 10, then the calculated value is 0.3*10=3.
Take 3 steps from this sample, arriving at the fourth sample (sample at index 3). The first sample is then assigned the amplitude of the fourth sample.

## Parameters
`Amount` The maximum time shifting distance, measured in samples.

`Curve` The curve/tension in the time shifting calculation.

- At 1.0, it behaves as a linear line, meaning more dynamics, shortened duration of distortion
- At 0.5, it behaves as a square root curve, meaning less dynamics, longer distortion

`Invert` Inverts the algorithm. When enabled, peaks are preserved more, leading to a warmer sound. 

## Installation
Download dust_saturator_v0.1.1.vst3.zip from [Releases](https://github.com/Everither/dust-saturation/releases/tag/v0.1.1), unzip and place in your VST3 folder. 

