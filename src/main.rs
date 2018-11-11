use std::io;
use std::process;

mod cell;
mod board;
mod player;
mod utils;

use self::board::*;
use self::cell::*;
use self::player::*;

const BOARD_SIZE: usize = 3;

fn main() {
    println!("Tic-tac-toe");

    let mut board = Board::new(BOARD_SIZE);
    let players = vec![
        Player { name: "Player 1".to_string(), symbol: &Cell::X },
        Player { name: "Player 2".to_string(), symbol: &Cell::O },
    ];

    let fetch_coordinate = || -> usize {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input: usize = input.trim().parse()
            .expect("Please type a number!");

        input
    };

    loop {
        for player in players.iter() {
            println!("{}", player);
            println!("{}", board);

            let mut position: (usize, usize);

            loop {
                let row = fetch_coordinate();
                let col = fetch_coordinate();
                position = (row, col);

                if !board.is_position_valid(position) {
                    println!("Invalid position");
                    continue;
                }

                if !board.is_position_free(position) {
                    println!("Position is already taken");
                    continue;
                }

                break;
            }

            board.update(position, player.symbol);

            if board.finished() {
                println!("Finished!");
                println!("{} wins!", player.name);
                println!("{}", board);
                process::exit(1);
            }

            if board.full() {
                println!("Board full!");
                println!("{}", board);
                process::exit(1);
            }
        }
    }
}
