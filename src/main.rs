extern crate sdl2;

use std::time::Duration;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


#[derive(Debug)]
struct WindowSpecs {
    name: String,
    width: u32,
    height: u32,
}

impl WindowSpecs {
    fn create_window(&self) -> (EventPump, Canvas<Window>) {
        let sdl_context = sdl2::init().unwrap(); // Initialize
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(&self.name, self.width, self.height)
            .position_centered() // Create the Window Centered relative to screen size
            .build()
            .unwrap();

        let canvas : Canvas<Window> = window.into_canvas()
            .present_vsync() // Can't render faster than display rate
            .build().unwrap();    

        let event_pump = sdl_context.event_pump().unwrap();

        return (event_pump, canvas)
    }
}

impl Default for WindowSpecs {
    fn default() -> WindowSpecs {
        WindowSpecs {
            name: String::from("Window Name"),
            width: 800,
            height: 600,
        }
    }
}

fn main() {
    let window: WindowSpecs = Default::default();

    // Type Tuple (EventPump, Canvas<Window>)
    let (mut event_pump, mut canvas) = window.create_window();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // Game Loop

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }    
}