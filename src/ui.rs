use gtk4::prelude::*;
use libadwaita::prelude::*;
use libadwaita as adw;
use gtk4 as gtk;
use gtk::glib;
use crate::checker::{scan_system_updates, UpdateReport};
use crate::theme::OmarchyColors;
use crate::runner::launch_update_in_terminal;

pub fn build_ui(app: &adw::Application) {
    let colors = OmarchyColors::load_current();
    let css = colors.to_gtk_css();

    let provider = gtk::CssProvider::new();
    provider.load_from_string(&css);
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Omarchy System Updates")
        .default_width(680)
        .default_height(580)
        .build();

    window.add_css_class("omarchy-updater");

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Header Bar
    let header = adw::HeaderBar::new();
    let refresh_btn = gtk::Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text("Check for updates")
        .build();
    header.pack_end(&refresh_btn);
    main_box.append(&header);

    // Scrolled window for content
    let scrolled = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .build();

    let content_box = gtk::Box::new(gtk::Orientation::Vertical, 16);
    content_box.set_margin_top(24);
    content_box.set_margin_bottom(24);
    content_box.set_margin_start(24);
    content_box.set_margin_end(24);

    // Banner card
    let banner_card = gtk::Box::new(gtk::Orientation::Vertical, 10);
    banner_card.add_css_class("update-card");

    let title_label = gtk::Label::builder()
        .label("Omarchy System Status")
        .css_classes(["main-header"])
        .xalign(0.0)
        .build();

    let status_badge = gtk::Label::builder()
        .label("Checking system status...")
        .css_classes(["badge-up-to-date"])
        .halign(gtk::Align::Start)
        .build();

    let kernel_info = format!(
        "Kernel: {} | Omarchy Linux",
        std::process::Command::new("uname")
            .arg("-r")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Linux".to_string())
    );
    let subtitle_label = gtk::Label::builder()
        .label(&kernel_info)
        .xalign(0.0)
        .build();

    banner_card.append(&title_label);
    banner_card.append(&subtitle_label);
    banner_card.append(&status_badge);

    content_box.append(&banner_card);

    // Action button area
    let action_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    action_box.set_halign(gtk::Align::Center);

    let update_btn = gtk::Button::builder()
        .label("Update Now")
        .css_classes(["btn-update"])
        .sensitive(false)
        .build();

    action_box.append(&update_btn);
    content_box.append(&action_box);

    // Preferences Groups for details
    let details_group = adw::PreferencesGroup::builder()
        .title("Repositories & Subsystems")
        .description("Overview of system packages, AUR, firmware, and toolchains")
        .build();

    // Arch Row
    let arch_row = adw::ActionRow::builder()
        .title("Official Arch Repositories")
        .subtitle("Scanning...")
        .build();
    let arch_img = gtk::Image::from_icon_name("system-software-install-symbolic");
    arch_row.add_prefix(&arch_img);
    details_group.add(&arch_row);

    // AUR Row
    let aur_row = adw::ActionRow::builder()
        .title("Arch User Repository (AUR)")
        .subtitle("Scanning...")
        .build();
    let aur_img = gtk::Image::from_icon_name("software-properties-symbolic");
    aur_row.add_prefix(&aur_img);
    details_group.add(&aur_row);

    // Omarchy Core
    let omarchy_row = adw::ActionRow::builder()
        .title("Omarchy Core")
        .subtitle("Scanning...")
        .build();
    let omarchy_img = gtk::Image::from_icon_name("preferences-system-symbolic");
    omarchy_row.add_prefix(&omarchy_img);
    details_group.add(&omarchy_row);

    // Hardware Firmware
    let firmware_row = adw::ActionRow::builder()
        .title("Hardware Firmware (LVFS)")
        .subtitle("Scanning...")
        .build();
    let fw_img = gtk::Image::from_icon_name("drive-harddisk-symbolic");
    firmware_row.add_prefix(&fw_img);
    details_group.add(&firmware_row);

    // Developer Tools
    let mise_row = adw::ActionRow::builder()
        .title("Developer Tools (mise)")
        .subtitle("Scanning...")
        .build();
    let mise_img = gtk::Image::from_icon_name("utilities-terminal-symbolic");
    mise_row.add_prefix(&mise_img);
    details_group.add(&mise_row);

    content_box.append(&details_group);
    scrolled.set_child(Some(&content_box));
    main_box.append(&scrolled);
    window.set_content(Some(&main_box));

    // Async Update Check logic
    let trigger_check = {
        let status_badge = status_badge.clone();
        let update_btn = update_btn.clone();
        let arch_row = arch_row.clone();
        let aur_row = aur_row.clone();
        let omarchy_row = omarchy_row.clone();
        let firmware_row = firmware_row.clone();
        let mise_row = mise_row.clone();

        move || {
            status_badge.set_label("Checking for updates...");
            status_badge.remove_css_class("badge-updates-available");
            status_badge.add_css_class("badge-up-to-date");
            update_btn.set_sensitive(false);

            arch_row.set_subtitle("Checking...");
            aur_row.set_subtitle("Checking...");
            omarchy_row.set_subtitle("Checking...");
            firmware_row.set_subtitle("Checking...");
            mise_row.set_subtitle("Checking...");

            let badge_clone = status_badge.clone();
            let btn_clone = update_btn.clone();
            let arch_clone = arch_row.clone();
            let aur_clone = aur_row.clone();
            let omarchy_clone = omarchy_row.clone();
            let fw_clone = firmware_row.clone();
            let mise_clone = mise_row.clone();

            let (sender, receiver) = async_channel::unbounded::<UpdateReport>();

            // Spawn background task
            std::thread::spawn(move || {
                let report = scan_system_updates();
                let _ = sender.send_blocking(report);
            });

            // Receive in GTK context
            glib::spawn_future_local(async move {
                if let Ok(report) = receiver.recv().await {
                    let total = report.total_updates();
                    if total == 0 {
                        badge_clone.set_label("✓ System is Fully Up to Date");
                        badge_clone.remove_css_class("badge-updates-available");
                        badge_clone.add_css_class("badge-up-to-date");
                        btn_clone.set_sensitive(false);
                    } else {
                        badge_clone.set_label(&format!("⚠ {} Updates Available", total));
                        badge_clone.remove_css_class("badge-up-to-date");
                        badge_clone.add_css_class("badge-updates-available");
                        btn_clone.set_sensitive(true);
                    }

                    if report.arch_updates.is_empty() {
                        arch_clone.set_subtitle("Up to date");
                    } else {
                        arch_clone.set_subtitle(&format!("{} packages pending update", report.arch_updates.len()));
                    }

                    if report.aur_updates.is_empty() {
                        aur_clone.set_subtitle("Up to date");
                    } else {
                        aur_clone.set_subtitle(&format!("{} packages pending update", report.aur_updates.len()));
                    }

                    if report.omarchy_update_needed {
                        omarchy_clone.set_subtitle("New Omarchy release available");
                    } else {
                        omarchy_clone.set_subtitle("Up to date");
                    }

                    if report.firmware_updates.is_empty() {
                        fw_clone.set_subtitle("All devices on latest firmware");
                    } else {
                        fw_clone.set_subtitle(&format!("{} firmware updates available", report.firmware_updates.len()));
                    }

                    if report.mise_updates.is_empty() {
                        mise_clone.set_subtitle("All toolchains up to date");
                    } else {
                        mise_clone.set_subtitle(&format!("{} tools can be updated", report.mise_updates.len()));
                    }
                }
            });
        }
    };

    // Connect refresh button
    {
        let trigger = trigger_check.clone();
        refresh_btn.connect_clicked(move |_| {
            trigger();
        });
    }

    // Connect Update Now button
    {
        let window_clone = window.clone();
        update_btn.connect_clicked(move |_| {
            if let Err(e) = launch_update_in_terminal() {
                let dialog = adw::MessageDialog::builder()
                    .transient_for(&window_clone)
                    .heading("Update Error")
                    .body(&e)
                    .build();
                dialog.add_response("ok", "OK");
                dialog.present();
            }
        });
    }

    // Run initial scan on startup
    trigger_check();

    window.present();
}
