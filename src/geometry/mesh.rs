pub struct Mesh {
    pub verts: Vec<f32>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn new(verts: Vec<f32> , indices: Vec<u16>) -> Self {
        Self {
            verts: verts, indices: indices,
        }
    }
}

// returns vertices & indices of a flat grid
// TODO model this after grasshopper
// ((Plane, Domain, Size) -> Mesh)
pub fn create_grid(n: usize) -> Mesh {

    let np = n + 1;
    let mut verts: Vec<f32> = vec![0.0; 3 * np * np];   
    let mut indices: Vec<u16> = vec![0; 6 * n * n];
    
    // NOTE: subject to change
    let width: f32 = 2.0;
    let width_start: f32 = -1.;
    let square_size: f32 = width / n as f32;

    // create all positions
    for z in 0..np {
        for x in 0..np {
            let start = 3 * ((z * np) + x);

            verts[start + 0] = width_start + (x as f32) * square_size;
            verts[start + 1] = 0.;
            verts[start + 2] = width_start + (z as f32) * square_size;
        }
    }    

    // create all indices
    // a---c
    // | \ |
    // b---d
    for z in 0..n {
        for x in 0..n {
            let start_index = 6 * (z * n + x);
            let a = (z * np + x) as u16;
            let b = (a + np as u16) as u16;
            let c = (a + 1) as u16;
            let d = (b + 1) as u16;

            indices[start_index + 0] = a;
            indices[start_index + 1] = b;
            indices[start_index + 2] = d;
            indices[start_index + 3] = c;
            indices[start_index + 4] = a;
            indices[start_index + 5] = d;
        }
    }    

    Mesh::new(verts, indices)
}