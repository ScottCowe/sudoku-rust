use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
struct Subgrid {
    rows: [[usize; 3]; 3],
}

#[derive(Copy, Clone)]
struct Grid {
    subgrids: [[Subgrid; 3]; 3],
}

impl Subgrid {
    fn empty() -> Self {
        let emptySubgrid: Subgrid = Subgrid {
            rows: [[0; 3]; 3],
        };

        emptySubgrid 
    }

    fn populate() -> Self {
        let mut subgrid = Subgrid::empty();

        let mut rng = thread_rng();

        let mut nums = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ];
        nums.shuffle(&mut rng);

        for i in 0..9 {
            let row = i / 3;
            let col = i % 3;

            subgrid.rows[row][col] = nums[i];
        }

        subgrid
    }
}

impl Grid {
    fn empty() -> Self {
        let subgrids: [[Subgrid; 3]; 3] = [[Subgrid::empty(); 3]; 3]; 

        let grid = Grid { 
            subgrids,
        };

        grid 
    }

    fn populate() -> Self {
        let mut grid = Grid::empty();

        // create 3 randomly filled subgrids
        // place these diagonally
        grid.subgrids[0][0] = Subgrid::populate();
        grid.subgrids[1][1] = Subgrid::populate();
        grid.subgrids[2][2] = Subgrid::populate();

        // for each blank square
        for i in 0..81 {
            let row = i / 9;
            let col = i % 9;

            if row % 3 == col % 3 { // if on \ diagonal subgrid
                continue;
            }

            let mut rng = thread_rng();

            let mut nums = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9 ];
            nums.shuffle(&mut rng);

            //  pop until allowed number is found and placed
        }

        grid
    }

    fn get_nth_row(&self, n: usize) -> [usize; 9] {
        let row = [0; 9];
        row
    }

    fn get_nth_col(&self, n: usize) -> [usize; 9] {
        let col = [0; 9];
        col
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
