#[derive(Debug, Clone, Copy)]
pub enum BoardMessage {
    CellLeftClick(usize, usize),
    CellRightClick(usize, usize),

    CellHover(usize, usize),
    CellUnhover(usize, usize),

    CellPress(usize, usize),

    OpenNewGameModal,
    SubmitNewGame(usize, usize, usize),

    Tick,
}
