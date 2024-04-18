// mod is defined in the main file only
mod color;
mod color_tool_app;
mod command_loop;

// crate is the root module of the current crate.
use crate::color_tool_app::ColorToolApp;
use crate::command_loop::CommandLoop;

fn main() {
    let mut app = ColorToolApp::new();
    let mut command_loop = CommandLoop::new(&mut app);
    command_loop.run();
}
