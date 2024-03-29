extern crate piston_window;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| {panic!("Failed to build PistonWindow: {}", e)});
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }
}
