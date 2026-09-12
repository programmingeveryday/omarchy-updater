mod theme;
mod checker;
mod runner;
mod ui;

use libadwaita as adw;
use adw::prelude::*;

const APP_ID: &str = "org.omarchy.updater";

fn main() {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(ui::build_ui);

    app.run();
}
