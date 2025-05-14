use crate::{messages::BoardMessage, resources};
use iced::Length;
use iced::widget::image::FilterMethod;
use iced::widget::{column, container, image, mouse_area, row};

pub fn new_game_element() -> iced::Element<'static, BoardMessage> {
    column![
        row![
            container(
                mouse_area(
                    image(resources::get_image_handle("difficulty/beginner.png"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .filter_method(FilterMethod::Nearest)
                )
                .on_release(BoardMessage::SubmitNewGame(8, 8, 10))
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill),
            container(
                mouse_area(
                    image(resources::get_image_handle("difficulty/intermediate.png"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .filter_method(FilterMethod::Nearest)
                )
                .on_release(BoardMessage::SubmitNewGame(16, 16, 40))
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill),
        ],
        row![
            container(
                mouse_area(
                    image(resources::get_image_handle("difficulty/expert.png"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .filter_method(FilterMethod::Nearest)
                )
                .on_release(BoardMessage::SubmitNewGame(30, 16, 99))
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill),
            container(
                mouse_area(
                    image(resources::get_image_handle("difficulty/null.png"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .filter_method(FilterMethod::Nearest)
                ) // .on_release(BoardMessage::SubmitNewGame(30, 16, 99))
            )
        ],
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
