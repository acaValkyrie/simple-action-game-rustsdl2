use sdl2::pixels::Color;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use std::time::Duration;

mod sdl_modules;

pub fn main() {
    let (mut canvas, mut event_pump) 
        = sdl_modules::sdl_setup(800, 600);
    let (mut x, mut y) = (350, 250);
    
    // イベントループ
    'running: loop {
        if sdl_modules::is_end_event(&mut event_pump) { break 'running; }
        
        // キャンバスの初期化
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let state = &event_pump.keyboard_state();
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
