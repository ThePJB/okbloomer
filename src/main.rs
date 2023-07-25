mod game;
mod vector;
mod vector_i32;
mod image;
mod random;
mod meshing;
mod heightmap;
mod chunk;
mod matrix;
mod camera;

fn main() {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let mut game = game::Game::new(&event_loop);
        event_loop.run(move |event, _, _| game.handle_event(event));
    }
}
