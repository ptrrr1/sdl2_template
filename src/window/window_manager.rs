#[allow(non_snake_case)]
pub mod WindowManager {
	extern crate sdl2;

	use sdl2::video::Window;
	use sdl2::render::Canvas;

	use sdl2::EventPump;


	#[derive(Debug)]
	pub struct WindowBuilder {
	    name: String,
	    width: u32,
	    height: u32,
	}

	impl WindowBuilder {
		pub fn new(name: String, width: u32, height: u32) -> WindowBuilder {
			WindowBuilder {
				name: name,
	    		width: width,
	    		height: height,
			}
		}

	    pub fn create_window(&self) -> (EventPump, Canvas<Window>) {
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

	impl Default for WindowBuilder {
	    fn default() -> WindowBuilder {
	        WindowBuilder {
	            name: String::from("Window Name"),
	            width: 800,
	            height: 600,
	        }
	    }
	}
}