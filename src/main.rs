use browser_tabs::{BrowserTabAction, BrowserTabs};
use cacao::button::Button;
use cacao::color::Color;
use cacao::input::{TextField, TextFieldDelegate};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::notification_center::Dispatcher;
use cacao::view::View;

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};
use cacao::appkit::{App, AppDelegate};

use gosub_engine::html5::node::{Node, NodeTrait, NodeType};
use gosub_engine::html5::parser::document::{Document, DocumentHandle, TreeIterator};
use render_cursor::RenderCursor;
use render_item::{Remove, RenderItem, RenderItemType};

pub mod browser_tabs;
pub mod render_cursor;
pub mod render_item;

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
    rendered_items: Vec<RenderItem>,
    input: TextField<ConsoleLogger>,
    back_button: Button,
    forward_button: Button,
    go_button: Button,
    content: View,
    render_window: View,
    pub(crate) browser_tabs: BrowserTabs,
}

impl AppWindow {
    pub fn new() -> Self {
        AppWindow {
            cursor: RenderCursor::new(),
            rendered_items: Vec::new(),
            input: TextField::with(ConsoleLogger),
            back_button: Button::new("<"),
            forward_button: Button::new(">"),
            go_button: Button::new("Go"),
            content: View::new(),
            render_window: View::new(),
            browser_tabs: BrowserTabs::new(),
        }
    }

    pub fn render_node(&mut self, force: bool, current_node: &Node) {
        if force || current_node.type_of() == NodeType::Element {
            if let Some(current_render_item) = &mut self.rendered_items.last_mut() {
                match &current_render_item.item_type {
                    RenderItemType::Empty => return,
                    RenderItemType::Heading1(text) | RenderItemType::Paragraph(text) => {
                        current_render_item.draw_label(&self.render_window, &mut self.cursor);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn render_html(&mut self, document: &DocumentHandle) {
        for render_item in &mut self.rendered_items {
            render_item.remove();
        }
        self.rendered_items.clear();
        self.cursor.reset();

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
                        self.rendered_items.push(RenderItem::new_heading1());
                        self.rendered_items
                            .last_mut()
                            .unwrap()
                            .place(self.cursor.x, self.cursor.y);
                    }
                    "p" => {
                        self.rendered_items.push(RenderItem::new_paragraph());
                        self.rendered_items
                            .last_mut()
                            .unwrap()
                            .place(self.cursor.x, self.cursor.y);
                    }
                    _ => {}
                },
                gosub_engine::html5::node::NodeData::Text(text_body) => {
                    let mut current_render_item = self.rendered_items.last_mut().unwrap();

                    match &mut current_render_item.item_type {
                        RenderItemType::Heading1(text) | RenderItemType::Paragraph(text) => {
                            current_render_item.append_body(text_body.value());
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
    }
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("Gosub Client");
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
                .offset(50.),
            self.render_window
                .width
                .constraint_equal_to(&self.content.width),
            self.render_window
                .height
                .constraint_equal_to(&self.content.height)
                .offset(-80.),
        ]);

        // draw browser tabs
        self.browser_tabs.add_tab("New Tab"); // there's at least one default tab
        self.browser_tabs.draw(&self.content);

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

impl Dispatcher for BasicApp {
    type Message = BrowserTabAction;
    fn on_ui_message(&self, message: Self::Message) {
        if let Some(delegate) = &self.window.delegate {
            /*
                NOTE: this is VERY hacky because cacao doesn't support
                mutable references to the window making it extremely difficult
                to modify state (such as browser tabs.)

                Even though this is "unsafe" code, it should not invoke any
                nasty behaviour with what we're doing.
            */
            let const_ptr = delegate as *const Box<AppWindow>;
            let mut_ptr = const_ptr as *mut Box<AppWindow>;

            match &message {
                BrowserTabAction::AddTab => unsafe {
                    let mutable_window = &mut *mut_ptr;
                    mutable_window.browser_tabs.add_tab("New Tab");
                    mutable_window.browser_tabs.draw(&mutable_window.content);
                },
                BrowserTabAction::CloseTab(idx) => unsafe {
                    let mutable_window = &mut *mut_ptr;
                    mutable_window.browser_tabs.remove_tab(*idx);
                    mutable_window.browser_tabs.draw(&mutable_window.content);
                },
                BrowserTabAction::ClickedTab(idx) => unsafe {
                    let mutable_window = &mut *mut_ptr;
                    let doc_clone =
                        Document::clone(&mutable_window.browser_tabs.tabs[*idx].document);
                    mutable_window.render_html(&doc_clone);
                },
            }
        }
    }
}
