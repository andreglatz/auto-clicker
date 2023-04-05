use enigo::{Enigo, MouseButton, MouseControllable};
use inputbot::{KeybdKey::*, *};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread, time,
};

fn main() {
    println!("Press F6 to start/stop the clicker");

    let clicker_is_active = Arc::new(AtomicBool::new(false));
    let clicker_is_active_clone = Arc::clone(&clicker_is_active);

    thread::spawn(move || {
        let mut enigo = Enigo::new();

        loop {
            if clicker_is_active_clone.load(Ordering::Relaxed) {
                enigo.mouse_click(MouseButton::Left);
                thread::sleep(time::Duration::from_millis(100));
            }
        }
    });

    F6Key.bind(move || {
        let val = clicker_is_active.load(Ordering::Relaxed);
        println!("Clicker is now {}", if val { "off" } else { "on" });

        clicker_is_active.store(!val, Ordering::Relaxed);
    });

    handle_input_events();
}
