use gtk::{prelude::*, Label};

pub struct Home {
    pub label: Label,
}
impl Home {
    pub fn new() -> Home {
        let label = Label::new(Some("Home"));
        Home { label }
    }
    pub fn render_all(&mut self, parent: &gtk::Box) {
        parent.pack_end(&self.label, true, true, 0);
    }
}

