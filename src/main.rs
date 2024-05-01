use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;

const WINDOW_HEIGHT: i32 = 1000;
const WINDOW_WIDTH: i32 = 1000;
const CELL_SIZE: i32 = 10;
const GRID_THICKNESS: f32 = 1.0;

const GRID_WIDTH: usize = (WINDOW_WIDTH / CELL_SIZE) as usize;
const GRID_HEIGHT: usize = (WINDOW_HEIGHT / CELL_SIZE) as usize;
const CELL_ALIVE: i32 = 1;
const CELL_DEAD: i32 = 0;
const FPS: i32 = 60;

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

async fn game() {
    let mut cells: [[i32; GRID_WIDTH]; GRID_HEIGHT] = [[CELL_DEAD; GRID_WIDTH]; GRID_HEIGHT];

    // generate start positions for alive cells
    for _ in 0..5000 {
        let rx = rand::gen_range(0, GRID_WIDTH);
        let ry = rand::gen_range(0, GRID_HEIGHT);

        cells[rx][ry] = CELL_ALIVE;
    }

    loop {
        clear_background(BLACK);
        // draw_grid(CELL_SIZE, CELL_SIZE);
        let new_cells = draw_and_mutate_cells(&cells);
        cells = new_cells;
        sleep(Duration::from_secs_f32(1.0 / FPS as f32));
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

fn get_cell_neighbours(cell_x: i32, cell_y: i32) -> Vec<(i32, i32)> {
    let mut neighbours = vec![];
    for x in cell_x - 1..=cell_x + 1 {
        for y in cell_y - 1..=cell_y + 1 {
            if ((x, y) == (cell_x, cell_y))
                | (x < 0)
                | (y < 0)
                | (x >= GRID_WIDTH as i32)
                | (y >= GRID_HEIGHT as i32)
            {
                continue;
            }
            neighbours.push((x, y))
        }
    }
    return neighbours;
}

fn get_alive_neighbours_count(
    neighbours: Vec<(i32, i32)>,
    cells: &[[i32; GRID_WIDTH]; GRID_HEIGHT],
) -> i32 {
    let mut count = 0;
    for neighbour in neighbours.iter() {
        if cells[neighbour.0 as usize][neighbour.1 as usize] == CELL_ALIVE {
            count += 1;
        }
    }
    return count;
}

fn draw_and_mutate_cells(
    cells: &[[i32; GRID_WIDTH]; GRID_HEIGHT],
) -> [[i32; GRID_WIDTH]; GRID_HEIGHT] {
    let mut new_cells = cells.clone();

    for (i, row) in cells.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == CELL_ALIVE {
                draw_cell(i as i32, j as i32);
            }

            let neighbours = get_cell_neighbours(i as i32, j as i32);
            let alive_count = get_alive_neighbours_count(neighbours, &cells);

            if (alive_count < 2) | (alive_count > 3) {
                new_cells[i][j] = CELL_DEAD;
            } else if (alive_count == 3) | ((alive_count == 2) & (*cell == CELL_ALIVE)) {
                new_cells[i][j] = CELL_ALIVE;
            }
        }
    }
    return new_cells;
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
