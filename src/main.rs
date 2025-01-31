#![allow(dead_code)]

use std::io::stdin;
use std::{cmp, fmt::Display};

const ADJACENCY: [[isize; 2]; 6] = [[0, 1], [0, -1], [1, 0], [-1, 0], [1, 1], [-1, -1]];

fn main() {
    let mut m = String::new();
    let mut n = String::new();
    let mut mine_count = String::new();

    println!("Enter number of rows:");
    stdin().read_line(&mut m).expect("Failed to read rows");
    let m: usize = match m.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    println!("Enter number of columns:");
    stdin().read_line(&mut n).expect("Failed to read columns");
    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    println!("Enter number of mines:");
    stdin()
        .read_line(&mut mine_count)
        .expect("Failed to read columns");
    let mine_count: usize = match mine_count.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    let mut board = Board::new(m, n, mine_count);

    loop {
        board.display();

        let mut input = String::new();
        println!("Enter guess:");
        stdin().read_line(&mut input).expect("Failed to read input");

        let (x, y): (usize, usize) = match input
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(nums) if nums.len() == 2 => (nums[0], nums[1]),
            _ => {
                println!("Invalid input: Please enter two numbers.\nex. 1 2");
                continue;
            }
        };

        board.reveal_tile(&Coordinate { x, y });

        if board.game_over {
            break;
        }
    }

    println!("Game over!");
    board.reveal_grid();
    board.display();
}

#[derive(Debug)]
pub struct Board {
    grid: Vec<Vec<Tile>>,
    dim_x: usize,
    dim_y: usize,
    game_over: bool,
}

impl Board {
    pub fn new(m: usize, n: usize, mine_count: usize) -> Self {
        let mut total_tiles = m * n;
        let mut unassigned_mines = cmp::min(mine_count, total_tiles);

        let mut grid: Vec<Vec<Tile>> = (0..m).map(|_| vec![Tile::Concealed; n]).collect();

        // insert mines
        for x in grid.iter_mut() {
            for y in x.iter_mut() {
                // resevior sampling probability
                let is_mine = rand::random_ratio(unassigned_mines as u32, total_tiles as u32);
                if is_mine {
                    *y = Tile::ConcealedMine;
                    unassigned_mines -= 1;
                }

                total_tiles -= 1;
            }
        }

        Self {
            grid,
            dim_x: n,
            dim_y: m,
            game_over: false,
        }
    }

    /// Get the current value of a `Tile`.
    ///
    /// # Returns
    ///
    /// Returns None if coordinates are out of board bounds
    fn get_tile(&self, coord: &Coordinate) -> Option<Tile> {
        self.grid.get(coord.y).and_then(|r| r.get(coord.x).copied())
    }

    fn update_tile(&mut self, coord: &Coordinate, tile: Tile) {
        if let Some(row) = self.grid.get_mut(coord.y) {
            if let Some(t) = row.get_mut(coord.y) {
                *t = tile;
            }
        }
    }
    /// # Returns
    ///
    /// number of adjacent mines
    fn get_num_adj_mines(&self, coord: &Coordinate) -> usize {
        let mut adj_mines = 0usize;

        for adj in ADJACENCY.iter() {
            let x = coord.x.checked_add_signed(adj[0]);
            let y = coord.y.checked_add_signed(adj[1]);

            if let (Some(new_x), Some(new_y)) = (x, y) {
                if let Some(Tile::ConcealedMine) | Some(Tile::RevealedMine) =
                    self.get_tile(&Coordinate { x: new_x, y: new_y })
                {
                    adj_mines += 1;
                }
            }
        }

        adj_mines
    }

    /// Recursive BFS to update selected tile and relevant neighbors.
    /// Mutates board in place.
    fn bfs_zeros(&mut self, coord: &Coordinate) {
        if !matches!(self.get_tile(coord), Some(Tile::Concealed)) {
            return;
        }

        let tile_score = self.get_num_adj_mines(coord);

        if tile_score.eq(&0) {
            self.update_tile(coord, Tile::Checked(tile_score));
            for adj in ADJACENCY.iter() {
                let x = coord.x.checked_add_signed(adj[0]);
                let y = coord.y.checked_add_signed(adj[1]);

                if let (Some(new_x), Some(new_y)) = (x, y) {
                    if matches!(
                        self.get_tile(&Coordinate { x: new_x, y: new_y }),
                        Some(Tile::Concealed)
                    ) {
                        self.bfs_zeros(&Coordinate { x: new_x, y: new_y });
                    }
                }
            }
        }
    }

    fn reveal_tile(&mut self, coord: &Coordinate) {
        match self.get_tile(coord) {
            None => (),
            Some(tile) => match tile {
                Tile::Concealed => {
                    let tile_score = self.get_num_adj_mines(coord);
                    if tile_score.eq(&0) {
                        self.bfs_zeros(coord)
                    } else {
                        self.update_tile(coord, Tile::Checked(tile_score));
                    }
                }
                Tile::ConcealedMine => {
                    self.update_tile(coord, Tile::RevealedMine);
                    self.game_over = true;
                }
                _ => (),
            },
        };
    }

    fn reveal_grid(&mut self) {
        for x in 0..self.dim_x {
            for y in 0..self.dim_y {
                self.reveal_tile(&Coordinate { x, y });
            }
        }
    }

    fn display(&self) {
        println!();
        println!("{}", self);
        println!();
    }
}

#[derive(Clone, Debug, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
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
    Concealed,
    ConcealedMine,
    RevealedMine,
    Checked(usize),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            // Tile::Concealed | Tile::ConcealedMine => "#".to_string(),
            Tile::Concealed => "#".to_string(),
            Tile::ConcealedMine => "x".to_string(),
            Tile::RevealedMine => "X".to_string(),
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
