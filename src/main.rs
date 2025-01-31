#![allow(dead_code)]

use std::collections::HashSet;
use std::io::stdin;
use std::{cmp, fmt::Display};

const ADJACENCY: [[isize; 2]; 8] = [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
    [1, 1],
    [1, -1],
    [-1, 1],
    [-1, -1],
];

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
        board.display(true);

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

        board.pick_tile(&Coordinate { x, y });

        if board.game_over {
            break;
        }
    }

    println!("Game over!");
    board.reveal_grid();
    board.display(true);
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

        let mut grid: Vec<Vec<Tile>> = (0..m).map(|_| vec![Tile::Concealed(0); n]).collect();

        // insert mines
        for x in grid.iter_mut() {
            for y in x.iter_mut() {
                // resevior sampling probability
                let probability = rand::random_ratio(unassigned_mines as u32, total_tiles as u32);
                if probability {
                    *y = Tile::ConcealedMine;
                    unassigned_mines -= 1;
                }

                total_tiles -= 1;
            }
        }

        let mut board = Self {
            grid,
            dim_x: m,
            dim_y: n,
            game_over: false,
        };

        for i in 0..m {
            for j in 0..n {
                if let Tile::Concealed(_) = board.grid[i][j] {
                    let tile_value = board.get_num_adj_mines(&Coordinate { x: i, y: j });
                    board.grid[i][j] = Tile::Concealed(tile_value);
                }
            }
        }

        board
    }

    /// Get the current value of a `Tile`.
    ///
    /// # Returns
    ///
    /// Returns None if coordinates are out of board bounds
    fn get_tile(&self, coord: &Coordinate) -> Option<Tile> {
        self.grid.get(coord.y).and_then(|r| r.get(coord.x).copied())
    }

    fn reveal_tile(&mut self, coord: &Coordinate) {
        if let Some(row) = self.grid.get_mut(coord.y) {
            if let Some(t) = row.get_mut(coord.x) {
                *t = t.reveal();
            }
        }
    }

    /// # Returns
    ///
    /// number of adjacent mines
    fn get_num_adj_mines(&self, coord: &Coordinate) -> usize {
        let mut adj_mines = 0usize;

        for [dx, dy] in ADJACENCY.iter() {
            let new_x = coord.x as isize + dx;
            let new_y = coord.y as isize + dy;

            if new_x >= 0
                && new_x < self.dim_x as isize
                && new_y >= 0
                && new_y < self.dim_y as isize
                && matches!(
                    self.grid[new_x as usize][new_y as usize],
                    Tile::ConcealedMine | Tile::RevealedMine
                )
            {
                adj_mines += 1;
            }
        }

        adj_mines
    }

    /// Recursive BFS to update selected tile and relevant neighbors.
    /// Mutates board in place.
    fn bfs_zeros(&mut self, coord: &Coordinate, visited: &mut HashSet<Coordinate>) {
        if !visited.insert(*coord) {
            return;
        }

        let tile = self.get_tile(coord);

        if let Some(Tile::Concealed(0)) = tile {
            self.reveal_tile(coord);
            for adj in ADJACENCY.iter() {
                let x = coord.x.checked_add_signed(adj[0]);
                let y = coord.y.checked_add_signed(adj[1]);

                if let (Some(new_x), Some(new_y)) = (x, y) {
                    self.bfs_zeros(&Coordinate { x: new_x, y: new_y }, visited);
                }
            }
        }
    }

    fn pick_tile(&mut self, coord: &Coordinate) {
        match self.get_tile(coord) {
            None => (),
            Some(tile) => match tile {
                Tile::Concealed(value) => {
                    if value.eq(&0) {
                        let mut visited_tiles = HashSet::new();
                        self.bfs_zeros(coord, &mut visited_tiles);
                    } else {
                        self.reveal_tile(coord);
                    }
                }
                Tile::ConcealedMine => {
                    self.reveal_tile(coord);
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

    fn display(&self, debug: bool) {
        println!();

        print!("  ");
        for col in 0..self.dim_y {
            print!("{:2}", col);
        }
        println!();

        for (row_idx, row) in self.grid.iter().enumerate() {
            print!("{:2} ", row_idx);

            for tile in row {
                if debug {
                    print!("{} ", tile);
                } else {
                    match tile {
                        Tile::Concealed(_) | Tile::ConcealedMine => print!("# "),
                        Tile::RevealedMine => print!("X "),
                        Tile::Revealed(val) => print!("{} ", val),
                    }
                }
            }
            println!();
        }
        println!();
    }
}

#[derive(Hash, Clone, Debug, Copy, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for tile in row {
                write!(f, "{} ", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum Tile {
    Concealed(usize),
    ConcealedMine,
    RevealedMine,
    Revealed(usize),
}

impl Tile {
    fn reveal(self) -> Self {
        match self {
            Tile::Concealed(v) => Tile::Revealed(v),
            Tile::ConcealedMine | Tile::RevealedMine => Tile::RevealedMine,
            Tile::Revealed(v) => Tile::Revealed(v),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Tile::Concealed(_) | Tile::ConcealedMine => "#".to_string(),
            Tile::RevealedMine => "X".to_string(),
            Tile::Revealed(v) => v.to_string(),
        };
        write!(f, "{}", symbol)
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
