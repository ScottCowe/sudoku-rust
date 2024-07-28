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

    fn solved(&self) -> Result<bool, String> {
        for (id, values) in &self.squares {
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
        for (id, values) in &self.squares {
            println!("{} {}", id, values);
        }
    }
}

fn get_peers_of_square(square: &str) -> Vec<String> {
    let mut peers = vec![];
    
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
    let block_row = (row as u8 - 65) / 3 + 1;
    let block_col = (col as u8 - '0' as u8) / 3 + 1;

    for index in 0..9 {
        let peer_row = (index / 3) * block_row;
        let peer_col = (index % 3 + 1) * block_col;

        let sqr = format!("{}{}", (peer_row + 65) as char, peer_col);

        if sqr != square {
            peers.push(sqr);
        }
    }

    let set: HashSet<_> = peers.into_iter().collect();

    set.into_iter().collect()
}

fn main() {
    let board = Board::empty();

    let peers = get_peers_of_square("A1");

    for peer in peers {
        println!("{}", peer);
    }
}
