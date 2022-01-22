mod grid;

use grid::Grid;
use pancurses::{
    curs_set, endwin, init_pair, initscr, start_color, Input, COLOR_BLACK, COLOR_GREEN, COLOR_RED,
    COLOR_WHITE,
};
use std::env;

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
        },
        None => Difficulty::Medium,
    };

    // Set window background
    for x in 0..window.get_max_x() {
        for y in 0..window.get_max_y() {
            window.mvprintw(y, x, " ");
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

    // Number of flags
    let mut flags = game_grid.get_flags();

    // Number of mines
    let mines = game_grid.get_mines();

    // GAME LOOP //
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
                let safe = game_grid.add_to_seen(x - gap_x, y - gap_y, 1);
                if !safe {
                    break;
                }
            }
            Some(Input::Character('f')) => {
                game_grid.flag(x - gap_x, y - gap_y);
                flags = game_grid.get_flags();
            }
            _ => (),
        }

        if game_grid.check_win() {
            lost = false;
            break;
        }

        window.mvprintw(1, 0, format!("FLAGS: {}, MINES: {}", flags, mines));
        game_grid.display_grid(&window, x, y, gap_x, gap_y);
    }
    endwin();

    // Print GAME OVER if player lost
    if lost {
        println!("GAME OVER");
    } else {
        println!("VICTORY");
    }
}
