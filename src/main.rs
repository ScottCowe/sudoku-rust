struct Grid {
    completed: bool,
    rows: [[u8; 9]; 9],
}

fn generate_grid() -> Grid {
    let mut rows: [[u8; 9]; 9] = [[0; 9]; 9];

    // for integers n 1-9
    // create array from 0-8 and shuffle
    // for each row
    //  insert n at i-th index
    //  set 
    for i in (1..9) {
        let mut indexes: [usize; 9] = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];

        indexes = shuffle_indexes(indexes);

        for j in (0..9) {
            let mut n: usize = 0;
            
            while rows[j][indexes[j]] != 0 {
                let temp = indexes[j];
                indexes[j] = indexes[n];
                indexes[n] = temp;
                 
                n += 1; 
            }

            rows[j][indexes[j]] = i; 
        }
    }

    let grid = Grid { 
        completed: true,
        rows: rows,
    };
    
    grid 
}

fn shuffle_indexes(indexes: [usize; 9]) -> [usize; 9] {
    indexes
}

fn print_grid(grid: Grid) {
    for row in (0..9) {
        if row % 3 == 0 {
            println!("+-------+-------+-------+");
        }

        let mut line = String::from(""); 

        for col in (0..9) {
            if col % 3 == 0 {
               line.push_str("| "); 
            }

            line.push_str(&format!("{} ", grid.rows[row][col]));
        }

        println!("{}|", line);
    }

    println!("+-------+-------+-------+");
}

fn main() {
    let grid: Grid = generate_grid();
    print_grid(grid);
}
