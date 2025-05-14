use crate::minesweeper;

struct MouseButtonState {
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl MouseButtonState {
    fn new() -> Self {
        Self {
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }
}

pub struct CellElement<Message> {
    pub pos: minesweeper::Vec2,
    pub size: f32,
    pub is_enabled: bool,
    pub content: Option<String>,

    pub on_press: Option<Message>,
    pub on_left_click: Option<Message>,
    pub on_right_click: Option<Message>,
}

#[allow(dead_code)]
impl<Message> CellElement<Message> {
    pub fn new(m_content: String, m_pos: minesweeper::Vec2, m_size: f32) -> Self {
        let cell_element = Self {
            content: Some(m_content),
            pos: m_pos,
            size: m_size,
            is_enabled: true,

            on_press: None,
            on_left_click: None,
            on_right_click: None,
        };
        cell_element
    }

    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(on_press);
        self
    }
    pub fn on_left_click(mut self, on_left_click: Message) -> Self {
        self.on_left_click = Some(on_left_click);
        self
    }
    pub fn on_right_click(mut self, on_right_click: Message) -> Self {
        self.on_right_click = Some(on_right_click);
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

#[allow(dead_code)]
pub fn cell_element<Message>(
    content: &str,
    pos: minesweeper::Vec2,
    size: f32,
) -> CellElement<Message> {
    CellElement::new(content.to_string(), pos, size)
}

impl<Message, Theme, Renderer> iced::advanced::widget::Widget<Message, Theme, Renderer>
    for CellElement<Message>
where
    Renderer: iced::advanced::text::Renderer,
    Message: Clone,
{
    fn size(&self) -> iced::Size<iced::Length> {
        iced::Size {
            width: iced::Length::Shrink,
            height: iced::Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        _limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        iced::advanced::layout::Node::new(iced::Size::new(self.size, self.size))
    }

    fn draw(
        &self,
        _state: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let palette = iced::widget::Theme::extended_palette(&iced::widget::Theme::GruvboxDark);

        let bg_color = match self.is_enabled {
            true => palette.primary.strong.color,
            false => palette.primary.weak.color,
        };

        renderer.fill_quad(
            iced::advanced::renderer::Quad {
                bounds: layout.bounds(),
                border: iced::border::rounded(5),
                ..iced::advanced::renderer::Quad::default()
            },
            bg_color,
        );

        let layout_bounds = layout.bounds();

        let bounds = iced::Rectangle {
            x: layout_bounds.x,
            y: layout_bounds.y,
            width: layout_bounds.width,
            height: layout_bounds.height,
        };

        let center_point = iced::Point::new(
            bounds.x + (bounds.width / 2.0),
            bounds.y + (bounds.height / 2.0),
        );

        renderer.fill_text(
            iced::advanced::text::Text {
                content: self.content.clone().expect("err"),
                bounds: iced::Size::new(f32::INFINITY, bounds.height),
                size: iced::Pixels(self.size),
                line_height: iced::advanced::text::LineHeight::default(),
                // font: iced::Font::MONOSPACE,
                font: Renderer::default_font(&renderer),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                shaping: iced::advanced::text::Shaping::Basic,
                wrapping: iced::advanced::text::Wrapping::default(),
            },
            center_point,
            style.text_color,
            *_viewport,
        );
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(MouseButtonState::new())
    }

    fn on_event(
        &mut self,
        tree: &mut iced::advanced::widget::tree::Tree,
        event: iced::event::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> iced::event::Status {
        match event {
            // OnPress
            iced::event::Event::Mouse(iced::mouse::Event::ButtonPressed(button)) => {
                let mut status = iced::event::Status::Ignored;

                if cursor.is_over(layout.bounds()) {
                    let state = tree.state.downcast_mut::<MouseButtonState>();

                    match button {
                        iced::mouse::Button::Left => {
                            state.is_left_pressed = true;
                            status = iced::event::Status::Captured;
                        }
                        iced::mouse::Button::Right => {
                            state.is_right_pressed = true;
                            status = iced::event::Status::Captured;
                        }
                        _ => {
                            state.is_left_pressed = true;
                            state.is_right_pressed = true;
                            status = iced::event::Status::Captured;
                        }
                    };
                    // on_press - unused for this project
                    // if let Some(on_press) = &self.on_press {
                    //     shell.publish(on_press.clone());
                    //     return event::Status::Captured;
                    // }
                }
                status
            }
            // OnRelease
            iced::event::Event::Mouse(iced::mouse::Event::ButtonReleased(_)) => {
                let state = tree.state.downcast_mut::<MouseButtonState>();

                let click_message = match (state.is_left_pressed, state.is_right_pressed) {
                    (true, false) => &self.on_left_click,
                    (false, true) => &self.on_right_click,
                    _ => return iced::event::Status::Ignored,
                };

                state.is_left_pressed = false;
                state.is_right_pressed = false;

                if let Some(click_message) = click_message.clone() {
                    if cursor.is_over(layout.bounds()) {
                        shell.publish(click_message)
                    }
                }

                iced::event::Status::Captured
            }
            _ => iced::event::Status::Ignored,
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &iced::advanced::widget::tree::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());
        let is_enabled = self.is_enabled;

        if is_mouse_over && is_enabled {
            iced::mouse::Interaction::Pointer
        } else {
            iced::mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme, Renderer> From<CellElement<Message>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::text::Renderer,
    Message: Clone + 'a,
{
    fn from(cell: CellElement<Message>) -> Self {
        Self::new(cell)
    }
}

#[allow(dead_code)]
pub fn get_cell_element<'lifetime>(
    m_size: u16,
    pos: &minesweeper::Vec2,
    m_state: &minesweeper::CellState,
    m_board: &crate::minesweeper::Board,
) -> iced::Element<'lifetime, crate::BoardMessage> {
    let text_content: String = match m_state {
        minesweeper::CellState::Uncovered => match m_board.get_cell(&pos).cell_type {
            minesweeper::CellType::Safe => m_board.get_cell(&pos).adjacent_bomb_count.to_string(),
            minesweeper::CellType::Bomb => "*".to_string(),
            minesweeper::CellType::Empty(_) => " ".to_string(),
        },
        minesweeper::CellState::Hidden => "H".to_string(),
        minesweeper::CellState::Flagged => "F".to_string(),
    };

    iced::widget::container(
        iced::widget::mouse_area(
            iced::widget::text(text_content)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center),
        )
        .on_press(crate::BoardMessage::CellLeftClick(pos.x, pos.y))
        .on_right_press(crate::BoardMessage::CellRightClick(pos.x, pos.y)),
    )
    .width(m_size)
    .height(m_size)
    .style(cell_theme)
    .into()
}

fn cell_theme(theme: &iced::widget::Theme) -> iced::widget::container::Style {
    let palette = theme.extended_palette();

    iced::widget::container::Style {
        background: Some(palette.background.strong.color.into()),
        border: iced::border::rounded(0),
        ..iced::widget::container::Style::default()
    }
}
