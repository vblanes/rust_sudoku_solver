// THIS IS A VERY SIMPLE PROGRAMM TO LEARN SOME
// OF THE RUST LANGUAGE DETAILS

// read the sudoku file
fn read_initial_position(){
    println!("TODO");
}

fn solve(board: &mut [[i32;9];9]) -> bool {
    // next empty slot
    let (row_free, col_free) = find_empty(board);
    if row_free == -1 {
        println!("We are done!");
        println!("{:?}", board);
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
    let mut board: [[i32; 9];9 ] =
        [[0, 0, 0, 2, 6, 0, 7, 0, 1],
        [6, 8, 0, 0, 7, 0, 0, 9, 0],
        [1, 9, 0, 0, 0, 4, 5, 0, 0],
        [8, 2, 0, 1, 0, 0, 0, 4, 0],
        [0, 0, 4, 6, 0, 2, 9, 0, 0],
        [0, 5, 0, 0, 0, 3, 0, 2, 8],
        [0, 0, 9, 3, 0, 0, 0, 7, 4],
        [0, 4, 0, 0, 5, 0, 0, 3, 6],
        [7, 0, 3, 0, 1, 8, 0, 0, 0],
    ];
    solve(&mut board);
}
