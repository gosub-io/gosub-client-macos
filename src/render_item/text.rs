use cacao::{layout::Layout, text::Label};

use crate::render_item::Remove;

#[derive(Debug)]
pub struct Text {
    pub(crate) body: String,
    pub(crate) font: String,
    pub(crate) font_size: f64,
    pub(crate) label: Label,
}

impl Text {
    pub fn new_heading1() -> Self {
        Self {
            body: "".to_owned(),
            font: "Times New Roman".to_owned(),
            font_size: 32.,
            label: Label::new(),
        }
    }

    pub fn new_paragraph() -> Self {
        Self {
            body: "".to_owned(),
            font: "Times New Roman".to_owned(),
            font_size: 16.,
            label: Label::new(),
        }
    }

    pub fn set_font(&mut self, font: &str) {
        self.font = font.to_owned();
    }
}

impl Remove for Text {
    fn remove(&mut self) {
        self.label.remove_from_superview();
    }
}
