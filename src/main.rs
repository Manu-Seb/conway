use conway_game_of_life::Cell;
use std::{isize, thread::sleep, time};

use macroquad::prelude::*;

#[cfg(feature = "terminal")]
fn main() {
    terminal_display();
}

#[cfg(feature = "window")]
#[macroquad::main("MyGame")]
async fn main() {
    let width = screen_width(); //800
    let height = screen_height(); //600
    let delay = 0.05;
    let mut time = get_time();
    let tile = width / 300.;
    let rows = height / tile;
    let cols = width / tile;
    let mut grid: Vec<Vec<Cell>> = (0..rows as u32)
        .map(|_| (0..cols as u32).map(|_| Cell::init_cell()).collect())
        .collect();

    let mut new_grid: Vec<Vec<Cell>> = (0..rows as u32)
        .map(|_| (0..cols as u32).map(|_| Cell::new(false, 0)).collect())
        .collect();
    loop {
        clear_background(BLACK);

        for i in 0..rows as usize {
            for j in 0..cols as usize {
                if grid[i][j].alive() {
                    let color = age_to_color(grid[i][j].age());
                    draw_rectangle(j as f32 * tile, i as f32 * tile, tile, tile, color);
                }
            }
        }
        if get_time() - time > delay {
            time = get_time();
            next_generation(&mut grid, &mut new_grid);
        }

        next_frame().await
    }
}
fn age_to_color(age: u32) -> Color {
    let max_age = 20; // After 20 generations, color stops changing
    let t = (age.min(max_age) as f32) / (max_age as f32);

    // interpolate from blue → green → yellow → red
    Color::new(
        t,       // red increases
        1.0 - t, // green decreases
        0.2,     // constant blue tint
        1.0,
    )
}
fn terminal_display() {
    let rows = 50;
    let cols = 80;

    let mut grid: Vec<Vec<Cell>> = (0..rows)
        .map(|_| (0..cols).map(|_| Cell::init_cell()).collect())
        .collect();

    let mut new_grid: Vec<Vec<Cell>> = (0..rows)
        .map(|_| (0..cols).map(|_| Cell::new(false, 0)).collect())
        .collect();
    loop {
        display_grid(&grid);
        next_generation(&mut grid, &mut new_grid);
        sleep(time::Duration::from_millis(50));
    }
}

fn display_grid(grid: &Vec<Vec<Cell>>) {
    println!("The gen ");
    for i in grid {
        for j in i {
            if j.alive() {
                print!("▀");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn next_generation(grid: &mut Vec<Vec<Cell>>, new_grid: &mut Vec<Vec<Cell>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    let delrow = [-1, -1, -1, 0, 0, 1, 1, 1];
    let delcol = [-1, 0, 1, -1, 1, -1, 0, 1];
    for i in 0..rows {
        for j in 0..cols {
            let mut count = 0;
            for k in 0..8 {
                let newrow = i as isize + delrow[k];
                let newcol = j as isize + delcol[k];
                if newrow < 0 || newrow >= rows as isize || newcol < 0 || newcol >= cols as isize {
                    continue;
                }
                let newrow = newrow as usize;
                let newcol = newcol as usize;
                if grid[newrow][newcol].alive() {
                    count += 1;
                }
            }
            new_grid[i][j].check_rules(grid[i][j].alive(), count);
        }
    }
    std::mem::swap(grid, new_grid);
}
