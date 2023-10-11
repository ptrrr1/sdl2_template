#[allow(non_snake_case)]
pub mod ProcessInput {
    extern crate sdl2;
    use sdl2::keyboard::Keycode;
    use sdl2::mouse::MouseButton;

    pub fn mouse_down(mouse_btn: MouseButton) {
        match mouse_btn {
            MouseButton::Left => { println!("Left Down"); },

            MouseButton::Middle => { println!("Middle Down"); },

            MouseButton::Right => { println!("Right Down"); },
            
            _ => (),
        }
    }

    pub fn mouse_up(mouse_btn: MouseButton) {
        match mouse_btn {
            MouseButton::Left => { println!("Left Up"); },

            MouseButton::Middle => { println!("Middle Up"); },

            MouseButton::Right => { println!("Right Up"); },
            
            _ => (),
        }
    }

    pub fn mouse_wheel(x: i32, y: i32) {
        match (x, y) {
            (0, y) if y > 0 => { println!("Up"); },

            (0, y) if y < 0 => { println!("Down"); },

            (x, 0) if x > 0 => { println!("Right"); },

            (x, 0) if x < 0 => { println!("Left"); },

            _ => (),
        }
    }

    pub fn key_down(key: Keycode) {
        match key {
            Keycode::W | Keycode::Up => { println!("W or Arrow Up down") },

            Keycode::A | Keycode::Left => { println!("A or Arrow Left down") },

            Keycode::S | Keycode::Down => { println!("S or Arrow Down down") },

            Keycode::D | Keycode::Right => { println!("D or Arrow Right down") },
            
            _ => (),
        }
    }

    pub fn key_up(key: Keycode) {
        match key {
            Keycode::W | Keycode::Up => { println!("W or Arrow Up up") },

            Keycode::A | Keycode::Left => { println!("A or Arrow Left up") },

            Keycode::S | Keycode::Down => { println!("S or Arrow Down up") },

            Keycode::D | Keycode::Right => { println!("D or Arrow Right up") },
            
            _ => (),
        }
    }
}