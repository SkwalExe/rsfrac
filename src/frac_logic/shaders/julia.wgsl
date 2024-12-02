struct Params {
    max_iter: i32,
    width_px: i32,
    height_px: i32,
    pos_real: f32,
    pos_imag: f32,
    cell_size: f32,
    y_offset: i32,
    julia_constant_real: f32,
    julia_constant_imag: f32,
}


@group(0) @binding(0) var<storage, read_write> output_buf: array<i32>; 
@group(0) @binding(1) var<uniform> params: Params; 

fn coords_to_c(x_: u32, y_: u32) -> vec2<f32> {
    let x = -params.width_px / 2 + i32(x_);
    let y = -params.height_px / 2 + i32(y_) + params.y_offset;
    return vec2(
        f32(x) * params.cell_size + params.pos_real,
        f32(y) * params.cell_size + params.pos_imag,
    );
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x: u32 = global_id.x;
    let y: u32 = global_id.y;


    let point = coords_to_c(x, y);
    let index = y * u32(params.width_px) + x;

    output_buf[index] = iterations(point);
}


// ============= Everything above this line should be the same in all fractal shaders


fn iterations(point: vec2<f32>) -> i32 {
    var z = point;
    var iter: i32 = 0i;

    while length(z) < 4f && iter < params.max_iter {
        // z = vec2<f32>(
        //     pow(z.x, 2f) - pow(z.y, 2f),
        //     2f * z.x * z.y + imag
        // );
        z = vec2<f32>(
            pow(z.x, 2f) - pow(z.y, 2f) + params.julia_constant_real,
            2f * z.x * z.y + params.julia_constant_imag
        );
        iter = iter + 1i;
    }
    if iter == params.max_iter {
        return -1i;
    }
    return iter;
}

