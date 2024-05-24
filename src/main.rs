use rand::Rng;

#[derive(Copy, Clone)]
struct Subgrid {
    rows: [[usize; 3]; 3],
}

#[derive(Copy, Clone)]
struct Grid {
    subgrids: [[Subgrid; 3]; 3],
}

impl Subgrid {
    fn contains(&self, number: usize) -> bool {
        for i in 0..9 {
            let row = i / 3;
            let col = i % 3;

            if self.rows[row][col] == number {
                return true;
            }
        }

        return false;
    }
}

impl Grid {
    fn empty() -> Self {
        let emptySubgrid: Subgrid = Subgrid {
            rows: [[0; 3]; 3],
        };

        let subgrids: [[Subgrid; 3]; 3] = [[emptySubgrid; 3]; 3]; 

        let grid = Grid { 
            subgrids,
        };

        grid 
    }

    fn populate() -> Self {
        let mut grid = Self::empty();

        for n in 1..=9 { 
            let mut allowedRows: Vec<usize> = vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8 ]; 
            let mut allowedCols: Vec<usize> = vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];

            let mut i = 0;

            while i < 9 { 
                let gridRow = i / 3;
                let gridCol = i % 3;

                let mut subgridRow = 3;
                let mut subgridCol = 3;

                let mut row = 9;
                let mut col = 9;

                let mut subgridRowOptions = vec![ 0, 1, 2 ];
                let mut subgridColOptions = vec![ 0, 1, 2 ];

                while !allowedRows.contains(&row) && subgridRowOptions.len() > 0 {
                    let index = rand::thread_rng().gen_range(0..subgridRowOptions.len());
                    subgridRow = subgridRowOptions.remove(index);
                    row = gridRow * 3 + subgridRow;
                    println!("randoming");
                }

                let rowIndex = allowedRows.iter().position(|&r| r == row).unwrap();
                allowedRows.remove(rowIndex); 

                while !allowedCols.contains(&col) && subgridColOptions.len() > 0 {
                    let index = rand::thread_rng().gen_range(0..subgridColOptions.len());
                    subgridCol = subgridColOptions.remove(index);
                    col = gridCol * 3 + subgridCol;
                    println!("randoming");
                }

                let colIndex = allowedCols.iter().position(|&r| r == col).unwrap();
                allowedCols.remove(colIndex);

                println!("Trying {} {}", row, col);

                if grid.subgrids[gridRow][gridCol].rows[subgridRow][subgridCol] == 0 {
                    grid.subgrids[gridRow][gridCol].rows[subgridRow][subgridCol] = n;
                    print_grid(grid);
                    i += 1;
                }
            }
        }

        grid
    }

    fn get_rows(&self) -> [[usize; 9]; 9] {
        let mut rows: [[usize; 9]; 9] = [[0; 9]; 9];

        for i in 0..81 {
            let row = i / 9;
            let col = i % 9;

            let gridRow = row / 3;
            let gridCol = col / 3;

            let subgridRow = row % 3;
            let subgridCol = col % 3;

            rows[row][col] = self.subgrids[gridRow][gridCol].rows[subgridRow][subgridCol];
        }

        rows
    }
}

fn print_grid(grid: Grid) {
    for row in (0..9) {
        if row % 3 == 0 {
            println!("+-------+-------+-------+");
        }

        let mut line = String::from(""); 

        for col in 0..9 {
            if col % 3 == 0 {
               line.push_str("| "); 
            }

            let num = grid.get_rows()[row][col];

            if num == 0 {
                line.push_str("  ");
            }
            else {
                line.push_str(&format!("{} ", num));
            }
        }

        println!("{}|", line);
    }

    println!("+-------+-------+-------+");
}

fn main() {
    let grid: Grid = Grid::populate();
    print_grid(grid);
}
