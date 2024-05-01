use std::{array, cell};

use macroquad::{miniquad::window, prelude::*};

const WINDOW_HEIGHT: i32 = 1000;
const WINDOW_WIDTH: i32 = 1000;
const CELL_SIZE: i32 = 20;
const GRID_THICKNESS: f32 = 1.0;

const GRID_WIDTH: usize = (WINDOW_WIDTH / CELL_SIZE) as usize;
const GRID_HEIGHT: usize = (WINDOW_HEIGHT / CELL_SIZE) as usize;
const CELL_ALIVE: i32 = 1;
const CELL_DEAD: i32 = 0;

struct Cell {
    x: u32,
    y: u32,
    alive: bool,
}

fn draw_grid(width: i32, height: i32) {
    for i in (0..(screen_width() as u32)).step_by(width as usize) {
        draw_line(
            i as f32,
            0_f32,
            i as f32,
            screen_height(),
            GRID_THICKNESS,
            GOLD,
        );
    }
    for i in (0..(screen_height() as u32)).step_by(height as usize) {
        draw_line(
            0_f32,
            i as f32,
            screen_width(),
            i as f32,
            GRID_THICKNESS,
            GOLD,
        );
    }
}

fn update_cells(cells: &[[i32; GRID_WIDTH]; GRID_HEIGHT]) {}

async fn game() {
    let mut cells: [[i32; GRID_WIDTH]; GRID_HEIGHT] = [[CELL_DEAD; GRID_WIDTH]; GRID_HEIGHT];
    let mut cells_after: [[i32; GRID_WIDTH]; GRID_HEIGHT] = [[CELL_DEAD; GRID_WIDTH]; GRID_HEIGHT];

    // generate start positions for alive cells
    for _ in 0..25 {
        let rx = rand::gen_range(0, GRID_WIDTH);
        let ry = rand::gen_range(0, GRID_HEIGHT);

        cells[rx][ry] = CELL_ALIVE;
    }

    loop {
        frame(&cells);

        next_frame().await
    }

    // let
}

fn draw_cell(x: i32, y: i32) {
    draw_rectangle(
        (x * CELL_SIZE) as f32,
        (y * CELL_SIZE) as f32,
        CELL_SIZE as f32,
        CELL_SIZE as f32,
        GOLD,
    );
}

fn frame(cells: &[[i32; GRID_WIDTH]; GRID_HEIGHT]) {
    clear_background(BLACK);

    draw_grid(CELL_SIZE, CELL_SIZE);

    for (i, row) in cells.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if cells[i][j] == CELL_ALIVE {
                draw_cell(i as i32, j as i32);
            }
        }
    }

    // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game Of Life".to_string(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    game().await
}
