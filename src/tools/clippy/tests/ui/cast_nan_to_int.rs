#![warn(clippy::cast_nan_to_int)]
#![allow(clippy::eq_op)]

fn main() {
    let _ = (0.0_f32 / -0.0) as usize;
    let _ = (f64::INFINITY * -0.0) as usize;
    let _ = (0.0 * f32::INFINITY) as usize;

    let _ = (f64::INFINITY + f64::NEG_INFINITY) as usize;
    let _ = (f32::INFINITY - f32::INFINITY) as usize;
    let _ = (f32::INFINITY / f32::NEG_INFINITY) as usize;

    // those won't be linted:
    let _ = (1.0_f32 / 0.0) as usize;
    let _ = (f32::INFINITY * f32::NEG_INFINITY) as usize;
    let _ = (f32::INFINITY - f32::NEG_INFINITY) as usize;
    let _ = (f64::INFINITY - 0.0) as usize;
}
