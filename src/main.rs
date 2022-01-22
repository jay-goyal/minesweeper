mod grid;

use std::env;
use pancurses::{COLOR_BLACK, COLOR_GREEN, COLOR_RED, COLOR_WHITE, curs_set, endwin, init_pair, initscr, Input, start_color};
use grid::Grid;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

fn main() {
    // Getting Window
    let window = initscr();
    window.keypad(true);
    window.refresh();
    window.nodelay(true);
    curs_set(1);
    window.timeout(100);

    // Getting difficulty
    let difficulty = match env::args().nth(1) {
        Some(diff) => match diff.to_lowercase().as_str() {
            "easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            _ => Difficulty::Medium,
        }
        None => Difficulty::Medium,
    };

    // Set window background
    for x in 0..window.get_max_x() {
        for y in 0..window.get_max_y() {
            window.mvprintw(y,x, ' ');
        }
    }

    // Setting Colors
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    init_pair(3, COLOR_RED, COLOR_BLACK);

    // Generating the game grid
    let mut game_grid = match difficulty {
        Difficulty::Easy => Grid::new(10, 30, difficulty),
        Difficulty::Medium => Grid::new(20, 60, difficulty),
        Difficulty::Hard => Grid::new(30, 90, difficulty),
    };

    // Setting gaps so that grid is centered
    let gap_x = (window.get_max_x() - game_grid.get_width()) / 2;
    let gap_y = (window.get_max_y() - game_grid.get_height()) / 2;

    let mut x = game_grid.get_width() / 2 + gap_x;
    let mut y = game_grid.get_height() / 2 + gap_y;

    // Lose state
    let mut lost = true;

    loop {
        // Moving the cursor
        match window.getch() {
            Some(Input::KeyRight) => {
                if x < game_grid.get_width() + gap_x {
                    x += 1;
                }
            }
            Some(Input::KeyLeft) => {
                if x > gap_x + 1 {
                    x -= 1;
                }
            }
            Some(Input::KeyDown) => {
                if y < game_grid.get_height() + gap_y {
                    y += 1;
                }
            }
            Some(Input::KeyUp) => {
                if y > gap_y + 1 {
                    y -= 1;
                }
            }
            Some(Input::Character(' ')) => {
                let safe = game_grid.add_to_seen(x - gap_x, y - gap_y);
                if !safe {
                    break;
                }
            }
            _ => (),
        }

        game_grid.display_grid(&window, x, y, gap_x, gap_y);
    }
    endwin();
    if lost {
        println!("GAME OVER")
    }
}