use macroquad::{prelude::*, time};

const SAND_SIZE : f32 = 10.0;

#[macroquad::main("Main")]
async fn main() {    
    let mut blocks : Vec<Vec<u32>> = Vec::new();

    let sand_colors = vec![
        Color::from_hex(0xffe0ab),
        Color::from_hex(0xf3ce93), 
        Color::from_hex(0xedc9af), 
        Color::from_hex(0xffe0ab), 
        Color::from_hex(0xdab984), 
    ];



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
        }

        // draw additional pixels
        if is_mouse_button_down(MouseButton::Left) {
            // draw_rectangle(mouse_position().0, mouse_position().1, 10.0, 10.0, WHITE);
            let (x, y) = ((x / SAND_SIZE).floor() as usize, (y / SAND_SIZE).floor() as usize);

            for i in -1..1 {
                for j in -1..1 {
                    let X = ((x as i32) + i) as usize;
                    let Y = ((y as i32) + j) as usize;

                    match blocks.get(X) {
                        Some(s) => {
                            match s.get(Y) {
                                Some(q) => if *q == 0 {blocks[X][Y] = rand::gen_range(1, 6)},
                                None => (),
                            }
                        }
                        None => ()
                    }
                    // blocks[x][y] = rand::gen_range(1, 6); 
                }
            }
        }



        // game of life
        for x in 0..b_width {
            for y in (0..b_height).rev() {
                let x = x as usize;
                let y = y as usize;
                if blocks[x][y] != 0 && y != (b_height as usize)-1 && blocks[x][y+1] == 0 {
                    blocks[x][y+1] = blocks[x][y];
                    blocks[x][y] = 0;
                }
            }
        }

        

        for x in 0..b_width {
            for y in 0..b_height {
                if blocks[x as usize][y as usize] != 0 {
                    draw_rectangle(x as f32  * SAND_SIZE, y as f32 * SAND_SIZE, SAND_SIZE, SAND_SIZE, sand_colors[(blocks[x as usize][y as usize]-1) as usize]);
                }
            }
        }

        next_frame().await
    }
}