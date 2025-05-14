mod custom_elements;
mod globals;
mod messages;
mod minesweeper;
mod resources;

use iced::time::{self, Duration};
use iced::{
    Element, Length, Size, Subscription, Task, Theme,
    widget::{column, container},
    window,
};
use messages::BoardMessage;
use minesweeper::{Board, CellPressedState, Vec2};

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .window_size(Size::new(
            (globals::SCALE * 8) as f32,
            (globals::SCALE * 10) as f32,
        ))
        .resizable(false)
        .centered()
        .run_with(|| (App::new(), Task::none()))
}

#[derive(Debug, Clone)]
struct App {
    pub board: Board,
    pub show_modal: bool,
    pub timer: usize,
    // pub time: u32,
    // pub difficulty: String,
}

impl App {
    fn new() -> Self {
        Self {
            board: Board::new(8, 8, 10).unwrap(),
            show_modal: false,
            timer: 0,
            // difficulty: "Beginner".to_string(),
        }
    }

    fn title(&self) -> String {
        // String::from(format!("Mineweeper - {}", self.difficulty))
        String::from("Mineweeper")
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn subscription(&self) -> Subscription<BoardMessage> {
        return match self.board.state {
            minesweeper::GameState::Playing => {
                time::every(Duration::from_secs(1)).map(|_| BoardMessage::Tick)
            }
            _ => Subscription::none(),
        };
    }

    fn update(&mut self, message: BoardMessage) -> Task<BoardMessage> {
        match message {
            // Cell
            BoardMessage::CellHover(pos_x, pos_y) => {
                self.board
                    .get_mut_cell(&Vec2 { x: pos_x, y: pos_y })
                    .pressed_state = CellPressedState::Hovered;
            }
            BoardMessage::CellUnhover(pos_x, pos_y) => {
                self.board
                    .get_mut_cell(&Vec2 { x: pos_x, y: pos_y })
                    .pressed_state = CellPressedState::None;
            }
            BoardMessage::CellPress(pos_x, pos_y) => {
                self.board
                    .get_mut_cell(&Vec2 { x: pos_x, y: pos_y })
                    .pressed_state = CellPressedState::Pressed;
            }
            BoardMessage::CellLeftClick(pos_x, pos_y) => {
                let pos = &Vec2 { x: pos_x, y: pos_y };
                let cell = self.board.get_cell(pos);

                if let CellPressedState::Pressed = cell.pressed_state {
                    self.board.uncover(pos);
                }

                self.board
                    .get_mut_cell(&Vec2 { x: pos_x, y: pos_y })
                    .pressed_state = CellPressedState::None;
            }
            BoardMessage::CellRightClick(pos_x, pos_y) => {
                self.board
                    .get_mut_cell(&Vec2 { x: pos_x, y: pos_y })
                    .toggle_flagged();
            }

            BoardMessage::Tick => {
                self.timer += 1;
            }

            // New game
            BoardMessage::OpenNewGameModal => {
                self.show_modal = true;

                return window::get_latest().and_then(move |id| {
                    window::resize(
                        id,
                        Size::new((globals::SCALE * 8) as f32, (globals::SCALE * 8) as f32),
                    )
                });
            }
            BoardMessage::SubmitNewGame(size_x, size_y, bomb_count) => {
                self.board = Board::new(size_x, size_y, bomb_count).unwrap();
                self.show_modal = false;
                self.timer = 0;

                let size = Size::new(
                    (globals::SCALE * self.board.size_x as u16) as f32,
                    (globals::SCALE * (2 + self.board.size_y as u16)) as f32,
                );

                return window::get_latest().and_then(move |id| window::resize(id, size));
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, BoardMessage> {
        let mut content = column![
            custom_elements::top_bar(&self.board.state, self.timer),
            container(custom_elements::grid(&self.board))
                .width(Length::Fill)
                .center_x(Length::Fill)
        ];

        if self.show_modal {
            content = column![custom_elements::new_game()]
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
