use std::vec::Vec;
use std::fmt;

pub struct Board {
    squares: Vec<String>,
    solved: bool
}

impl Board {
    pub fn empty() -> Self {
        let mut squares = Vec::new();

        for _ in 0..81 {
            squares.push("123456789".to_string());
        }

        let solved = false;
        let board = Board { squares, solved } ;

        board
    }

    fn solve(&mut self) {
        let mut avaliable_squares: Vec<_> = Vec::new();
       
        for index in 0..81 {
            if &self.squares[index].len() > &1 {
                avaliable_squares.push(index);
            } 
        }
       
        while !&self.solved {
            // pick random avaliable square
            // pick random value for this square 
            // set value of square to this value, keeping track of index and old value 
            // if a square has no possible value, revert and try again
            // set solved to true if every square has only one value
        }
    }

    fn peers_of(&self, square: usize) -> Vec<usize> {
        let mut peers = Vec::new();

        let row = square / 9;
        let col = square % 9;

        // note: index = 9 * row + col
        // squares in row are gonna be 
        //  ..., row-1, row+1, ...
        (0..9).into_iter().map(|i| i*9 + col).for_each(|i| if i != square { peers.push(i) });

        // squares in col are gonna be 
        //  ..., col-1, col+1, ...
        (0..9).into_iter().map(|i| row*9 + i).for_each(|i| if i != square { peers.push(i) });

        // squares in block are gonna be 
        //  center row/col at i/3 * 3 + 1
        //  (row-1, col-1), (row-1, col+1)
        //  (row+1, col-1), (row+1, col+1)

        peers
    }

    fn set_square(&mut self, square: usize, to: &String) {
        let peers = Self::peers_of(self, square);

        let modify_peers = to.len() != 1;

        self.squares[square] = to.to_string();

        if modify_peers {
            for peer in peers {
                let new_value = &self.squares[peer].replace(to, "");
                Self::set_square(self, peer, &new_value);
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new(); 

        for index in 0..81 {
            let square = &self.squares[index];
            let num_spaces = 10 - self.squares[index].len();
            // let square = format!("{}", index);
            // let num_spaces = 3 - square.len();
            let square_str = format!("{}{}", square, " ".repeat(num_spaces));

            out.push_str(&square_str);

            if index % 9 == 8 {
                out.push_str("\n");
            }
        }

        write!(f, "{}", out)
    }
}
