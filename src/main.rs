// THIS IS A VERY SIMPLE PROGRAMM TO LEARN SOME
// OF THE RUST LANGUAGE DETAILS

use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_initial_position(filename: &String) -> [[i32;9];9]{
    // this code assumes correct formating rn
    let mut board = [[0; 9]; 9];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut i: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors
        let value_iter = line.split(',');
        let mut j: usize = 0;
        for elem in value_iter {
            let elem: i32 = elem.trim().parse().unwrap();
            board[i][j] = elem;
            j += 1;
        }
        i += 1;
    }
    board
}

fn print_board(board: &[[i32;9];9]) {
    for i in 0..board.len() {
        println!("|{:?}|", board[i])
    }
}

fn solve(board: &mut [[i32;9];9]) -> bool {
    // next empty slot
    let (row_free, col_free) = find_empty(board);
    if row_free == -1 {
        println!("We are done!");
        print_board(board);
        return true;
    }
    // find option to this free slot
    for opt in 1..10 {
        let row = row_free as usize;
        let col = col_free as usize;
        if valid(board, opt, (row, col)){
            board[row][col] = opt;
            // recursion until above condition
            // not free cells are complete
            if solve(board) {
                return true;
            }
            // don't know if this reset is needed
            board[row][col] = 0;
        }
    }
    false
}

fn valid(board: &mut [[i32;9];9], opt: i32, pos: (usize, usize)) -> bool {
    let row = pos.0;
    let col = pos.1;
    // row check
    for c in 0..9 {
        if board[row][c] == opt {
            return false;
        }
    }
    //col check
    for r in 0..9 {
        if board[r][col] == opt {
            return false;
        }
    }

    // 3x3 check
    let horizontal_cuadrant = col / 3;
    let vertical_cuadrant = row / 3;
    // iterate over the 3x3 cuadrant
    for i in 3*horizontal_cuadrant..3*horizontal_cuadrant+3 {
        for j in 3*vertical_cuadrant..3*vertical_cuadrant+3{
            // vertical first... is which line is it
            if board[j][i] == opt {
                return false;
            }
        }
    }

    true
    
}

fn find_empty(board: &mut [[i32;9];9]) -> (isize, isize) {
    // Iterators use usize, have to convert them for this purpose
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if board[row][col] == 0 {
                let r = row as isize;
                let c = col as isize;
                return (r, c)
            }
        }
    }
    (-1, -1)
}


fn main() {
    let filename = String::from("sudokus/not_fun.txt");
    let mut board = read_initial_position(&filename);
    println!("Initial setup");
    print_board(&board);
    solve(&mut board);
}
