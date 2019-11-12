use std::process;

mod cell;
mod board;
mod player;
mod utils;

use self::board::*;
use self::cell::*;
use self::player::*;
use self::utils::*;

const BOARD_SIZE: usize = 3;

fn main() {
    println!("Tic-tac-toe");
    println!("===========");
    println!();

    let mut board = Board::new(BOARD_SIZE);
    let players = vec![
        Player { name: "Player 1".to_string(), symbol: &Cell::X },
        Player { name: "Player 2".to_string(), symbol: &Cell::O },
    ];

    loop {
        for player in players.iter() {
            println!("{}", player);
            println!();
            println!("{}", board);
            println!();

            let mut position: (usize, usize);

            loop {
                println!("Row?");
                let row = request_coordinate();
                println!("Col?");
                let col = request_coordinate();
                position = (row, col);

                if !board.is_position_valid(position) {
                    println!("Invalid position");
                    continue;
                }

                if !board.is_position_free(position) {
                    println!("Position already taken");
                    continue;
                }

                break;
            }

            board.update(position, player.symbol);

            println!();
            if board.finished() {
                println!("===========");
                println!();
                println!("{}", board);
                println!();
                println!("{} wins!", player.name);
                process::exit(1);
            }

            if board.full() {
                println!("===========");
                println!();
                println!("{}", board);
                println!();
                println!("Board full!");
                process::exit(1);
            }
            println!("-----------");
            println!();
        }
    }
}
