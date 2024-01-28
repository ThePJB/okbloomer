
use crate::vector::*;
use crate::chunk::*;
use glow::HasContext;

fn xy_verts_pos(xy: Vec2, dir: Vec2) -> (Vec2, Vec2) {
    // xy + dir + dir*90o
    // xy + dir + dir*-90o
    let dir = dir * 0.5;
    let dir_plus_90 = vec2(dir.y, -dir.x);
    let dir_minus_90 = vec2(dir.y, -dir.x);
    let v = xy + vec2(0.5, 0.5);
    return (v + dir + dir_plus_90, v + dir + dir_minus_90);
}

pub struct IndexedMesh {
    vert: Vec<Vec3>,
    ind: Vec<u16>,
}

impl IndexedMesh {
    pub fn new() -> Self {
        IndexedMesh { vert: vec![], ind: vec![] }
    }
    // better to push all and reuse ind
    // how to determine ind
    // intermediate format or not
    // list of sorted start/ends
    // chunk mesh data needs to be origin based and transform is relative to player for le precish
    pub fn push_col_all(&mut self, xy: Vec2, px: u64, mx: u64, py: u64, my: u64, pz: u64, mz: u64) {
        self.push_col(px, xy, vec2(1., 0.));
        self.push_col(mx, xy, vec2(-1., 0.));
        self.push_col(py, xy, vec2(0., 1.));
        self.push_col(my, xy, vec2(0., -1.));
        self.push_col_nocombine(pz, xy, 1.0);
        self.push_col_nocombine(mz, xy, -1.0);
    }

    pub fn push_col_nocombine(&mut self, col: u64, xy: Vec2, dirz: f32) {
        let xy = xy + vec2(0.5, 0.5);
        let i = vec2(0.5, 0.0);
        let j = vec2(0.0, 0.5);

        let p00 = xy - i - j;
        let p01 = xy - i + j;
        let p11 = xy + i + j;
        let p10 = xy + i - j;

        let mut z = 0;
        let mut val = col;

        // up z or down z
        let dirz = dirz.max(0.0);
        while val > 0 {
            if val & 1 == 1 {
                self.vert.push(vec3(p00.x, p00.y, z as f32 + dirz));
                self.vert.push(vec3(p01.x, p01.y, z as f32 + dirz));
                self.vert.push(vec3(p10.x, p10.y, z as f32 + dirz));
                self.vert.push(vec3(p11.x, p11.y, z as f32 + dirz));

                let len = self.vert.len();
                self.ind.push((len-1) as u16);
                self.ind.push((len-2) as u16);
                self.ind.push((len-4) as u16);
                self.ind.push((len-3) as u16);
                self.ind.push((len-4) as u16);
                self.ind.push((len-1) as u16);

            }
            val >>= 1;
            z += 1;
        }
    }

    // or do we do it and then trim unused, sound slow
    // can code several implementations and compare the numbers
    // ok because +0,+0 refers to 
    pub fn push_col(&mut self, col: u64, xy: Vec2, dir: Vec2) {
        let (v1, v2) = xy_verts_pos(xy, dir);
        let mut z = 0;
        let mut start = 0;
        let mut val = 0;
        while val > 0 {
            if val & 1 == 1 {
                // sweet
            } else {
                // and emit start-z quad
                // start inds
                self.vert.push(vec3(v1.x, v1.y, start as f32));
                self.vert.push(vec3(v2.x, v2.y, start as f32));
                self.vert.push(vec3(v1.x, v1.y, z as f32));
                self.vert.push(vec3(v2.x, v2.y, z as f32));

                let len = self.vert.len();
                self.ind.push((len-1) as u16);
                self.ind.push((len-2) as u16);
                self.ind.push((len-4) as u16);
                self.ind.push((len-3) as u16);
                self.ind.push((len-4) as u16);
                self.ind.push((len-1) as u16);

                start = z + 1;
            }
            val >>= 1;
            z += 1;
        }
    }
}


/*
// mesh on cpu for now, eye for making this a geometry shader
// can also do dual contouring / / marching cubes at this step
// um directional culling but im not having 6 vaos per chunk, does it need subbuffers or something. sheesh
// fark not even exploiting my u64s here. just write it shit first
// but yea can just do andings and shiftings to determine quads mask
pub fn mesh(chunk: OpaqueChunk) -> IndexedMesh {
    const PLUS_X: usize = 0;
    const MINUS_X: usize = 1;
    const PLUS_Y: usize = 2;
    const MINUS_Y: usize = 3;
    const PLUS_Z: usize = 4;
    const MINUS_Z: usize = 5;

    let dirs = [
        vec3i(1, 0, 0),
        vec3i(-1, 0, 0),
        vec3i(0, 1, 0),
        vec3i(0, -1, 0),
        vec3i(0, 0, 1),
        vec3i(0, 0, -1),
    ];

    // +x -x +y -y +z -z
    let mut quads = [vec![], vec![], vec![], vec![], vec![], vec![]];

    for dir in 0..6 {
        for i in 0..CHUNK_XY {
            for j in 0..CHUNK_XY {
                for k in 0..CHUNK_Z {
                    let p = vec3i(i as i32, j as i32, k as i32);
                    if !chunk.get(p) {continue};
                    if chunk.get(p + dirs[dir]) {continue};
                    // emit quad
                    quads[dir].push(p)
                }
            }
        }
    }

    // so this would be god tier if i wasnt doing this bit shit

    let mut combined_quads = [vec![], vec![], vec![], vec![], vec![], vec![]];

    // quads should be sorted
    // quads -> combined quads?
    // should be like (low, high) tuple. Maybe is like a fold.
    for dir in 0..4 {
        let start = None;
        let prev_accepted = None;
        for i in 0..quads[dir].len() {
            if start == None {
                start = Some(quads[dir][i]);
                prev_accepted = Some(quads[dir][i]);
            } else if let Some(start_pos) = start {
                // is there a way of doing prev
                // if not broken from prev
                // prev accepted
                if let Some(prev_accepted_pos) = prev_accepted {

                }
            }
            // combined_quads[i] = 
        }

        // no do like just prev and ensure same x, same y, 1 less z, for example
    }
    
    let mut result = RawMesh{vertex: vec![], index: vec![]};
    
    
    // possibly quads list as intermediate representation
    // we do the occlusion cullings
    // we could greedy mesh: combine adjacent quads
    // the half assed greedy meshing i did was good: pick a direction and combine in that direction
    
    // quads by direction
    // quads by plane
    // do face extraction first since that reduces like 97% 
    // then combine to polys
    // then polys to quads decomposition
    // then I guess select a random one, attempt to expand rows cols in a direction
}
*/

// consider column_face_meshes is col & ~neigh
// then contiguous ones mesh together sya 2 lowest and 2 highest
// so if we hadf 0b000001111101111
// we say, start = 0, rsh and while 1 &1 == 1 and count

pub fn mesh_bitboard(chunk: OpaqueChunk) -> IndexedMesh {
    let mut mesh = IndexedMesh::new();

    for i in 0..CHUNK_XY {
        for j in 0..CHUNK_XY {
            let me = chunk.data[i * CHUNK_XY + j];
            let n_mx = if i > 0 {
                chunk.data[(i-1) * CHUNK_XY + j]
            } else {
                0
            };
            let n_px = if i < (CHUNK_XY-1) {
                chunk.data[(i+1) * CHUNK_XY + j]
            } else {
                0
            };
            let n_my = if j > 0 {
                chunk.data[i * CHUNK_XY + (j-1)]
            } else {
                0
            };
            let n_py = if j < (CHUNK_XY-1) {
                chunk.data[i * CHUNK_XY + (j+1)]
            } else {
                0
            };

            // masks for presence of quad
            let qpx = me & !n_px;
            let qmx = me & !n_mx;
            let qpy = me & !n_py;
            let qmy = me & !n_my;
            let qpz = me & !(me << 1);
            let qmz = me & !(me >> 1);

            mesh.push_col_all(vec2(i as f32, j as f32), qpx, qmx, qpy, qmy, qpz, qmz);
        }
    }
    mesh
}

pub unsafe fn mesh_triangle(gl: &glow::Context) -> MeshHandle {
    let mut mesh = IndexedMesh::new();

    // Vertices
    mesh.vert.push(vec3(1.0, 0.0, -0.5));    // 0
    mesh.vert.push(vec3(0.0, 1.0, -0.5));    // 0
    mesh.vert.push(vec3(-1.0, 0.0, -0.5));    // 0
    mesh.ind.push(0);
    mesh.ind.push(1);
    mesh.ind.push(2);
    mesh.upload(gl)
}

pub unsafe fn mesh_cube(gl: &glow::Context) -> MeshHandle {
    let mut mesh = IndexedMesh::new();

    // Vertices
    mesh.vert.push(vec3(1.0, 1.0, 1.0));    // 0
    mesh.vert.push(vec3(1.0, 1.0, -1.0));   // 1
    mesh.vert.push(vec3(1.0, -1.0, -1.0));  // 2
    mesh.vert.push(vec3(1.0, -1.0, 1.0));   // 3
    mesh.vert.push(vec3(-1.0, -1.0, 1.0));  // 4
    mesh.vert.push(vec3(-1.0, 1.0, 1.0));   // 5
    mesh.vert.push(vec3(-1.0, 1.0, -1.0));  // 6
    mesh.vert.push(vec3(-1.0, -1.0, -1.0)); // 7

    // Indices for each face (using clockwise order)
    // Front face (v0, v1, v5), (v0, v5, v6)
    mesh.ind.push(0);
    mesh.ind.push(1);
    mesh.ind.push(5);

    mesh.ind.push(0);
    mesh.ind.push(5);
    mesh.ind.push(6);

    // Right face (v0, v6, v7), (v0, v7, v3)
    mesh.ind.push(0);
    mesh.ind.push(6);
    mesh.ind.push(7);

    mesh.ind.push(0);
    mesh.ind.push(7);
    mesh.ind.push(3);

    // Back face (v3, v7, v4), (v3, v4, v2)
    mesh.ind.push(3);
    mesh.ind.push(7);
    mesh.ind.push(4);

    mesh.ind.push(3);
    mesh.ind.push(4);
    mesh.ind.push(2);

    // Left face (v1, v2, v4), (v1, v4, v5)
    mesh.ind.push(1);
    mesh.ind.push(2);
    mesh.ind.push(4);

    mesh.ind.push(1);
    mesh.ind.push(4);
    mesh.ind.push(5);

    // Top face (v0, v3, v2), (v0, v2, v1)
    mesh.ind.push(0);
    mesh.ind.push(3);
    mesh.ind.push(2);

    mesh.ind.push(0);
    mesh.ind.push(2);
    mesh.ind.push(1);

    // Bottom face (v4, v7, v6), (v4, v6, v5)
    mesh.ind.push(4);
    mesh.ind.push(7);
    mesh.ind.push(6);

    mesh.ind.push(4);
    mesh.ind.push(6);
    mesh.ind.push(5);

    // Return the mesh handle
    mesh.upload(gl)
}

// upload: RawMesh -> vao,vbo

// could defs have "All one or the other" optimization
// does introduce a branch and non homogenous / different class
//  but it is a majority of instances too.
// maybe just goes in chunk_properties: small struct. AllOpaque: bool, AllTransparent: bool
// be good if hashing shit was the same for all
// see it could have more elaborate structures in it. could have a much bigger one that was explicit of structures etc.
// that was sparse instead. some sparse, seems powerful
// and dont keep chunks for shit thats non default. like tree growth if theres no trees in the chunk
// ideally reusing indexes with a fixed offset

impl IndexedMesh {
    pub unsafe fn upload(&self, gl: &glow::Context) -> MeshHandle {
        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        let ebo = gl.create_buffer().unwrap();

        let vert_bytes: &[u8] = std::slice::from_raw_parts(
            self.vert.as_ptr() as *const u8,
            self.vert.len() * 4 * 3,
        );
        let ind_bytes: &[u8] = std::slice::from_raw_parts(
            self.ind.as_ptr() as *const u8,
            self.ind.len() * 2,
        );

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.bind_vertex_array(Some(vao));
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 4*3, 0);
        gl.enable_vertex_attrib_array(0);

        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vert_bytes, glow::STATIC_DRAW);
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, ind_bytes, glow::STATIC_DRAW);
        // drop(element_u8);
        // drop(vertex_u8);
        MeshHandle {vao, vbo, ebo, num_verts: self.ind.len()}
    }
}

pub struct MeshHandle {
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub ebo: glow::NativeBuffer,
    pub num_verts: usize,
}

impl MeshHandle {
    pub unsafe fn draw(&self, gl: &glow::Context) {
        gl.bind_vertex_array(Some(self.vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
        gl.draw_elements(glow::TRIANGLES, self.num_verts as i32, glow::UNSIGNED_SHORT, 0);
    }
}