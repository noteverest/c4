extern crate rand;

// rust has something like c++ namespaces
use std::fmt;

// global board size constants
const BOARD_ROWS: usize = 6;
const BOARD_COLS: usize = 7;

#[derive(Copy)]
enum BoardSpace {
    Empty,
    Player1,
    Player2,
}

impl fmt::Display for BoardSpace {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            BoardSpace::Empty => write!( f, " " ),
            BoardSpace::Player1 => write!( f, "O" ),
            BoardSpace::Player2 => write!( f, "X" ),
        }
    }
}

fn main() {
    println!( "Guess the number!" );

    let mut row: [BoardSpace; BOARD_COLS] = [ BoardSpace::Empty; BOARD_COLS ];
    let mut board: [[BoardSpace; BOARD_COLS]; BOARD_ROWS] = [ row; BOARD_ROWS ];

    print_board( board );

    /*
    loop {
        println!( "Please input your guess." );

        let mut guess = String::new();

        std::io::stdin().read_line( &mut guess )
            .expect( "Failed to read line" );

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!( "You guessed: {}", guess );

        match guess.cmp( &secret_number ) {
            std::cmp::Ordering::Less => println!( "Too small!" ),
            std::cmp::Ordering::Greater => println!( "Too big!" ),
            std::cmp::Ordering::Equal => {
                println!( "You win!" );
                break;
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

