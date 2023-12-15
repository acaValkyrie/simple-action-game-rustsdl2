use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Duration;

pub fn main() {
    // SDL2の初期化
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // ウィンドウの作成
    let window = video_subsystem
        .window("Rust-SDL2", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // キャンバスの作成
    let mut canvas = window.into_canvas().build().unwrap();
    
    let (mut x, mut y) = (350, 250);
    
    // イベントループ
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // キャンバスの初期化
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let state = event_pump.keyboard_state();
        if state.is_scancode_pressed(Scancode::Up){    y -= 5; }
        if state.is_scancode_pressed(Scancode::Down){  y += 5; }
        if state.is_scancode_pressed(Scancode::Left){  x -= 5; }
        if state.is_scancode_pressed(Scancode::Right){ x += 5; }
        
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(x, y, 100, 100)).unwrap();

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
