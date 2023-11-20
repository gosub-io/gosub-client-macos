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

use gosub_engine::render_tree::{RenderTree, TreeIterator};

pub mod browser_tabs;
pub mod draw;

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

struct AppWindow {
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
            input: TextField::with(ConsoleLogger),
            back_button: Button::new("<"),
            forward_button: Button::new(">"),
            go_button: Button::new("Go"),
            content: View::new(),
            render_window: View::new(),
            browser_tabs: BrowserTabs::new(),
        }
    }

    pub fn render_html(&self, render_tree: &RenderTree) {
        // TODO: need to implement a new way to "clear" the view, although
        // it would be nice to do modify cacao directly and add a new .remove_all_subviews()
        // method so we don't have to keep track

        let tree_iterator = TreeIterator::new(render_tree);

        for current_node in tree_iterator {
            match &current_node.borrow().node_type {
                gosub_engine::render_tree::NodeType::Text(text_node) => {
                    draw::draw_text(&self.render_window, &current_node.borrow(), text_node);
                }
                _ => { /* TODO: others when we add more types */ }
            }
        }
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
                    let render_tree = &mutable_window.browser_tabs.tabs[*idx].render_tree;
                    mutable_window.render_html(render_tree);
                },
            }
        }
    }
}
