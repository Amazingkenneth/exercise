
#[derive(Debug)]
enum BoardState {
    Config,
    Playing,
    Finish,
}

#[derive(Debug)]
pub struct Map {
    state: BoardState,
}