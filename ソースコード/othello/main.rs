#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ggez::event::{self};
use ggez::ContextBuilder;
mod mygame;
use mygame::MyGame;
const WIDTH: f32 = 480.0;
const HEIGHT: f32 = 540.0;
fn main() {
    let resource_dir = std::path::PathBuf::from("./resources");

    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Ide")
        .window_mode(ggez::conf::WindowMode {
            width: WIDTH,
            height: HEIGHT,
            borderless: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            maximized: false,
            resizable: false,
            visible: true,
            min_width: 0.,
            max_width: 0.,
            min_height: 0.,
            max_height: 0.,
        })
        .add_resource_path(resource_dir)
        .build()
        .expect("Could not create ggez context!");
    let title = "MyOthelloGame".to_string();
    ggez::graphics::set_window_title(&ctx, &title);
    let my_game = MyGame::new(&mut ctx, WIDTH as u32, HEIGHT as u32);
    event::run(ctx, event_loop, my_game);
}
