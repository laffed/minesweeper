#![allow(dead_code)]

mod board;
mod tile;

use board::{Board, Coordinate};
use std::io::stdin;

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

        if board.is_game_over() {
            break;
        }
    }

    println!("Game over!");
    board.reveal_grid();
    board.display(true);
}

#[cfg(test)]
mod tests {
    // TODO
}
