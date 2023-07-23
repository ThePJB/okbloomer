use std::collections::HashMap;
use crate::vector::*;
use crate::vector_i32::*;
use crate::random::*;
use crate::kimg::*;

use glow::*;
use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event::MouseButton;
use glutin::event::ElementState;

const CHUNK_XY: usize = 128;
const CHUNK_Z: usize = 64;

pub struct Game {
    pub xres: i32,
    pub yres: i32,
    pub window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    pub gl: glow::Context,

    pub program: glow::NativeProgram,
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
}

impl Game {
    pub unsafe fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Game {
        let xres = 800i32;
        let yres = 800i32;
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("OK BLOOMER")
            .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        gl.enable(DEPTH_TEST);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        gl.depth_func(LEQUAL);
        gl.enable(BLEND);
        // gl.enable(CULL_FACE);
        // gl.debug_message_callback(|a, b, c, d, msg| { println!("{} {} {} {} msg: {}", a, b, c, d, msg); });

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 4*3, 0);
        gl.enable_vertex_attrib_array(0);


        // Shader
        let program = gl.create_program().expect("Cannot create program");
    
        let vs = gl.create_shader(glow::VERTEX_SHADER).expect("cannot create vertex shader");
        gl.shader_source(vs, include_str!("shader.vert"));
        gl.compile_shader(vs);
        if !gl.get_shader_compile_status(vs) {
            panic!("{}", gl.get_shader_info_log(vs));
        }
        gl.attach_shader(program, vs);

        let fs = gl.create_shader(glow::FRAGMENT_SHADER).expect("cannot create fragment shader");
        gl.shader_source(fs, include_str!("shader.frag"));
        gl.compile_shader(fs);
        if !gl.get_shader_compile_status(fs) {
            panic!("{}", gl.get_shader_info_log(fs));
        }
        gl.attach_shader(program, fs);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
        gl.detach_shader(program, fs);
        gl.delete_shader(fs);
        gl.detach_shader(program, vs);
        gl.delete_shader(vs);

        let triangle_mesh = [
            // 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0
            -1.0, -1.0, 0.0,
            1.0, -1.0, 0.0,
            0.0, 1.0, 0.0
        ];
        let float_bytes: &[u8] = std::slice::from_raw_parts(
            triangle_mesh.as_ptr() as *const u8,
            std::mem::size_of_val(&triangle_mesh),
        );

        gl.use_program(Some(program));
        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, float_bytes, glow::STATIC_DRAW);

        gl.viewport(0, 0, xres, yres);

        Game {
            xres,
            yres,
            window,
            gl,
            vao,
            vbo,
            program,
        }
    }

    pub unsafe fn handle_event(&mut self, event: Event<()>) {
        match event {
            Event::LoopDestroyed |
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                std::process::exit(0);
            },

            Event::WindowEvent {event, .. } => {
                match event {
                    WindowEvent::Resized(size) => {
                        self.xres = size.width as i32;
                        self.yres = size.height as i32;
                        self.gl.viewport(0, 0, size.width as i32, size.height as i32)
                    },
                    _ => {},
                }
            },
            Event::MainEventsCleared => self.frame(),
            _ => {},
        }

    }

    pub unsafe fn frame(&mut self) {
        self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
        self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
        self.gl.use_program(Some(self.program));
        self.gl.bind_vertex_array(Some(self.vao));
        self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));

        self.gl.draw_arrays(glow::TRIANGLES, 0, 3 as i32);
        self.window.swap_buffers().unwrap();

    }
}


pub struct OpaqueChunk {
    data: [u64; CHUNK_XY * CHUNK_XY],
}

impl OpaqueChunk {
    // true if there is one, false if oob or not one
    pub fn get(&self, pos: Vec3i) -> bool {
        let min = vec3i(0,0,0);
        let max = vec3i(CHUNK_XY as i32 - 1, CHUNK_XY as i32 - 1, CHUNK_Z as i32 - 1);
        if pos < min {return false};
        if pos > max {return false};
        if self.data[pos.x as usize * CHUNK_XY + pos.y as usize] & (1 << pos.z as u64) == 0 {return false};
        return true;
    }
}

pub fn xy_verts_pos(xy: Vec2, dir: Vec2) -> (Vec2, Vec2) {
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
    face: Vec<u16>,
}

impl IndexedMesh {
    pub fn new() -> Self {
        IndexedMesh { vert: vec![], face: vec![] }
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
                self.face.push((len-1) as u16);
                self.face.push((len-2) as u16);
                self.face.push((len-4) as u16);
                self.face.push((len-3) as u16);
                self.face.push((len-4) as u16);
                self.face.push((len-1) as u16);

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
                self.face.push((len-1) as u16);
                self.face.push((len-2) as u16);
                self.face.push((len-4) as u16);
                self.face.push((len-3) as u16);
                self.face.push((len-4) as u16);
                self.face.push((len-1) as u16);

                start = z + 1;
            }
            val >>= 1;
            z += 1;
        }
    }
}

// yes there will be f32 vertex data and u16 index data
// do I make this just return a vao or what
// meshmanager should be vao, vbo per chunk

pub struct MeshManager {

}

pub struct OpaqueManager {
    chunks: HashMap<Vec3i, OpaqueChunk>,
}

pub fn heightmap(p: Vec2, seed: u32) -> f32 {
    let h = heightmap_unit(p, seed);
    h.max(0.0) * 30.0
}

// iq would do rotmap on baby p for less artifacts
pub fn heightmap_unit(p: Vec2, seed: u32) -> f32 {
    let f = 8.0;
    1.000 * noise_grad(p * f * 1.000, seed.wrapping_add(1713513437)) +
    0.500 * noise_grad(p * f * 2.000, seed.wrapping_add(1967234473)) +
    0.250 * noise_grad(p * f * 4.000, seed.wrapping_add(3851234713)) +
    0.125 * noise_grad(p * f * 8.000, seed.wrapping_add(3572312267)) /
    1.875 + 0.5
}

// well I can't really test correctness until I do meshings and renderings
// but I can print out stats etc.
pub fn generate(coords: Vec3i) -> OpaqueChunk {
    let mut chunk = OpaqueChunk { data: [0; CHUNK_XY * CHUNK_XY] };
    let coords_base = vec3(coords.x as f32 * CHUNK_XY as f32, coords.y as f32 * CHUNK_XY as f32, coords.z as f32);
    for i in 0..CHUNK_XY {
        for j in 0..CHUNK_XY {
            let xy_offset = vec3(i as f32, j as f32, 0.);
            let h = heightmap((coords_base + xy_offset).xy(), 69);
            if h < coords_base.z {
                chunk.data[i * CHUNK_XY + j] = 0;
            } else if h > coords_base.z + CHUNK_Z as f32 {
                chunk.data[i * CHUNK_XY + j] = 0xFFFFFFFFFFFFFFFF;
            } else {
                chunk.data[i * CHUNK_XY + j] = 0xFFFFFFFFFFFFFFFF >> ((h - coords_base.z).round()) as u64;
            }
        }
    }
    chunk
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

#[test]
pub fn hm_test() {
    let w = 1000;
    let h = 1000;
    let mut imbuf = ImageBuffer::new(w, h);
    for i in 0..w {
        for j in 0..h {
            let p = vec2(i as f32 / w as f32, j as f32 / h as f32);
            let h = heightmap_unit(p, 69);
            let c = ((255.0 * h) as u8, (255.0 * h) as u8, (255.0 * h) as u8);
            imbuf.set_px(i,j,c);
        }
    }
    imbuf.dump_to_file("hm.png");
}