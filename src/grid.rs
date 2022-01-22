use crate::Difficulty;
use pancurses::{Window, ACS_BLOCK, ACS_CKBOARD, COLOR_PAIR};
use rand::Rng;

pub struct Grid {
    height: i32,
    width: i32,
    flags: i32,
    mines: i32,
    mine_loc: Vec<(i32, i32)>,
    seen: Vec<(i32, i32)>,
    flagged: Vec<(i32, i32)>,
}

impl Grid {
    pub fn new(height: i32, width: i32, difficulty: Difficulty) -> Grid {
        let mut grid = Grid {
            height,
            width,
            flags: 0,
            mines: 0,
            mine_loc: Vec::new(),
            seen: Vec::new(),
            flagged: Vec::new(),
        };
        grid.gen_mines(difficulty);
        return grid;
    }

    pub fn get_height(&mut self) -> i32 {
        return self.height;
    }

    pub fn get_width(&mut self) -> i32 {
        return self.width;
    }

    pub fn get_flags(&mut self) -> i32 {
        return self.flags;
    }

    pub fn get_mines(&mut self) -> i32 {
        return self.mines;
    }

    pub fn add_to_seen(&mut self, x: i32, y: i32) -> bool {
        // Return false if user clicks on mine
        if self.mine_loc.contains(&(x, y)) {
            return false;
        }
        if !(self.seen.contains(&(x, y))) {
            self.seen.push((x, y));
        }
        return true;
    }

    pub fn flag(&mut self, x: i32, y: i32) {
        if self.flagged.contains(&(x, y)) {
            self.flagged.retain(|&a| a != (x, y));
            self.flags -= 1;
        } else {
            self.flagged.push((x, y));
            self.flags += 1;
        }
    }

    pub fn display_grid(
        &mut self,
        window: &Window,
        init_x: i32,
        init_y: i32,
        gap_x: i32,
        gap_y: i32,
    ) {
        for x in 1..=self.width {
            for y in 1..=self.height {
                if self.flagged.contains(&(x, y)) {
                    window.attron(COLOR_PAIR(3));
                    window.mvaddch(y + gap_y, x + gap_x, ACS_BLOCK());
                } else if self.mine_loc.contains(&(x, y)) {
                    // Printing using red color
                    window.attron(COLOR_PAIR(3));
                    window.mvaddch(y + gap_y, x + gap_x, ACS_CKBOARD());
                } else if self.seen.contains(&(x, y)) {
                    let num_mines = self.get_surr_mines(x, y);

                    // Printing using green color
                    window.attron(COLOR_PAIR(2));

                    // Printing Number of mines if it is more than 0
                    if num_mines > 0 {
                        window.mvprintw(y + gap_y, x + gap_x, format!("{}", num_mines));
                    } else {
                        window.mvaddch(y + gap_y, x + gap_x, ACS_BLOCK());
                    }
                } else {
                    // Printing using white color
                    window.attron(COLOR_PAIR(1));
                    window.mvaddch(y + gap_y, x + gap_x, ACS_BLOCK());
                }
            }
        }
        window.mv(init_y, init_x);
        window.refresh();
    }

    pub fn check_win(&self) -> bool {
        if self.flagged.len() == self.mine_loc.len() {
            let mut eq = true;
            for i in 0..self.flagged.len() {
                if !self.mine_loc.contains(&self.flagged[i]) {
                    eq = false;
                }
            }
            if eq {
                return true;
            }
        }

        let temp: Vec<(i32, i32)> = self
            .seen
            .iter()
            .cloned()
            .chain(self.mine_loc.iter().cloned())
            .collect();

        if temp.len() == (self.height * self.width) as usize {
            return true;
        }

        return false;
    }

    fn gen_mines(&mut self, dif: Difficulty) {
        let num_mine = match dif {
            Difficulty::Easy => 10,
            Difficulty::Medium => 35,
            Difficulty::Hard => 110,
        };

        loop {
            let x = rand::thread_rng().gen_range(1..self.width + 1);
            let y = rand::thread_rng().gen_range(1..self.height + 1);
            if !(self.mine_loc.contains(&(x, y))) {
                self.mine_loc.push((x, y));
            }
            if self.mine_loc.len() == num_mine {
                break;
            }
        }

        self.mines = num_mine as i32;
    }

    fn get_surr_mines(&mut self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        if self.mine_loc.contains(&(x - 1, y)) {
            count += 1;
        }
        if self.mine_loc.contains(&(x - 1, y - 1)) {
            count += 1;
        }
        if self.mine_loc.contains(&(x - 1, y + 1)) {
            count += 1;
        }
        if self.mine_loc.contains(&(x, y + 1)) {
            count += 1;
        }
        if self.mine_loc.contains(&(x, y - 1)) {
            count += 1;
        }
        if self.mine_loc.contains(&(x + 1, y)) {
            count += 1;
        }
        if self.mine_loc.contains(&(x + 1, y - 1)) {
            count += 1;
        }
        if self.mine_loc.contains(&(x + 1, y + 1)) {
            count += 1;
        }
        return count;
    }
}
