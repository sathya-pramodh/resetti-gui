use gtk::{prelude::*, Label};

pub struct Run {
    pub label: Label,
}
impl Run {
    pub fn new() -> Run {
        let label = Label::new(Some("Run"));
        Run { label }
    }
    pub fn render_all(&mut self, parent: &gtk::Box) {
        parent.pack_end(&self.label, true, true, 0);
    }
}

