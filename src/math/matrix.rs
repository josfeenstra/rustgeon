// author: Jos Feenstra 
// based on: Doug Milford's Rust tutorials
use nalgebra::Perspective3;

pub fn create_identity() -> [f32; 16]
{
    [
        1.,     0.,     0.,     0.,
        0.,     1.,     0.,     0.,
        0.,     0.,     1.,     0.,
        0.,     0.,     0.,     1.
    ]
}

pub fn create_rotation_x_axis(r:f32) -> [f32; 16]
{
    [
        1.,     0.,     0.,       0.,
        0.,  r.cos(),-r.sin(),    0.,
        0.,  r.sin(), r.cos(),    0.,
        0.,     0.,     0.,       1.
    ]
}

pub fn create_rotation_y_axis(r:f32) -> [f32; 16]
{
    [
     r.cos(),   0.,  r.sin(),   0.,
        0.,     1.,     0.,     0.,
    -r.sin(),   0.,  r.cos(),   0.,
        0.,     0.,     0.,     1.
    ]
}

pub fn create_rotation_z_axis(r:f32) -> [f32; 16]
{
    [
     r.cos(), -r.sin(), 0.,     0.,
     r.sin(),  r.cos(), 0.,     0.,
        0.,     0.,     1.,     0.,
        0.,     0.,     0.,     1.
    ]
}

pub fn create_translation(tx: f32, ty: f32, tz: f32) -> [f32; 16] 
{
    let mut m = create_identity();
    m[12] = tx;
    m[13] = ty;
    m[14] = tz;

    m
}

pub fn create_scale(sx: f32, sy: f32, sz: f32) -> [f32; 16] 
{
    let mut m = create_identity();
    m[0]  = sx;
    m[5]  = sy;
    m[11] = sz;

    m
}

pub fn multiply(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut m = [0.; 16];

    m[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    m[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    m[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    m[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    m[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    m[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    m[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    m[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    m[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    m[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    m[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    m[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    m[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    m[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    m[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    m[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    m
}

pub fn print_once(m: [f32; 16], key: &str)
{
    // very elegant and scalable...
    let string: String = format!("{}, {}, {}, {} \n{}, {}, {}, {} \n {}, {}, {}, {} \n {}, {}, {}, {}",
    m[0], m[1],m[2],m[3],m[4],m[5],m[6],m[7], m[8], m[9], m[10], m[11], m[12], m[13], m[14], m[15])
        ;
    super::super::log_once(&string, key);
}

// get the 'camera' matrix
pub fn get_3d_projection_matrix(
    bottom: f32, top: f32, left: f32, right: f32,
    canvas_width: f32, canvas_height: f32, alpha_angle: f32, beta_angle: f32, scroll_scale: f32
) -> [f32; 16] {
    const PI: f32 = std::f32::consts::PI;
    const FIELD_OF_VIEW: f32 = 45. * std::f32::consts::PI / 100.;
    const Z_FAR: f32 = 100.;
    const Z_NEAR: f32 = 0.1;
    let z_plane: f32 = -1. / (PI / 8.).tan();
    
    // aspects
    let aspect: f32 = canvas_width / canvas_height; // note: this should be constant
    let scale_x = (right-left) / canvas_width;
    let scale_y = (top-bottom) / canvas_height;
    let scale_val = scale_y + scroll_scale;

    // translated to fit screen
    let translation = create_translation(
        -1. + scale_x + 2. * left / canvas_width,
        -1. + scale_y + 2. * bottom / canvas_height,
        z_plane,
    );
    
    // rotated by user
    let x_rotation = create_rotation_x_axis(alpha_angle);
    let y_rotation = create_rotation_y_axis(beta_angle);
    let rotation = multiply(x_rotation, y_rotation);

    // scaled to fit screen
    let scale = create_scale(scale_val, scale_val, 0.);

    // total 
    let transform = multiply(multiply(rotation, scale), translation);

    // projection to screen
    let perspective_tmp: Perspective3<f32> = Perspective3::new(aspect, FIELD_OF_VIEW, Z_NEAR, Z_FAR);
    let mut perspective: [f32; 16] = [0.; 16];
    perspective.copy_from_slice(perspective_tmp.as_matrix().as_slice());

    // return
    multiply(transform, perspective)
}
