use std::collections::{ HashMap, HashSet };

struct Board {
    squares: HashMap<String, String>
}

impl Board {
    fn empty() -> Self {
        let mut squares = HashMap::new();

        for index in 0..81 {
            let row: u8 = index / 9;
            let col: u8 = index % 9;

            let square = format!("{}{}", (row + 65) as char, col + 1);
            
            squares.insert(square, "123456789".to_string());
        }

        let board = Board { squares };
        board
    }

    fn generate_puzzle() -> Self {
        let mut board = Self::empty();

        board
    }

    fn set_square(&self, square: &str, to: u8) {
        let peers = get_peers_of_square(square);
    
        // set entry in hashmap to to 

        // basic strat
        // if square only has one possibility
        //  iterate thru peers
        //      set square to current minus to (recursive) 
    }

    fn is_solved(&self) -> Result<bool, String> {
        for (_, values) in &self.squares {
            if values == "" {
                return Err("Board is invalid".to_string());
            }

            if values.len() != 1 {
                return Ok(false);
            }
        }

        Ok(true) 
    }

    fn print(&self) {
        let mut board = String::new();

        for index in 0..81 {
            let row = index / 9;
            let col = index % 9;

            let square = format!("{}{}", (row as u8 + 65) as char, col + 1);

            let value = &self.squares.get(&square).unwrap();

            board += value; 
            board += "\t";

            if index % 9 == 8 {
                board += "\n";
            }
        }

        println!("{}", board);
    }
}

fn get_peers_of_square(square: &str) -> Vec<String> {
    let mut peers = Vec::new();
    
    let row = square.chars().nth(0).unwrap();
    let col = square.chars().nth(1).unwrap();

    for index in 1..=9 {
        let sqr = format!("{}{}", row, index);
        
        if sqr != square {
            peers.push(sqr);
        }
    }
    
    for index in 0..9 {
        let sqr = format!("{}{}", (index as u8 + 65) as char, col);

        if sqr != square {
            peers.push(sqr);
        }
    }

    // Get peers in block
    let block_row = (row as u8 - 65) / 3;
    let block_col = (col as u8 - '0' as u8) / 3;

    for index in 0..9 {
        let inblock_row = index / 3;
        let inblock_col = index % 3;

        let peer_row = block_row * 3 + inblock_row + 1;
        let peer_col = block_col * 3 + inblock_col + 1;

        let sqr = format!("{}{}", (peer_row + 64) as char, peer_col);

        if sqr != square {
            peers.push(sqr);
        }
    }

    // Removes duplicated and turns back into vec
    let set: HashSet<String> = peers.into_iter().collect();
    let mut result: Vec<String> = set.into_iter().collect();
    result.sort();

    result
}

fn main() {
    let board = Board::empty();
    board.print();
}
