use std::path::Iter;

use macroquad::{prelude::*, time};

const SAND_SIZE : f32 = 4.0;
const PHYSICS_TICK_MILLIS : f32 = 0.008;

#[macroquad::main("Main")]
async fn main() {    
    let mut blocks : Vec<Vec<u32>> = Vec::new();

    let sand_colors = vec![
        Color::from_hex(0xffe0ab),
        Color::from_hex(0xf3ce93), 
        Color::from_hex(0xd6b588), 
        Color::from_hex(0xffe0ab), 
        Color::from_hex(0xdab984), 
        Color::from_hex(0x5b270b)
    ];

    let mut acc_time = 0.0;

    // 0 == SAND
    // 1 == WOOD
    // 2 == ClEAR
    let mut mode = 0;

    loop {
        let width = screen_width();
        let height = screen_height();
        let time = time::get_time() as f32;

        let b_width = (width / SAND_SIZE).ceil() as u32;
        let b_height = (height / SAND_SIZE).ceil() as u32;

        blocks = (0..b_width).map(|x| (0..b_height).map(|y| {
            if (x as usize) < blocks.len() && (y as usize) < blocks[0].len() {
                blocks[x as usize][y as usize]
            } else {
                0
            }
        }).collect()).collect();

        clear_background(BLACK);

        let (x, y) = mouse_position();


        //clear
        if is_key_down(KeyCode::C) {
            blocks = (0..b_width).map(|x| (0..b_height).map(|y| { 0 }).collect()).collect();
            mode = 0;
        }

        if is_key_down(KeyCode::W) {
            mode = 1;
        }

        if is_key_down(KeyCode::S) {
            mode = 0;
        }

        if is_key_down(KeyCode::E) {
            mode = 2;
        }


        if acc_time > PHYSICS_TICK_MILLIS {
            
            // draw additional pixels
            if is_mouse_button_down(MouseButton::Left) {
                // draw_rectangle(mouse_position().0, mouse_position().1, 10.0, 10.0, WHITE);
                let (x, y) = ((x / SAND_SIZE).floor() as usize, (y / SAND_SIZE).floor() as usize);

                for i in -2..2 {
                    for j in -2..2 {
                        let X = ((x as i32) + i) as usize;
                        let Y = ((y as i32) + j) as usize;

                        match blocks.get(X) {   
                            Some(s) => {
                                match s.get(Y) {
                                    Some(q) => {
                                        if mode == 0 {
                                            if *q == 0 && rand::gen_range(0, 2) != 1 {
                                                blocks[X][Y] = rand::gen_range(1, 6)
                                            }
                                        } else if mode == 1 {
                                            blocks[X][Y] = 6;
                                        } else if mode == 2 {
                                            blocks[X][Y] = 0;
                                        }
                                    },
                                    // Some(q) => if *q == 0 {blocks[X][Y] = 1},
                                    None => (),
                                }
                            }
                            None => ()
                        }
                        // blocks[x][y] = rand::gen_range(1, 6); 
                    }
                }
            }

            let row_iter: Vec<u32> = if rand::gen_range(0, 2) == 1 {
                (0..b_width-1).collect()
            } else {
                (0..b_width-1).rev().collect()
            };

            // game of life
            for y in (0..b_height).rev() {
                for x in row_iter.iter(){
                    let x = *x as usize;
                    let y = y as usize;
                    if blocks[x][y] != 0 && blocks[x][y] != 6 && y != (b_height as usize)-1 {
                        let left = x != 0 && blocks[x-1][y+1] == 0;
                        let right = x != (b_width as usize) - 1 && blocks[x+1][y+1] == 0;

                        if blocks[x][y+1] == 0 {
                            blocks[x][y+1] = blocks[x][y];
                            blocks[x][y] = 0;
                        } else if left && right{
                            let offset: i32 = if rand::gen_range(0, 2) == 1 {1} else {-1};
                            blocks[(x as i32 + offset) as usize][y+1] = blocks[x][y];
                            blocks[x][y] = 0;
                        } else if left {
                            blocks[x-1][y+1] = blocks[x][y];
                            blocks[x][y] = 0;
                        } else if right {
                            blocks[x+1][y+1] = blocks[x][y];
                            blocks[x][y] = 0;
                        }
                    }
                }
            }

            acc_time = 0.0;
        }
        

        for x in 0..b_width {
            for y in 0..b_height {
                if blocks[x as usize][y as usize] != 0 {
                    draw_rectangle(x as f32  * SAND_SIZE, y as f32 * SAND_SIZE, SAND_SIZE, SAND_SIZE, sand_colors[(blocks[x as usize][y as usize]-1) as usize]);
                }
            }
        }

        acc_time += time::get_frame_time();
        next_frame().await
    }
}