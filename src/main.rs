use cacao::button::Button;
use cacao::color::Color;
use cacao::input::{TextField, TextFieldDelegate};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::view::View;

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};
use cacao::appkit::{App, AppDelegate};

use gosub_engine::bytes::{CharIterator, Confidence};
use gosub_engine::html5::node::{Node, NodeTrait, NodeType};
use gosub_engine::html5::parser::document::{Document, DocumentBuilder, TreeIterator};
use gosub_engine::html5::parser::Html5Parser;
use render_cursor::RenderCursor;
use render_item::{RenderItem, RenderItemType};

mod render_cursor;
mod render_item;

struct BasicApp {
    window: Window<AppWindow>,
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![
            Menu::new(
                "",
                vec![
                    MenuItem::Services,
                    MenuItem::Separator,
                    MenuItem::Hide,
                    MenuItem::HideOthers,
                    MenuItem::ShowAll,
                    MenuItem::Separator,
                    MenuItem::Quit,
                ],
            ),
            Menu::new("File", vec![MenuItem::CloseWindow]),
            Menu::new(
                "Edit",
                vec![
                    MenuItem::Undo,
                    MenuItem::Redo,
                    MenuItem::Separator,
                    MenuItem::Cut,
                    MenuItem::Copy,
                    MenuItem::Paste,
                    MenuItem::Separator,
                    MenuItem::SelectAll,
                ],
            ),
            Menu::new("View", vec![MenuItem::EnterFullScreen]),
            Menu::new(
                "Window",
                vec![
                    MenuItem::Minimize,
                    MenuItem::Zoom,
                    MenuItem::Separator,
                    MenuItem::new("Bring All to Front"),
                ],
            ),
            Menu::new("Help", vec![]),
        ]);

        App::activate();
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

#[derive(Debug, Default)]
pub struct ConsoleLogger;

impl TextFieldDelegate for ConsoleLogger {
    const NAME: &'static str = "ConsoleLogger";

    fn text_should_begin_editing(&self, value: &str) -> bool {
        println!("Should begin with value: {}", value);
        true
    }

    fn text_did_change(&self, value: &str) {
        println!("Did change to: {}", value);
    }

    fn text_did_end_editing(&self, value: &str) {
        println!("Ended: {}", value);
    }
}

#[derive(Debug)]
struct AppWindow {
    cursor: RenderCursor,
    current_render_item: RenderItem,
    input: TextField<ConsoleLogger>,
    back_button: Button,
    forward_button: Button,
    go_button: Button,
    content: View,
    render_window: View,
}

impl AppWindow {
    pub fn new() -> Self {
        AppWindow {
            cursor: RenderCursor::new(),
            current_render_item: RenderItem::new(),
            input: TextField::with(ConsoleLogger),
            back_button: Button::new("<"),
            forward_button: Button::new(">"),
            go_button: Button::new("Go"),
            content: View::new(),
            render_window: View::new(),
        }
    }

    pub fn render_node(&mut self, force: bool, current_node: &Node) {
        if force
            || current_node.type_of() == NodeType::Element
                && self.current_render_item.item_type != RenderItemType::Empty
        {
            match &self.current_render_item.item_type {
                RenderItemType::Heading1 { .. } | RenderItemType::Paragraph { .. } => {
                    self.current_render_item
                        .draw_label(&self.render_window, &mut self.cursor);
                }
                _ => {}
            }

            self.current_render_item.item_type = RenderItemType::Empty;
        }
    }
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("GosuB Client");
        window.set_minimum_content_size(300., 300.);

        self.content.add_subview(&self.input);
        self.content.add_subview(&self.back_button);
        self.content.add_subview(&self.forward_button);
        self.content.add_subview(&self.go_button);
        self.render_window.set_background_color(Color::white(255.));
        self.content.add_subview(&self.render_window);
        window.set_content_view(&self.content);

        // draw main buttons/URL bar at top of browser

        LayoutConstraint::activate(&[
            self.input
                .center_x
                .constraint_equal_to(&self.content.center_x),
            self.input
                .top
                .constraint_equal_to(&self.content.top)
                .offset(40.),
            self.input
                .width
                .constraint_equal_to(&self.content.width)
                .offset(-300.),
        ]);

        LayoutConstraint::activate(&[
            self.forward_button
                .right
                .constraint_equal_to(&self.input.left)
                .offset(-15.),
            self.forward_button.top.constraint_equal_to(&self.input.top),
            self.forward_button.width.constraint_equal_to_constant(50.),
        ]);

        LayoutConstraint::activate(&[
            self.back_button
                .right
                .constraint_equal_to(&self.forward_button.left)
                .offset(-15.),
            self.back_button.top.constraint_equal_to(&self.input.top),
            self.back_button.width.constraint_equal_to_constant(50.),
        ]);

        LayoutConstraint::activate(&[
            self.go_button
                .left
                .constraint_equal_to(&self.input.right)
                .offset(15.),
            self.go_button.top.constraint_equal_to(&self.input.top),
            self.go_button.width.constraint_equal_to_constant(50.),
        ]);

        LayoutConstraint::activate(&[
            self.render_window
                .left
                .constraint_equal_to(&self.content.left),
            self.render_window
                .top
                .constraint_equal_to(&self.input.bottom)
                .offset(15.),
            self.render_window
                .width
                .constraint_equal_to(&self.content.width),
            self.render_window
                .height
                .constraint_equal_to(&self.content.height)
                .offset(-80.),
        ]);

        // render some basic sample HTML just for proof of concept

        let sample_html = "\
            <html>\
                <h1>sample heading</h1>\
                <p>sample paragraph</p>\
                <p>another sample paragraph</p>\
            </html>";

        let mut char_iter = CharIterator::new();
        char_iter.read_from_str(sample_html, Some(gosub_engine::bytes::Encoding::UTF8));
        char_iter.set_confidence(Confidence::Certain);

        let document = DocumentBuilder::new_document();
        // don't worry about parse errors in proof of concept
        let _ = Html5Parser::parse_document(&mut char_iter, Document::clone(&document), None);

        let tree_iterator = TreeIterator::new(&document);

        let doc_read = document.get();

        // this reference_node is mainly needed for checking the very last node in the tree
        // to force render it. We initially set it to the root node as a dummy
        let mut reference_node: &Node = doc_read.get_root();

        for current_node_id in tree_iterator {
            let current_node = doc_read.get_node_by_id(current_node_id).unwrap();
            reference_node = current_node;

            self.render_node(false, &current_node);

            match &current_node.data {
                gosub_engine::html5::node::NodeData::Element(element) => match element.name() {
                    "h1" => {
                        self.current_render_item = RenderItem::new_heading1();
                        self.current_render_item.place(self.cursor.x, self.cursor.y);
                    }
                    "p" => {
                        self.current_render_item = RenderItem::new_paragraph();
                        self.current_render_item.place(self.cursor.x, self.cursor.y);
                    }
                    _ => {}
                },
                gosub_engine::html5::node::NodeData::Text(text_body) => {
                    match &self.current_render_item.item_type {
                        RenderItemType::Heading1 { .. } | RenderItemType::Paragraph { .. } => {
                            self.current_render_item.append_body(text_body.value());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // in the cases where this is the last element, it won't be rendered
        // unless we force it
        self.render_node(true, &reference_node);

        window.show();
    }
}

fn main() {
    App::new(
        "com.test.window",
        BasicApp {
            window: Window::with(WindowConfig::default(), AppWindow::new()),
        },
    )
    .run();
}
