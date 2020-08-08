extern crate rand;

// rust has something like c++ namespaces
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
    */
}

fn print_board( board: [[BoardSpace; BOARD_COLS]; BOARD_ROWS] ) {
    for row in board.iter() {
        print!( "|" );
        for space in row {
            print!( " {} |", space );
        }
        println!( "" );
    }
}

