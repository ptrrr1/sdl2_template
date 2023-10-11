#[allow(non_snake_case, dead_code)]
pub mod SpriteManager {
	extern crate sdl2;

	use sdl2::render::{Texture, TextureCreator};
	use sdl2::video::WindowContext;
	
	use sdl2::image::LoadTexture;

	use std::path::Path;

	pub struct Sprite<'a> {
	    texture: Texture<'a>,
	    width: u32,
	    height: u32,
	    x: i32,
	    y: i32,
	}

	impl Sprite<'_> {
		pub fn new(texture_creator: &TextureCreator<WindowContext>, path: String, width: u32, height: u32, x: i32, y: i32) -> Sprite {
			Sprite {
				texture: Self::load(path, texture_creator),
	    		width: width,
	    		height: height,
	    		x: x,
	    		y: y,
			}
		}

		fn load(path: String, texture_creator: &TextureCreator<WindowContext>) -> Texture<'_> {
			let texture: Texture<'_> = texture_creator.load_texture(Path::new(&path)).unwrap();
		    return texture
		}

		pub fn texture(&self) -> &Texture<'_> {
			&self.texture
		}

		pub fn width(&self) -> u32 {
			self.width
		}

		pub fn height(&self) -> u32 {
			self.height
		}

		pub fn x(&self) -> i32{
            return self.x
        }

        pub fn y(&self) -> i32 {
            return self.y
        }

        pub fn set_x(&mut self, x: i32) {
            self.x = x;
        }

        pub fn set_y(&mut self, y: i32) {
            self.y = y;
        }
	}
}