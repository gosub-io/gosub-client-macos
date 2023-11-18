use cacao::color::Color;
use cacao::layout::Layout;
use cacao::layout::LayoutConstraint;
use cacao::text::Font;
use cacao::view::View;

use crate::render_cursor::RenderCursor;

pub mod text;

use crate::render_item::text::Text;

/// A trait to be implemented by different render items to remove them from the render window
pub trait Remove {
    fn remove(&mut self);
}

#[derive(Debug)]
pub enum RenderItemType {
    Empty,
    Heading1(Text),
    Paragraph(Text),
}

#[derive(Debug)]
pub struct RenderItem {
    pub item_type: RenderItemType,
    pub x: f64,
    pub y: f64,
}

impl Remove for RenderItem {
    fn remove(&mut self) {
        match &mut self.item_type {
            RenderItemType::Empty => {}
            RenderItemType::Heading1(item) | RenderItemType::Paragraph(item) => item.remove(),
        }
    }
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
            item_type: RenderItemType::Heading1(Text::new_heading1()),
            x: 0.,
            y: 0.,
        }
    }

    pub fn new_paragraph() -> Self {
        Self {
            item_type: RenderItemType::Paragraph(Text::new_paragraph()),
            x: 0.,
            y: 0.,
        }
    }

    pub fn append_body(&mut self, content: &str) {
        match self.item_type {
            RenderItemType::Heading1(ref mut text) | RenderItemType::Paragraph(ref mut text) => {
                text.body.push_str(content)
            }
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
        match &self.item_type {
            RenderItemType::Heading1(text) | RenderItemType::Paragraph(text) => {
                text.label
                    .set_font(Font::with_name(&text.font, text.font_size));
                text.label.set_text(&text.body);
                text.label.set_text_color(Color::SystemBlack);
                context.add_subview(&text.label);
                LayoutConstraint::activate(&[
                    text.label
                        .left
                        .constraint_equal_to(&context.left)
                        .offset(self.x),
                    text.label
                        .top
                        .constraint_equal_to(&context.top)
                        .offset(self.y),
                ]);

                // the +5 is for vertical padding
                cursor.adjust(0., text.font_size + 5.);
            }
            _ => {}
        }
    }
}
