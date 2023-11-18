use cacao::{
    appkit::App,
    button::Button,
    color::Color,
    layout::{Layout, LayoutConstraint},
    view::View,
};
use gosub_engine::{
    bytes::{CharIterator, Confidence},
    html5::parser::{
        document::{Document, DocumentBuilder, DocumentHandle},
        Html5Parser,
    },
};

use crate::BasicApp;

pub enum BrowserTabAction {
    ClickedTab(usize),
    CloseTab(usize),
    AddTab,
}

#[derive(Debug)]
pub struct BrowserTab {
    pub(crate) name: String,
    pub(crate) tab: Button,
    pub(crate) close: Button,
    pub(crate) width: f64,
    pub(crate) padding: f64,
    pub(crate) document: DocumentHandle,
}

impl BrowserTab {
    pub fn new(name: &str, tab: Button, close: Button) -> Self {
        Self {
            name: name.to_owned(),
            tab,
            close,
            width: 200.,
            padding: 10.,
            document: DocumentBuilder::new_document(),
        }
    }

    pub fn set_html(&mut self, html: &str) {
        let mut char_iter = CharIterator::new();
        char_iter.read_from_str(html, Some(gosub_engine::bytes::Encoding::UTF8));
        char_iter.set_confidence(Confidence::Certain);

        // don't worry about parse errors in proof of concept
        let _ = Html5Parser::parse_document(&mut char_iter, Document::clone(&self.document), None);
    }
}

#[derive(Debug)]
pub struct BrowserTabs {
    pub(crate) tabs: Vec<BrowserTab>,
    pub(crate) new_tab: Box<Button>,
    starting_x: f64,
    starting_y: f64,
}

impl BrowserTabs {
    pub fn new() -> Self {
        let mut browser_tabs = BrowserTabs {
            tabs: Vec::new(),
            new_tab: Box::new(Button::new("+")),
            starting_x: 10.,
            starting_y: 75.,
        };

        browser_tabs.new_tab.set_bordered(false);
        browser_tabs
            .new_tab
            .set_bezel_style(cacao::button::BezelStyle::RoundRect);
        browser_tabs.new_tab.set_text_color(Color::SystemWhite);
        browser_tabs.new_tab.set_background_color(Color::SystemBlue);
        browser_tabs.new_tab.set_action(|_| {
            App::<BasicApp, BrowserTabAction>::dispatch_main(BrowserTabAction::AddTab)
        });

        browser_tabs
    }

    pub fn add_tab(&mut self, tab_name: &str) {
        let next_tab_idx = self.tabs.len();

        let mut tab_button = Button::new(tab_name);
        tab_button.set_action(move |_| {
            App::<BasicApp, BrowserTabAction>::dispatch_main(BrowserTabAction::ClickedTab(
                next_tab_idx,
            ))
        });

        let mut close_button = Button::new("X");
        close_button.set_bezel_style(cacao::button::BezelStyle::HelpButton);
        close_button.set_bordered(false);
        close_button.set_background_color(Color::SystemRed);
        close_button.set_action(move |_| {
            App::<BasicApp, BrowserTabAction>::dispatch_main(BrowserTabAction::CloseTab(
                next_tab_idx,
            ))
        });

        self.tabs
            .push(BrowserTab::new(tab_name, tab_button, close_button));

        // insert temporary HTML just to display something.
        let temp_html = format!("<html><h1>Tab {}</h1></html>", next_tab_idx);
        let tab_mut = self.tabs.last_mut().unwrap();
        tab_mut.set_html(&temp_html);
    }

    pub fn remove_tab(&mut self, tab_idx: usize) {
        let n_tabs = self.tabs.len();

        // do not close only existing tab. we could change this to close the window itself in the future?
        if n_tabs == 1 {
            return;
        }

        // tab idx is self-managed and (in theory) should never
        // be out of bounds
        self.tabs[tab_idx].tab.remove_from_superview();
        self.tabs[tab_idx].close.remove_from_superview();

        // update the action index for all tabs to the right of the closed tab
        // (otherwise you can get index out of bounds error)
        let mut new_tab_idx = tab_idx;
        if tab_idx < n_tabs - 1 {
            let mut right_tabs = &mut self.tabs[tab_idx + 1..n_tabs];
            for tab in right_tabs {
                tab.tab.set_action(move |_| {
                    App::<BasicApp, BrowserTabAction>::dispatch_main(BrowserTabAction::ClickedTab(
                        new_tab_idx,
                    ))
                });

                tab.close.set_action(move |_| {
                    App::<BasicApp, BrowserTabAction>::dispatch_main(BrowserTabAction::CloseTab(
                        new_tab_idx,
                    ))
                });

                new_tab_idx = new_tab_idx + 1;
            }
        }
        self.tabs.remove(tab_idx);
    }

    pub fn draw(&self, context: &View) {
        let mut start_x = self.starting_x;
        let start_y = self.starting_y;

        for tab in &self.tabs {
            let tab_button = &tab.tab;
            context.add_subview(tab_button);
            LayoutConstraint::activate(&[
                tab_button
                    .left
                    .constraint_equal_to(&context.left)
                    .offset(start_x),
                tab_button
                    .top
                    .constraint_equal_to(&context.top)
                    .offset(start_y),
                tab_button.width.constraint_equal_to_constant(tab.width),
            ]);
            start_x = start_x + tab.width + tab.padding;

            let close_button = &tab.close;
            context.add_subview(close_button);
            LayoutConstraint::activate(&[
                close_button
                    .right
                    .constraint_equal_to(&tab_button.right)
                    .offset(-4.),
                close_button
                    .height
                    .constraint_equal_to(&tab_button.height)
                    .offset(-16.),
                close_button.width.constraint_equal_to(&close_button.height),
                close_button
                    .center_y
                    .constraint_equal_to(&tab_button.center_y),
            ]);
        }

        let new_tab = &self.new_tab;
        new_tab.remove_from_superview();
        context.add_subview(new_tab.as_ref());
        LayoutConstraint::activate(&[
            new_tab
                .left
                .constraint_equal_to(&context.left)
                .offset(start_x),
            new_tab
                .top
                .constraint_equal_to(&context.top)
                .offset(start_y + 6.),
            new_tab.width.constraint_equal_to_constant(25.),
        ]);
    }
}
