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

        for i in 0..9 {
            for j in 0..9 {
                let row = j / 3;
                let col = j - row * 3;

                grid.subgrids[i / 3][i - (i / 3) * 3].rows[row][col] = j + 1;
            }
        }

        grid
    }

    fn get_rows(&self) -> [[usize; 9]; 9] {
        let mut rows: [[usize; 9]; 9] = [[0; 9]; 9];

        for i in 0..81 {
            let row = i / 9;
            let col = i - row * 9;

            let gridRow = i / 27;
            let gridCol = (i - gridRow * 27) / 9;

            let subgrid = self.subgrids[gridRow][gridCol];

            let subgridRow = row / 3;
            let subgridCol = row - subgridRow * 3;

            rows[row][col] = subgrid.rows[subgridRow][subgridCol];
        }

        rows
    }
}

fn shuffle_indexes(indexes: [usize; 9]) -> [usize; 9] {
    let mut shuffled = indexes;

    for i in (0..7) {
        let j = rand::thread_rng().gen_range(i..9);
        let temp = shuffled[i];
        shuffled[i] = shuffled[j];
        shuffled[j] = temp;
    }

    shuffled 
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

    let rows = grid.get_rows();
}
