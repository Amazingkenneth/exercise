
#[derive(Debug)]
enum BoardState {
    Config,
    Playing,
    Finish,
}

#[derive(Debug)]
pub struct Map {
    undos: usize,
    moves: usize,
    hints: usize,
    conflict: Option<[usize; 2]>,
    state: BoardState,
    stopwatch: Stopwatch,
}