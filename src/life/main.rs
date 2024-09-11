
macro_rules! board_fn {
    ($name:ident, $body:block) => {
        fn $name<T, const W: usize, const H: usize>(board: &Board<T, W, H>) $body
    };
}

// trait LifeRules {
//     fn adjacent(&self, board: &Board<T, const W:usize, const H: usize>) -> Vec<Cell>;
// }

struct Board<T, const W: usize, const H: usize> {
    cells: [[T; W]; H],
}


impl<T, const W: usize, const H: usize> Board<T, W, H> {
    // Constructor to initialize the board with a default value
    pub fn new(default: T) -> Self {
        todo!()
    }
}

enum State {
    Alive,
    Dead,
}

struct Cell {
    state: State
}




fn run() {
    let board = Board::new(Cell::new());
}