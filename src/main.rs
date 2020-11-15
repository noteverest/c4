extern crate rand;

// rust has something like c++ namespaces
// TODO: module system weird
use std::fmt;
//use std::cmp;
//use std::ops;

// global board size constants
const BOARD_ROWS: u32 = 6;
const BOARD_COLS: u32 = 7;
const BOARD_SIZE: usize = ( BOARD_ROWS * BOARD_COLS ) as usize;
const PLAYERS: u32 = 2;
const WIN_CNT: u32 = 4;

#[derive(Copy,Clone,Eq,PartialEq)]
enum BoardSpace {
    Empty,
    Player(u32),
}

impl fmt::Display for BoardSpace {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            BoardSpace::Empty => write!( f, " " ),
            BoardSpace::Player(i) => write!( f, "{}", i ),
        }
    }
}

fn main() {
    println!( "Connect {}", WIN_CNT );

    let mut board: [BoardSpace; BOARD_SIZE] = [BoardSpace::Empty; BOARD_SIZE];
    let mut turn: u32 = 0;

    loop {
        // Print board state
        let player = turn % PLAYERS;
        println!( "Turn {} - Player {}", turn + 1, player );
        print_board( board );

        // Make move
        let mut played = false;
        while !played  {
            let mut input = String::new();
            std::io::stdin().read_line( &mut input )
                .expect( "failed to get" );
            let input : u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            // TODO: check for moves that are out of bounds!!!
            played = make_move( &mut board, BoardSpace::Player(player), input );
            if !played {
                println!( "Invalid move -- try again" );
            }
        }

        // Print board state again?
        println!( "Turn {} - Player {}", turn + 1, player );
        print_board( board );

        // Check for winner
        let winner = check_victory( board );
        if winner != BoardSpace::Empty {
            println!( "PLAYER {} IS THE WINNER", winner );
            break;
        }

        // Next turn
        turn += 1;
    }
}

fn print_board( board: [BoardSpace; BOARD_SIZE] ) {
    // column ledger
    for i in 0..BOARD_COLS {
        print!( " {}", i );
    }
    println!( "" );
    for i in 0..BOARD_SIZE {
        print!( "|{}", board[i] );
        if (i as u32) % BOARD_COLS == BOARD_COLS - 1 {
            println!( "|" );
        }
    }
}

fn make_move( board: &mut [BoardSpace; BOARD_SIZE], player: BoardSpace, col: u32 ) -> bool {
    let mut found : bool = false;
    let mut found_row : u32 = 0;

    // TODO: try iterating up column instead

    // iterate down column
    for row in 0..BOARD_ROWS {
        // remember empty spaces
        if board[(row * BOARD_COLS + col) as usize] == BoardSpace::Empty {
            found = true;
            found_row = row;
        }
    }

    // place in last found empty space
    if found == true {
        board[(found_row * BOARD_COLS + col) as usize] = player;
    }

    return found;
}

fn check_victory( board: [BoardSpace; BOARD_SIZE] ) -> BoardSpace {
    let mut player: BoardSpace = BoardSpace::Empty;
    let mut count : u32 = 0;

    // check rows
    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            let idx = (BOARD_COLS * row + col) as usize;
            match board[idx] {
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
            //println!( "rows - ({}, {}) - got '{}' - '{}' has {}", row, col, board[idx], player, count );
            if count == WIN_CNT {
                return player;
            }
        }
    }

    // check columns
    for col in 0..BOARD_COLS {
        for row in 0..BOARD_ROWS {
            let idx = (BOARD_COLS * row + col) as usize;
            match board[idx] {
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
            //println!( "cols - ({}, {}) - got '{}' - '{}' has {}", row, col, board[idx], player, count );
            if count == WIN_CNT {
                return player;
            }
        }
    }

    /* Strategy for checking diagonals:
        Always iterate left to right across the board
        The board is 'extended' vertically, we 'check' many off-board spaces
        We start checking with the 'highest' diagonal
            diags1 run NW to SE
                first diags1 diagonal starts 'over' the board
                intersects only the top-right board space
            diags2 run SW to NE
                first diags2 diagonal starts in the top-left board space
                the rest extends up 'over' the board
        Each following diagonal check starts vertically one space lower,
            until the whole board is checked
    */

    // check diagonals 1 - Northwest / Southeast
    for diag in 0..(BOARD_ROWS + BOARD_COLS - 1) {
        for x in 0..BOARD_COLS {
            let y : i32 = (BOARD_ROWS as i32) - (diag as i32) + (x as i32) - 1;
            if y >= 0 && y < BOARD_ROWS as i32 {
                let idx = (BOARD_COLS * y as u32 + x) as usize;
                match board[idx] {
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
                //println!( "diags1 d{} - ({}, {}) - got '{}' - '{}' has {}", diag, x, y, board[idx], player, count );
            } else {
                player = BoardSpace::Empty;
                count = 0;
            }
            if count == WIN_CNT {
                return player;
            }
        }
    }

    // check diagonals 2 - Northeast / Southwest
    for diag in 0..(BOARD_ROWS + BOARD_COLS - 1) {
        for x in 0..BOARD_COLS {
            let y : i32 = (diag as i32) - (x as i32);
            if y >= 0 && y < BOARD_ROWS as i32 {
                let idx = (BOARD_COLS * y as u32 + x) as usize;
                match board[idx] {
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
                //println!( "diags2 d{} - ({}, {}) - got '{}' - '{}' has '{}'", diag, x, y, board[idx], player, count );
            } else {
                player = BoardSpace::Empty;
                count = 0;
            }
            if count == WIN_CNT {
                return player;
            }
        }
    }

    return BoardSpace::Empty;
}

