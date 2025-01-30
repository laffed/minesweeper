#![allow(dead_code)]

use std::{cmp, fmt::Display};

fn main() {
    let m = Minesweeper::new(4, 7, 4);
    println!("{}", m);
}

#[derive(Debug)]
pub struct Minesweeper {
    board: Vec<Vec<Tile>>,
    dim_x: usize,
    dim_y: usize,
}

impl Minesweeper {
    pub fn new(m: usize, n: usize, mine_count: usize) -> Self {
        let mut total_tiles = m * n;
        let mut unassigned_mines = cmp::min(mine_count, total_tiles);

        let mut board: Vec<Vec<Tile>> = (0..m).map(|_| vec![Tile::Empty; n]).collect();

        // insert mines
        for x in board.iter_mut() {
            for y in x.iter_mut() {
                // resevior sampling probability
                let is_mine = rand::random_ratio(unassigned_mines as u32, total_tiles as u32);
                if is_mine {
                    *y = Tile::Mine;
                    unassigned_mines -= 1;
                }

                total_tiles -= 1;
            }
        }

        Self {
            board,
            dim_x: n,
            dim_y: m,
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        self.board.get(y).and_then(|r| r.get(x).copied())
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for tile in row {
                write!(f, "{} ", tile)?; // Print each cell with space separator
            }
            writeln!(f)?; // Newline after each row
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Copy)]
enum Tile {
    Empty,
    Mine,
    Checked(usize),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Tile::Empty => "#".to_string(),
            Tile::Mine => "X".to_string(),
            Tile::Checked(v) => v.to_string(),
        };
        write!(f, "{}", symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
