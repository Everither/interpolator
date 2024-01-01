use nih_plug::prelude::*;

use interpolator::Interpolator;

fn main() {
    nih_export_standalone::<Interpolator>();
}