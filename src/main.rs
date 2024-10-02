//! A font editor.

use runebender_lib::data::{AppState, Workspace};

use masonry::dpi;
use winit::window;
use xilem::{self, view};

struct HelloWorld;

fn main() {
    let app = xilem::Xilem::new(HelloWorld, app_logic);
    let window_attributes = window::Window::default_attributes()
        .with_title("Hello World")
        .with_inner_size(dpi::LogicalSize::new(300, 200));
    app.run_windowed_in(xilem::EventLoop::with_user_event(), window_attributes)
        .unwrap();
}
    
fn app_logic(_: &mut HelloWorld) -> impl xilem::WidgetView<HelloWorld> {
    view::label("Hello, World!")
}

/// If there was an argument passed at the command line, try to open it as a .ufo
/// file, otherwise return blank state.
fn get_initial_state() -> AppState {
    let (font_file, path) = if let Some(arg) = std::env::args().nth(1) {
        match norad::Ufo::load(&arg) {
            Ok(ufo) => (ufo, Some(std::path::PathBuf::from(arg))),
            Err(e) => {
                eprintln!(
                    "Failed to load first arg '{}' as ufo file.\nError:'{}'",
                    arg, e
                );
                std::process::exit(1);
            }
        }
    } else {
        (runebender_lib::create_blank_font(), None)
    };

    let mut workspace = Workspace::default();
    workspace.set_file(font_file, path);
    AppState { workspace }
}

