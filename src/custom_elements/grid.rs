use crate::globals;
use crate::messages::BoardMessage;
use crate::minesweeper::{Board, CellPressedState, CellState, CellType, Vec2};
use crate::resources;
use iced::Element;
use iced::widget::image::FilterMethod;
use iced::widget::{Column, Row, container, image, mouse_area};

pub fn cell_grid_element(board: &Board) -> Element<'_, BoardMessage> {
    let mut grid = Row::new();

    for row in 0..board.size_x {
        let mut column_element: Column<BoardMessage> = Column::new();

        for column in 0..board.size_y {
            let pos = Vec2 { x: row, y: column };
            let cell = board.get_cell(&pos);

            let image_name = match cell.state {
                CellState::Hidden => match cell.pressed_state {
                    CellPressedState::None => "hidden",
                    CellPressedState::Hovered => "hidden-hovered",
                    CellPressedState::Pressed => "hidden-pressed",
                },
                CellState::Flagged => "flag",
                CellState::Uncovered => match cell.cell_type {
                    CellType::Bomb => match cell.is_exploded {
                        false => "bomb",
                        true => "bomb-exploded",
                    },
                    CellType::Safe => &cell.adjacent_bomb_count.to_string(),
                    CellType::Empty(..) => "empty",
                },
            };

            let cell_element: Element<'_, BoardMessage> = container(
                mouse_area(
                    image(resources::get_image_handle(format!("{}.png", image_name)))
                        .filter_method(FilterMethod::Nearest)
                        .height(globals::SCALE)
                        .width(globals::SCALE),
                )
                .on_enter(BoardMessage::CellHover(row, column))
                .on_exit(BoardMessage::CellUnhover(row, column))
                .on_press(BoardMessage::CellPress(row, column))
                .on_release(BoardMessage::CellLeftClick(row, column))
                .on_right_release(BoardMessage::CellRightClick(row, column)),
            )
            .height(globals::SCALE)
            .width(globals::SCALE)
            .into();

            column_element = column_element.push(cell_element);
        }
        grid = grid.push(column_element);
    }
    grid.into()
}
