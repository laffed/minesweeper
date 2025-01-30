use std::{cmp, fmt::Display};

fn main() {
    let m = Minesweeper::new(4, 7, 4);
    println!("{}", m);
}

#[derive(Debug)]
struct Minesweeper {
    board: Vec<Vec<Tile>>,
}

impl Minesweeper {
    fn new(n: u32, m: u32, mine_count: u32) -> Self {
        let mut total_tiles = n * m;
        let mut unassigned_mines = cmp::min(mine_count, total_tiles);

        let mut board: Vec<Vec<Tile>> = (0..n).map(|_| vec![Tile::Empty; m as usize]).collect();
        for x in board.iter_mut() {
            for y in x.iter_mut() {
                // resevior sampling probability
                let is_mine = rand::random_ratio(unassigned_mines, total_tiles);
                if is_mine {
                    *y = Tile::Mine;
                    unassigned_mines -= 1;
                }

                total_tiles -= 1;
            }
        }

        Self { board }
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for cell in row {
                write!(f, "{} ", cell)?; // Print each cell with space separator
            }
            writeln!(f)?; // Newline after each row
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
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
