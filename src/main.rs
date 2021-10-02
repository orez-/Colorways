use piston_window::*;
use sdl2_window::Sdl2Window;

mod app;
mod player;
mod view;
// mod maze;

fn main() {
    use opengl_graphics::GlGraphics;

    let (width, height) = (800, 800);

    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("LD49", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut app = app::App::new();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    while let Some(e) = window.next() {
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut gl);
        }

        if let Some(ref args) = e.update_args() {
           app.update(args);
        }

        if let Some(ref args) = e.press_args() {
            app.key_press(args);
        }

        if let Some(ref args) = e.release_args() {
            app.key_release(args);
        }
    }
}
