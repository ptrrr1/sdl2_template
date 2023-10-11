extern crate sdl2;

use sdl2::keyboard::Keycode;

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