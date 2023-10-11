
#[path = "window_manager.rs"] mod window;
use crate::window::WindowManager::WindowBuilder;

#[path = "process_input.rs"] mod process_input;

extern crate sdl2;

use sdl2::pixels::Color;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

fn main() {
    let window: WindowBuilder = WindowBuilder::default();

    // Type Tuple (EventPump, Canvas<Window>, TextureCreator<WindowContext>)    
    let (mut event_pump, mut canvas, texture_creator) = window.create_window();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                Event::KeyDown { keycode: Some(key), .. } => { process_input::key_down(key) },

                Event::KeyUp { keycode: Some(key), .. } => { process_input::key_up(key) }

                _ => {}
            }
        }
        // Game Loop

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }    
}