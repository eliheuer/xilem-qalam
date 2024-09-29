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
