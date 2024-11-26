struct Params {
    max_iter: i32,
    width_px: i32,
}


@group(0) @binding(0) var<storage, read> input_buf: array<vec2<f32>>; 
@group(0) @binding(1) var<storage, read_write> output_buf: array<i32>; 
@group(0) @binding(2) var<uniform> params: Params; 


fn mandelbrot_iterations(real: f32, imag: f32) -> i32 {
    var iter: i32 = 0i;
    var z: vec2<f32> = vec2<f32>(0f, 0f);

    while length(z) < 2f && iter < params.max_iter {
        z = vec2<f32>(
            z.x * z.x - z.y * z.y + real,
            2f * z.x * z.y + imag
        );
        iter = iter + 1i;
    }
    if iter == params.max_iter {
        return -1i;
    }
    return iter;
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x: u32 = global_id.x;
    let y: u32 = global_id.y;
    let index = y * u32(params.width_px) + x;
    let point: vec2<f32> = input_buf[index];
    output_buf[index] = mandelbrot_iterations(point.x, point.y);
}


