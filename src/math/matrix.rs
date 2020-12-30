// author: Jos Feenstra 
// based on: Doug Milford's Rust tutorials


pub fn create_identity() -> [f32; 16]
{
    [
        1.,    0.,     0.,     0.,
        0.,     1.,     0.,     0.,
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
