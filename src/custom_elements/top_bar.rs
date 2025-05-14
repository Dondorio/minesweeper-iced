use crate::globals;
use crate::messages::BoardMessage;
use crate::minesweeper::GameState;
use crate::resources;

use iced::widget::image::FilterMethod;
use iced::widget::{Row, Theme, container, image, mouse_area, row};
use iced::{Border, Element, Length, border, color};

fn red_text(number: usize) -> Element<'static, BoardMessage> {
    let n = number + 1000;

    let row = n.to_string()[1..4]
        .chars()
        .map(|i| {
            image(resources::get_image_handle(format!("red_text/{}.png", i)))
                .filter_method(FilterMethod::Nearest)
                .width(globals::SCALE)
                .height(globals::SCALE as f32 * 1.5)
                .into()
        })
        .collect::<Vec<_>>();

    container(Row::from_vec(row).spacing(globals::PIXEL_SIZE * 2.0))
        .padding(globals::PIXEL_SIZE * 2.0)
        .height(Length::Shrink)
        .width(Length::Shrink)
        .style(red_text_style)
        .into()
}

fn red_text_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(color!(0x0b0b1c).into()),
        border: Border::default()
            .color(color!(0x3d3d3d))
            .width(globals::PIXEL_SIZE * 0.5),
        ..container::Style::default()
    }
}

fn top_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(color!(0x828282).into()),
        border: border::rounded(0),
        ..container::Style::default()
    }
}

pub fn top_bar_element(game_state: &GameState, time: usize) -> Element<'_, BoardMessage> {
    let image_name = format!(
        "{}.png",
        match game_state {
            GameState::Playing => "playing",
            GameState::Lost => "lost",
            GameState::Won => "won",
        }
    );

    row![
        // Centre
        container(
            // Face
            container(
                mouse_area(
                    image(resources::get_image_handle(image_name))
                        .filter_method(FilterMethod::Nearest)
                        .height(globals::SCALE)
                        .width(globals::SCALE)
                )
                .on_release(BoardMessage::OpenNewGameModal)
            )
            .height(globals::SCALE)
            .width(globals::SCALE)
        )
        .style(top_bar_style)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill),
        // Timer
        container(red_text(time))
            .center_y(Length::Fill)
            .center_x(Length::Fill)
            .style(top_bar_style),
    ]
    .width(Length::Fill)
    .height(globals::SCALE * 2)
    .into()
}
