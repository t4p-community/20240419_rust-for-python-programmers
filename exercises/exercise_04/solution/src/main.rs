mod car;
mod car_tool_app;
mod command_loop;

use crate::car_tool_app::CarToolApp;
use crate::command_loop::CommandLoop;

fn main() {
    let mut app = CarToolApp::new();
    let mut command_loop = CommandLoop::new(&mut app);
    command_loop.run();
}
