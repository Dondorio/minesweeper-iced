mod grid;
mod new_game;
mod top_bar;

use crate::BoardMessage;
use crate::minesweeper::{Board, GameState};
use iced::Element;

pub fn grid(board: &Board) -> Element<'_, BoardMessage> {
    grid::cell_grid_element(&board)
}

pub fn top_bar(game_state: &GameState, time: usize) -> Element<'_, BoardMessage> {
    top_bar::top_bar_element(game_state, time)
}

pub fn new_game() -> Element<'static, BoardMessage> {
    new_game::new_game_element()
}
