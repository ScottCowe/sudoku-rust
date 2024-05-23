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
        let indexes: [usize; 9] = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];

        // shuffle indexes

        for j in (0..8) {
            rows[j][indexes[j]] = i; 
        }
    }

    let grid = Grid { 
        completed: true,
        rows: rows,
    };
    
    grid 
}

fn main() {
    println!("Hello, world!");
}
