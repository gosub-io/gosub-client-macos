//! This example showcases setting up a basic application and window, setting up some views to
//! work with autolayout, and some basic ways to handle colors.

use cacao::button::Button;
use cacao::color::Color;
use cacao::input::{TextField, TextFieldDelegate};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::text::{Font, Label};
use cacao::view::View;

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};
use cacao::appkit::{App, AppDelegate};

use gosub_engine::bytes::{CharIterator, Confidence};
use gosub_engine::html5::node::NodeId;
use gosub_engine::html5::parser::document::{Document, DocumentBuilder};
use gosub_engine::html5::parser::Html5Parser;

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
            input: TextField::with(ConsoleLogger),
            back_button: Button::new("<"),
            forward_button: Button::new(">"),
            go_button: Button::new("Go"),
            content: View::new(),
            render_window: View::new(),
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

        let default_font = "Times New Roman";
        let default_margin_px = 5.;

        let sample_html = "\
            <html>
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

        // gosub TODO: instead of reimplementing tree traversal, would be nice to just do
        // something like tree.next_node() to get the next tree-order node

        let mut node_stack: Vec<NodeId> = Vec::new();
        let root_id = document.get().get_root().id;
        node_stack.push(root_id);

        let doc_read = document.get();

        // starting render positions
        let render_x = default_margin_px;
        let mut render_y = default_margin_px;

        while let Some(current_node_id) = node_stack.pop() {
            let current_node = doc_read.get_node_by_id(current_node_id).unwrap();

            // this would obviously be its own (set of) functions, but for prototyping
            // i'm just dumping it here to make it easy
            match &current_node.data {
                gosub_engine::html5::node::NodeData::Element(element) => match element.name() {
                    "h1" => {
                        let h1 = Label::new();
                        // gosub TODO: if a text node is appended to an element, append the content in the NodeElement itself
                        // so we can do something like element.value() here.
                        h1.set_text("h1 placeholder");
                        h1.set_font(&Font::with_name(default_font, 32.));
                        h1.set_text_color(Color::SystemBlack);
                        self.render_window.add_subview(&h1);
                        LayoutConstraint::activate(&[
                            h1.left
                                .constraint_equal_to(&self.render_window.left)
                                .offset(render_x),
                            h1.top
                                .constraint_equal_to(&self.render_window.top)
                                .offset(render_y),
                        ]);

                        // 32 for font size, 2 for default element spacing
                        // (these would become global constants but again... prototyping)
                        render_y = render_y + 32. + 2.;
                    }
                    "p" => {
                        let p = Label::new();
                        p.set_text("p placeholder");
                        p.set_font(Font::with_name(default_font, 16.));
                        p.set_text_color(Color::SystemBlack);
                        self.render_window.add_subview(&p);
                        LayoutConstraint::activate(&[
                            p.left
                                .constraint_equal_to(&self.render_window.left)
                                .offset(render_x),
                            p.top
                                .constraint_equal_to(&self.render_window.top)
                                .offset(render_y),
                        ]);

                        // 16 for font size, 2 for default element spacing
                        // (these would become global constats but again... prototyping)
                        render_y = render_y + 16. + 2.;
                    }
                    _ => {}
                },
                _ => {}
            }

            if let Some(sibling_id) = doc_read.get_next_sibling(current_node_id) {
                node_stack.push(sibling_id);
            }

            if !current_node.children.is_empty() {
                node_stack.push(current_node.children[0]);
            }
        }

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
