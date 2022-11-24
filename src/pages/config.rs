use gtk::{prelude::*, Label};

pub struct Config {
    pub label: Label,
}
impl Config {
    pub fn new() -> Config {
        let label = Label::new(Some("Config"));
        Config { label }
    }
    pub fn render_all(&mut self, parent: &gtk::Box) {
        parent.pack_end(&self.label, true, true, 0);
    }
}
