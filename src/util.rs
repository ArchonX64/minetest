
pub fn lerp(value: f32, smooth_value: f32, alpha: f32) -> f32 {
    return smooth_value * (1. - alpha) + value * alpha
}