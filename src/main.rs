extern crate rand;

// rust has something like c++ namespaces
// TODO: module system weird
use std::fmt;
use std::cmp;
//use std::ops;

// global board size constants
const BOARD_ROWS: usize = 6;
const BOARD_COLS: usize = 7;
const WIN_CNT: usize = 4;

#[derive(Copy,Clone,Eq,PartialEq)]
enum BoardSpace {
    Empty,
    Player(u8),
}

impl fmt::Display for BoardSpace {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            BoardSpace::Empty => write!( f, " " ),
            BoardSpace::Player(1) => write!( f, "O" ),
            BoardSpace::Player(2) => write!( f, "X" ),
            BoardSpace::Player(i) => write!( f, "{}", i ),
        }
    }
}

fn main() {
    println!( "Connect {}", WIN_CNT );

    //let mut row: [BoardSpace; BOARD_COLS] = [ BoardSpace::Empty; BOARD_COLS ];
    //let mut board: [[BoardSpace; BOARD_COLS]; BOARD_ROWS] = [ row; BOARD_ROWS ];
    //print_board( board );

    let mut board: [BoardSpace; BOARD_ROWS * BOARD_COLS ] = [BoardSpace::Empty; BOARD_ROWS * BOARD_COLS];
    let mut winner: BoardSpace = BoardSpace::Empty;
    let mut turn: u32 = 1;

    loop {
        println!( "Turn {}", turn );
        print_board( board );

        let mut input = String::new();

        std::io::stdin().read_line( &mut input )
            .expect( "failed to get" );

        let input : usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if turn % 2 == 0 {
            if !place_thing( &mut board, BoardSpace::Player(1), input ) {
                println!( "didn't work, guess you lost your turn, dummy" );
            }
        } else {
            if !place_thing( &mut board, BoardSpace::Player(2), input ) {
                println!( "didn't work, guess you lost your turn, dummy" );
            }
        }
        println!( "Turn {}", turn );
        print_board( board );

        winner = check_victory( board );
        if winner != BoardSpace::Empty {
            println!( "A WINNER IS {}", winner );
            break;
        }

        turn += 1;
    }
}

fn print_board( board: [BoardSpace; BOARD_ROWS * BOARD_COLS ] ) {
    // ledger
    for i in 0..BOARD_COLS {
        print!( " {}", i );
    }
    println!( "" );
    for i in 0..(BOARD_ROWS * BOARD_COLS) {
        print!( "|{}", board[i] );
        if i % BOARD_COLS == BOARD_COLS - 1 {
            println!( "|" );
        }
    }
}

fn place_thing( board: &mut [BoardSpace; BOARD_ROWS * BOARD_COLS ], player: BoardSpace, col: usize ) -> bool {
    let mut found : bool = false;
    let mut found_row : usize = 0;

    // iterate down column
    for row in 0..BOARD_ROWS {
        // remember empty spaces
        if board[row * BOARD_COLS + col] == BoardSpace::Empty {
            found = true;
            found_row = row;
        }
    }

    // place in last found empty space
    if found == true {
        board[found_row * BOARD_COLS + col] = player;
    }

    return found;
}

fn check_victory( board: [BoardSpace; BOARD_ROWS * BOARD_COLS ] ) -> BoardSpace {
    let mut player: BoardSpace = BoardSpace::Empty;
    let mut count : usize = 0;

    // check rows
    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            match board[BOARD_COLS * row + col] {
                BoardSpace::Empty => {
                    player = BoardSpace::Empty;
                    count = 0;
                },
                BoardSpace::Player(p) => {
                    if player == BoardSpace::Player(p) {
                        count += 1;
                    } else {
                        player = BoardSpace::Player(p);
                        count = 1;
                    }
                },
            }
            println!( "row - check ({}, {}) - cnt {} {}", row, col, count, player );
            if count == WIN_CNT {
                return player;
            }
        }
    }

    // check columns
    for col in 0..BOARD_COLS {
        for row in 0..BOARD_ROWS {
            match board[BOARD_COLS * row + col] {
                BoardSpace::Empty => {
                    player = BoardSpace::Empty;
                    count = 0;
                },
                BoardSpace::Player(p) => {
                    if player == BoardSpace::Player(p) {
                        count += 1;
                    } else {
                        player = BoardSpace::Player(p);
                        count = 1;
                    }
                },
            }
            println!( "col - check ({}, {}) - cnt {} {}", row, col, count, player );
            if count == WIN_CNT {
                return player;
            }
        }
    }

    // build diagonal lengths
    let len_diag: usize = cmp::min( BOARD_ROWS, BOARD_COLS );
    let mut len_diag_rows: [usize; BOARD_ROWS - 1] = [0; BOARD_ROWS - 1];
    let mut len_diag_cols: [usize; BOARD_COLS - 1] = [0; BOARD_COLS - 1];

    // fill in row diagonal lengths
    for r in (1..BOARD_ROWS).rev() {
        if BOARD_ROWS > BOARD_COLS {
            let diff = BOARD_COLS - BOARD_ROWS;
            if r < diff {
                len_diag_rows[r] = BOARD_COLS;
            } else {
                len_diag_rows[r] = BOARD_COLS - (r - diff + 1);
            }
        } else {
            // BOARD_COLS > BOARD_ROWS {
            len_diag_rows[r] = BOARD_COLS - (r + 1);
        }
    }

    // fill in column diagonal lengths
    for c in 0..BOARD_COLS - 1 {
        if BOARD_COLS > BOARD_ROWS {
            let diff = BOARD_COLS - BOARD_ROWS;
            if c < diff {
                len_diag_cols[c] = BOARD_ROWS;
            } else {
                len_diag_cols[c] = BOARD_ROWS - (c - diff + 1);
            }
        } else {
            // BOARD_ROWS >= BOARD_COLS {
            len_diag_cols[c] = BOARD_ROWS - (c + 1);
        }
    }

    // check diagonals, top-left <-> bottom-right

    // check corner diagonal
    for space in 0..cmp::min( BOARD_ROWS, BOARD_COLS ) {
        match board[BOARD_COLS * (space) + space] {
                BoardSpace::Empty => {
                    player = BoardSpace::Empty;
                    count = 0;
                },
                BoardSpace::Player(p) => {
                    if player == BoardSpace::Player(p) {
                        count += 1;
                    } else {
                        player = BoardSpace::Player(p);
                        count = 1;
                    }
                },
            }
            println!( "d,rd  - check ({}, {}) - cnt {} {}", space, space, count, player );
            if count == WIN_CNT {
                return player;
            }
    }

    // check from left side, top -> bottom
    for diag in 0..BOARD_ROWS - (WIN_CNT - 1) - 1 {
        for space in 0..len_diag_rows[diag] {
            match board[BOARD_COLS * (diag + space + 1) + space] {
                BoardSpace::Empty => {
                    player = BoardSpace::Empty;
                    count = 0;
                },
                BoardSpace::Player(p) => {
                    if player == BoardSpace::Player(p) {
                        count += 1;
                    } else {
                        player = BoardSpace::Player(p);
                        count = 1;
                    }
                },
            }
            println!( "d,rd  - check ({}, {}) - cnt {} {}", diag+space+1, space, count, player );
            if count == WIN_CNT {
                return player;
            }
        }
    }
    // check from top row, left -> right
    for diag in 0..BOARD_COLS - (WIN_CNT - 1) {
        for space in 0..len_diag_cols[diag] {
            match board[BOARD_COLS * space + (diag + space + 1)] {
                BoardSpace::Empty => {
                    player = BoardSpace::Empty;
                    count = 0;
                },
                BoardSpace::Player(p) => {
                    if player == BoardSpace::Player(p) {
                        count += 1;
                    } else {
                        player = BoardSpace::Player(p);
                        count = 1;
                    }
                },
            }
            println!( "d,rd  - check ({}, {}) - cnt {} {}", space, diag+space+1, count, player );
            if count == WIN_CNT {
                return player;
            }
        }
    }

    // check diagonals, bottom-left <-> top-right

    // check corner diagonal
    for space in 0..cmp::min( BOARD_ROWS, BOARD_COLS ) {
        match board[BOARD_COLS * (BOARD_ROWS - 1 - space) + space] {
                BoardSpace::Empty => {
                    player = BoardSpace::Empty;
                    count = 0;
                },
                BoardSpace::Player(p) => {
                    if player == BoardSpace::Player(p) {
                        count += 1;
                    } else {
                        player = BoardSpace::Player(p);
                        count = 1;
                    }
                },
            }
            println!( "d,ru  - check ({}, {}) - cnt {} {}", BOARD_ROWS - space - 1, space, count, player );
            if count == WIN_CNT {
                return player;
            }
    }

    // check from left side, bottom -> top
    for diag in 0..BOARD_ROWS - (WIN_CNT - 1) {
        for space in 0..cmp::min( BOARD_ROWS, BOARD_COLS ) - diag {
            match board[BOARD_COLS * (BOARD_ROWS - 1 - diag - space) + space] {
                BoardSpace::Empty => {
                    player = BoardSpace::Empty;
                    count = 0;
                },
                BoardSpace::Player(p) => {
                    if player == BoardSpace::Player(p) {
                        count += 1;
                    } else {
                        player = BoardSpace::Player(p);
                        count = 1;
                    }
                },
            }
            println!( "d,ru - check ({}, {}) - cnt {} {}", BOARD_ROWS-diag-space, space, count, player );
            if count == WIN_CNT {
                return player;
            }
        }
    }

    // check from bottom row, left -> right
    for diag in 0..BOARD_COLS - (WIN_CNT - 1) {
        for space in 0..cmp::min( BOARD_ROWS, BOARD_COLS ) - diag {
            match board[BOARD_COLS * (BOARD_ROWS - 1 - space) + diag + space] {
                BoardSpace::Empty => {
                    player = BoardSpace::Empty;
                    count = 0;
                },
                BoardSpace::Player(p) => {
                    if player == BoardSpace::Player(p) {
                        count += 1;
                    } else {
                        player = BoardSpace::Player(p);
                        count = 1;
                    }
                },
            }
            println!( "d,ru  - check ({}, {}) - cnt {} {}", BOARD_ROWS-space, diag+space, count, player );
            if count == WIN_CNT {
                return player;
            }
        }
    }

    return BoardSpace::Empty;
}

/*
fn print_board( board: [[BoardSpace; BOARD_COLS]; BOARD_ROWS] ) {
    for row in board.iter() {
        print!( "|" );
        for space in row {
            print!( " {} |", space );
        }
        println!( "" );
    }
}
*/

