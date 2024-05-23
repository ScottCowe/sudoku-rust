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
            let col = i - row * 3;

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

        let allowedCols = [[ 0, 1, 2, 3, 4, 5, 6, 7, 8 ]; 9];

        for i in 1..=1 { // for testing
            let mut allowedSquares = allowedCols[i - 1];
            
            for row in 0..9 {
                let mut index = rand::thread_rng().gen_range(0..=8);
                let mut col = allowedSquares[index];

                while col == 10 {
                    index = rand::thread_rng().gen_range(0..=8);
                    col = allowedSquares[index];
                }

                allowedSquares[index] = 10;

                let gridRow = row / 3;
                let gridCol = col / 3;

                let subgridRow = row % 3;
                let subgridCol = col % 3;

                println!("{} {} {} {} {} {}", row, col, gridRow, gridCol, subgridRow, subgridCol);

                grid.subgrids[gridRow][gridCol].rows[subgridRow][subgridCol] = i;
                print_grid(grid);
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
