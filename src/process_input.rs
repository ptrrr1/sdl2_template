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
            Keycode::W => { println!("W down") },

            Keycode::A => { println!("A down") },

            Keycode::S => { println!("S down") },

            Keycode::D => { println!("D down") },
            
            _ => (),
        }
    }

    pub fn key_up(key: Keycode) {
        match key {
            Keycode::W => { println!("W up") },

            Keycode::A => { println!("A up") },

            Keycode::S => { println!("S up") },

            Keycode::D => { println!("D up") },
            
            _ => (),
        }
    }
}