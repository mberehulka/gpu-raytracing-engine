@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    switch i32(i) {
        case 0:  { return vec4<f32>(-1.,  1., 0., 1.); }
        case 1:  { return vec4<f32>(-1., -1., 0., 1.); }
        case 2:  { return vec4<f32>( 1., -1., 0., 1.); }
        case 3:  { return vec4<f32>( 1.,  1., 0., 1.); }
        case 4:  { return vec4<f32>(-1.,  1., 0., 1.); }
        default: { return vec4<f32>( 1., -1., 0., 1.); }
    }
}

struct Camera {
    @location(0) position: vec4<f32>,
    @location(1) direction: vec4<f32>,
    @location(2) screen_size: vec4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

@fragment fn fs_main(@builtin(position) pixel_pos: vec4<f32>) -> @location(0) vec4<f32> {
    let screen_coord = vec2<f32>(pixel_pos.x * camera.screen_size.x - 1., pixel_pos.y * camera.screen_size.y - 1.);
    return vec4<f32>(0.);
}