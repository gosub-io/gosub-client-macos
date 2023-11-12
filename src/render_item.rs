use cacao::color::Color;
use cacao::layout::Layout;
use cacao::layout::LayoutConstraint;
use cacao::text::Font;
use cacao::text::Label;
use cacao::view::View;

use crate::render_cursor::RenderCursor;

#[derive(Debug, PartialEq)]
pub enum RenderItemType {
    Empty,
    Heading1 { body: String, font_size: f64 },
    Paragraph { body: String, font_size: f64 },
}

#[derive(Debug)]
pub struct RenderItem {
    pub item_type: RenderItemType,
    pub x: f64,
    pub y: f64,
}

impl RenderItem {
    pub fn new() -> Self {
        Self {
            item_type: RenderItemType::Empty,
            x: 0.,
            y: 0.,
        }
    }

    pub fn new_heading1() -> Self {
        Self {
            item_type: RenderItemType::Heading1 {
                body: "".to_owned(),
                font_size: 32.,
            },
            x: 0.,
            y: 0.,
        }
    }

    pub fn new_paragraph() -> Self {
        Self {
            item_type: RenderItemType::Paragraph {
                body: "".to_owned(),
                font_size: 16.,
            },
            x: 0.,
            y: 0.,
        }
    }

    pub fn append_body(&mut self, content: &str) {
        match self.item_type {
            RenderItemType::Heading1 { ref mut body, .. }
            | RenderItemType::Paragraph { ref mut body, .. } => body.push_str(content),
            _ => {}
        }
    }

    pub fn place(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Draws a label object onto the screen and returns a (x, y) tuple
    /// of how many pixels to offset the cursor for drawing the next item.
    pub fn draw_label(&self, context: &View, cursor: &mut RenderCursor) {
        let label = Label::new();
        match &self.item_type {
            RenderItemType::Heading1 { body, font_size }
            | RenderItemType::Paragraph { body, font_size } => {
                label.set_font(Font::with_name("Times New Roman", *font_size));
                label.set_text(body);
                label.set_text_color(Color::SystemBlack);
                context.add_subview(&label);
                LayoutConstraint::activate(&[
                    label.left.constraint_equal_to(&context.left).offset(self.x),
                    label.top.constraint_equal_to(&context.top).offset(self.y),
                ]);

                // the +5 is for vertical padding
                cursor.adjust(0., *font_size + 5.);
            }
            _ => {}
        }
    }
}
