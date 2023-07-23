use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use crate::vector::*;
use crate::vector_i32::*;
use crate::random::*;
use crate::image::*;
use crate::chunk::*;
use crate::meshing::*;
use crate::matrix::*;

use glow::*;
use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event::MouseButton;
use glutin::event::ElementState;

pub struct Game {
    pub xres: i32,
    pub yres: i32,
    pub window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    pub gl: glow::Context,

    pub program: glow::NativeProgram,
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,

    pub cam_pos: Vec3,
    pub cam_dir: Vec3,

    pub held_keys: HashSet<VirtualKeyCode>,
    pub t_last: Instant,
    pub t: f32,
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
            -0.1f32, -0.1, -1.0,
            0.1, -0.1, -1.0,
            0.0, 0.1, -1.0
        ];
        let float_bytes: &[u8] = std::slice::from_raw_parts(
            triangle_mesh.as_ptr() as *const u8,
            triangle_mesh.len() * 4,
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
            cam_pos: vec3(0.0, 0.0, 0.0),
            cam_dir: vec3(0.0, 0.0, -1.0),
            held_keys: HashSet::new(),
            t_last: Instant::now(),
            t: 0.0,
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
                    WindowEvent::KeyboardInput {input, ..} => {
                        match input {
                            glutin::event::KeyboardInput {virtual_keycode: Some(code), state: ElementState::Pressed, ..} => {
                                self.held_keys.insert(code);
                            },
                            glutin::event::KeyboardInput {virtual_keycode: Some(code), state: ElementState::Released, ..} => {
                                self.held_keys.remove(&code);
                            },
                            _ => {},
                        }
                    },
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
        let t_now = Instant::now();
        let dt = (t_now - self.t_last).as_secs_f32();
        self.t += dt;
        self.t_last = t_now;

        self.simulate(dt);

        self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
        self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
        self.gl.use_program(Some(self.program));
        self.gl.bind_vertex_array(Some(self.vao));
        self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));

        let cam_mat = look_at(self.cam_pos, self.cam_dir, 2.0, self.xres as f32 / self.yres as f32, 0.01, 1000.0);
        self.gl.uniform_matrix_4_f32_slice(self.gl.get_uniform_location(self.program, "projection").as_ref(), true, &cam_mat);

        self.gl.draw_arrays(glow::TRIANGLES, 0, 3 as i32);
        self.window.swap_buffers().unwrap();
    }

    pub fn simulate(&mut self, dt: f32) {
        let x = if self.held_keys.contains(&VirtualKeyCode::A) {
            -1.0f32
        } else if self.held_keys.contains(&VirtualKeyCode::D) {
            1.0
        } else {
            0.0
        };
        let z = if self.held_keys.contains(&VirtualKeyCode::W) {
            -1.0f32
        } else if self.held_keys.contains(&VirtualKeyCode::S) {
            1.0
        } else {
            0.0
        };
        let y = if self.held_keys.contains(&VirtualKeyCode::LControl) {
            -1.0f32
        } else if self.held_keys.contains(&VirtualKeyCode::LShift) {
            1.0
        } else {
            0.0
        };

        self.movement(vec3(x, y, z).normalize(), dt);
    }


    
    pub fn movement(&mut self, dir: Vec3, dt: f32) {
        let speed = 10.0;

        let up = vec3(0.0, 1.0, 0.0);
        let cam_right = (up.cross(self.cam_dir)).normalize();
        let cam_up = cam_right.cross(self.cam_dir).normalize();

        let v = self.cam_dir * dir.dot(self.cam_dir) + cam_right * dir.dot(cam_right) + cam_up * dir.dot(cam_up);

        self.cam_pos += dt * speed * v;
    }
    // not getting smaller
}