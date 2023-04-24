use core::panic;
use core::time;
use rand::Rng;
use rayon::prelude::*;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;
const CELLSIZE: usize = 2;
const BORDERSIZE: usize = 0;
const TWODWIDTH: usize = WIDTH * HEIGHT;
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Window",
            ((WIDTH * CELLSIZE) + BORDERSIZE).try_into().unwrap(),
            ((WIDTH * CELLSIZE) + BORDERSIZE).try_into().unwrap(),
        )
        .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        //.index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
    print!("\x1B[2J\x1B[1;1H");
    print!("\x1B[?25l");
    let mut map = Vec::new();
    for _i in 0..TWODWIDTH {
        map.push(false);
    }
    let wait_time = time::Duration::from_millis(1   );
    map[get_index(0, 0)] = true;
    map[get_index(0, 1)] = true;
    map[get_index(2, 0)] = true;
    map[get_index(1, 2)] = true;
    map[get_index(1, 0)] = true;
    //print_map(&map);
    //let sw = Stopwatch::start_new();
    let mut asd = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => random_map(&mut map),
                Event::Quit { .. } => {
                    return;
                }
                _ => (),
            }
        }
        asd += 1;
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        map.iter().enumerate().for_each(|(i, val)| {
            match val {
                true => canvas.set_draw_color(Color::WHITE),
                false => canvas.set_draw_color(Color::BLACK),
            }
            let result = canvas.fill_rect(Rect::new(
                (((i / WIDTH) * CELLSIZE) + BORDERSIZE).try_into().unwrap(),
                (((i % WIDTH) * CELLSIZE) + BORDERSIZE).try_into().unwrap(),
                (CELLSIZE - (BORDERSIZE * 2)).try_into().unwrap(),
                (CELLSIZE - (BORDERSIZE * 2)).try_into().unwrap(),
            ));
            match result {
                Ok(_) => {}
                Err(_) => panic!("wtf lol"),
            }
        });
        let window = canvas.window_mut();
        let ss: &str = &asd.to_string();
        let result = window.set_title(ss);
        match result {
            Ok(_) => {}
            Err(_) => panic!("wtf lol"),
        }
        canvas.present();

        map = do_generation(&map);
        //print_map(&map);
        thread::sleep(wait_time);
    }
}
fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}
fn random_map(map: &mut [bool]) {
    for y in map.iter_mut() {
        *y = rand::thread_rng().gen_bool(0.40);
    }
}
fn get_index(row: usize, col: usize) -> usize {
    row * WIDTH + col
}
fn do_generation(map: &[bool]) -> Vec<bool> {
    let mut next_map = map.to_owned();
    next_map.par_iter_mut().enumerate().for_each(|(i, val)| {
        let neighbours = calc_neighbours(map, i / WIDTH, i % WIDTH);
        let cell = map[i];
        *val = match (cell, neighbours) {
            (true, x) if x < 2 => false,
            (true, x) if x > 3 => false,
            (false, 3) => true,
            (otherwise, _) => otherwise,
        }
    });
    next_map
}
fn calc_neighbours(map: &[bool], row: usize, col: usize) -> u8 {
    let mut count = 0;
    for delta_row in [WIDTH - 1, 0, 1].iter().cloned() {
        for delta_col in [WIDTH - 1, 0, 1].iter().cloned() {
            if delta_row == 0 && delta_col == 0 {
                continue;
            }
            let neighbor_row = (row + delta_row) % WIDTH;
            let neighbor_col = (col + delta_col) % WIDTH;
            let idx = neighbor_row * WIDTH + neighbor_col;
            count += map[idx] as u8;
        }
    }
    count
}
