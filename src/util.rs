use cgmath::Point3;

pub fn lerp(value: f32, smooth_value: f32, alpha: f32) -> f32 {
    return smooth_value * (1. - alpha) + value * alpha
}

pub fn range3d(xi: (i32, i32), yi: (i32, i32), zi: (i32, i32)) -> Vec<Point3<i32>>{
    let mut output = Vec::new();
    for x in xi.0..xi.1 {
        for y in yi.0..yi.1 {
            for z in zi.0..zi.1 {
                output.push(Point3 { x, y, z })
            }
        }
    }
    output
}