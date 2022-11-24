use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Orientation::*, Stack, StackSidebar};
mod pages;
use pages::{config::Config, home::Home, run::Run};
const WIDTH: i32 = 500;
const HEIGHT: i32 = 300;

fn construct_stack() -> (Stack, StackSidebar) {
    let stack = Stack::new();
    let sidebar = StackSidebar::new();
    let (home, config, run) = (
        gtk::Box::new(Vertical, 0),
        gtk::Box::new(Vertical, 0),
        gtk::Box::new(Vertical, 0),
    );

    let (mut home_widgets, mut config_widgets, mut run_widgets) =
        (Home::new(), Config::new(), Run::new());

    home_widgets.render_all(&home);
    config_widgets.render_all(&config);
    run_widgets.render_all(&run);
    sidebar.set_stack(&stack);

    stack.add_titled(&home, "Home", "Home");
    stack.add_titled(&config, "Config", "Config");
    stack.add_titled(&run, "Run", "Run");

    (stack, sidebar)
}

fn main() {
    let app = Application::builder()
        .application_id("me.SathyaPramodh")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(WIDTH)
            .default_height(HEIGHT)
            .title("Resetti GUI")
            .build();
        let root = gtk::Box::new(Horizontal, 0);
        let (stack, sidebar) = construct_stack();
        root.add(&sidebar);
        root.add(&stack);
        win.add(&root);
        win.show_all();
    });
    app.run();
}
