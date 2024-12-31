// 1. Take user input for move - row, column. game keeps track of who's turn it is
// 2. print game board
// 3. break if the board has either been won, or is full
//  3a. break if there are no remaining win conditions
//  .1 (1 space)
//  ....2 (4 spaces)
//  .......3 (7 spaces) 3n-2
//  up - ^[[A
//  left - ^[[C
//  right - ^[[D
//  down - ^[[B
//  [[x x x] [x x x] [x x x]]

use console::Term;
use std::usize;

struct GameInfo {
    board: [[char; 3]; 3],
    noughts_turn: bool,
    selected_column: usize,
    selected_row: usize,
}

fn main() {
    use console::Key::*;
    let mut play: bool = true;
    let mut noughts_turn: bool = true;
    let mut board: [[char; 3]; 3] = [['*'; 3]; 3];
    let stdout = Term::buffered_stdout();

    let mut selected_column = 0;
    let mut selected_row = 0;

    let mut game = GameInfo { board, noughts_turn, selected_column, selected_row };

    print_frame(&mut game);

    'game_loop: while play {
        let mut selected = false;

        while !selected {
            if let Ok(character) = stdout.read_key() {
                match character {
                    ArrowDown => if game.selected_row < 2 {game.selected_row += 1},
                    ArrowUp => if game.selected_row > 0 {game.selected_row -= 1},
                    ArrowLeft => if game.selected_column > 0 {game.selected_column -= 1},
                    ArrowRight => if game.selected_column < 2 {game.selected_column += 1},
                    Enter => if game.board[game.selected_row][game.selected_column] == '*' { selected = true },
                    _ => break 'game_loop,
                }
            }
            print_frame(&mut game);
        }

        let active_cell = &mut game.board[game.selected_row][game.selected_column];

        if game.noughts_turn {
            *active_cell = 'o'; // nought
        } else {
            *active_cell = 'x'; // cross
        }

        game.noughts_turn = !game.noughts_turn; // toggle turn

        print_frame(&mut game);
    }
}

fn print_frame(game: &mut GameInfo) {
    print!("{esc}c", esc = 27 as char); // clear terminal
    
    // turn intro
    if game.noughts_turn {
        println!("Noughts turn!");
    } else {
        println!("Crosses turn!");
    }




    for i in 0..=2 {
        for j in 0..=2 {
            if j == game.selected_column && i == game.selected_row {
                print!("[{}]", game.board[i][j]);
            } else {
                print!(" {} ", game.board[i][j]);
            }
        }
        println!("");
    }
}
