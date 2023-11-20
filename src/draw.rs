use cacao::{
    color::Color,
    layout::{Layout, LayoutConstraint},
    text::{Font, Label},
    view::View,
};
use gosub_engine::render_tree::{text::TextNode, Node};

pub fn draw_text(context: &View, node: &Node, text_node: &TextNode) {
    let text = Label::new();
    text.set_font(Font::with_name(
        text_node.font.as_str(),
        text_node.font_size,
    ));
    text.set_text(&text_node.value);
    text.set_text_color(Color::SystemBlack);
    context.add_subview(&text);
    LayoutConstraint::activate(&[
        text.left
            .constraint_equal_to(&context.left)
            .offset(node.position.x),
        text.top
            .constraint_equal_to(&context.top)
            .offset(node.position.y),
    ]);
}
