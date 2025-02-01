#![allow(dead_code)]

mod board;
mod tile;

use board::{Board, Coordinate};
use std::io::stdin;

fn main() {
    let (m, n, mine_count) = get_user_input();

    let mut board = Board::new(m, n, mine_count);

    loop {
        board.display(true);

        let (x, y) = get_guess();
        board.pick_tile(&Coordinate { x, y });

        if board.is_game_over() {
            break;
        }
    }

    println!("Game over!");
    board.reveal_grid();
    board.display(true);
}

fn get_user_input() -> (usize, usize, usize) {
    let mut input = String::new();

    println!("Enter number of rows:");
    stdin().read_line(&mut input).expect("Failed to read rows");
    let m = input.trim().parse().expect("Invalid number!");

    input.clear();
    println!("Enter number of columns:");
    stdin()
        .read_line(&mut input)
        .expect("Failed to read columns");
    let n = input.trim().parse().expect("Invalid number!");

    input.clear();
    println!("Enter number of mines:");
    stdin().read_line(&mut input).expect("Failed to read mines");
    let mine_count = input.trim().parse().expect("Invalid number!");

    (m, n, mine_count)
}

fn get_guess() -> (usize, usize) {
    loop {
        let mut input = String::new();
        println!("Enter guess:");
        stdin().read_line(&mut input).expect("Failed to read input");

        if let Ok(nums) = input
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
        {
            if nums.len() == 2 {
                return (nums[0], nums[1]);
            }
        }

        println!("Invalid input: Please enter two numbers.\nex. 1 2");
    }
}
