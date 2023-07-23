mod game;
mod vector;
mod vector_i32;
mod kimg;
mod random;
mod meshing;
mod heightmap;
mod chunk;

fn main() {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let mut game = game::Game::new(&event_loop);
        event_loop.run(move |event, _, _| game.handle_event(event));
    }
}
