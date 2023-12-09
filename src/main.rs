use adw::{
    prelude::{ActionRowExt, MessageDialogExt, PreferencesRowExt},
    ActionRow, Application, ToastOverlay,
};
use gtk::{
    glib, Align, ApplicationWindow, Notebook, NotebookPage, NotebookTab, Orientation, PositionType,
};
use gtk::{prelude::*, subclass::box_};

mod distrobox_handler;
use distrobox_handler::*;

mod utils;
use utils::{get_distro_img, has_distrobox_installed};

const APP_ID: &str = "io.github.Dvlv.BoxBuddyRs";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("BoxBuddy")
        .build();

    window.set_default_size(800, 450);

    make_titlebar(&window);

    let toast_overlay = ToastOverlay::new();
    let main_box = gtk::Box::new(Orientation::Vertical, 10);
    main_box.set_orientation(Orientation::Vertical);
    main_box.set_hexpand(true);
    main_box.set_vexpand(true);
    main_box.set_margin_top(10);
    main_box.set_margin_bottom(10);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    toast_overlay.set_child(Some(&main_box));
    window.set_child(Some(&toast_overlay));

    if has_distrobox_installed() {
        load_boxes(&main_box, &window);
    } else {
        render_not_installed(&main_box);
    }

    // Present window
    window.present();
}

fn make_titlebar(window: &ApplicationWindow) {
    let add_btn = gtk::Button::from_icon_name("list-add-symbolic");
    add_btn.set_tooltip_text(Some("Create A Distrobox"));
    add_btn.connect_clicked(|_btn| create_new_distrobox());

    let about_btn = gtk::Button::from_icon_name("help-about-symbolic");
    about_btn.set_tooltip_text(Some("About BoxBuddy"));
    about_btn.connect_clicked(|_btn| show_about_popup());

    let title_lbl = gtk::Label::new(Some("BoxBuddy"));
    title_lbl.add_css_class("header");

    let titlebar = adw::HeaderBar::builder().title_widget(&title_lbl).build();

    titlebar.pack_start(&add_btn);
    titlebar.pack_end(&about_btn);

    window.set_titlebar(Some(&titlebar))
}

fn render_not_installed(main_box: &gtk::Box) {
    let not_installed_lbl = gtk::Label::new(Some("Distrobox not found!"));
    not_installed_lbl.add_css_class("title-1");

    let not_installed_lbl_two = gtk::Label::new(Some(
        "Distrobox could not be found, please ensure it is installed!",
    ));
    not_installed_lbl_two.add_css_class("title-2");

    main_box.append(&not_installed_lbl);
    main_box.append(&not_installed_lbl_two);
}

fn load_boxes(main_box: &gtk::Box, window: &ApplicationWindow) {
    let tabs = Notebook::new();
    tabs.set_tab_pos(PositionType::Left);
    tabs.set_hexpand(true);
    tabs.set_vexpand(true);

    let boxes = get_all_distroboxes();

    for dbox in boxes.iter() {
        let tab = make_box_tab(&dbox, window);
        // TODO shouldnt this be in make_box_tab
        tab.set_hexpand(true);
        tab.set_vexpand(true);

        let tab_title = gtk::Box::new(Orientation::Horizontal, 5);
        let tab_title_lbl = gtk::Label::new(Some(&dbox.name));
        let tab_title_img = gtk::Label::new(None);
        tab_title_img.set_markup(&get_distro_img(&dbox.distro));

        tab_title.append(&tab_title_img);
        tab_title.append(&tab_title_lbl);

        tabs.append_page(&tab, Some(&tab_title));
    }

    main_box.append(&tabs);
}

fn make_box_tab(dbox: &DBox, window: &ApplicationWindow) -> gtk::Box {
    let box_name = dbox.name.clone();

    let tab_box = gtk::Box::new(Orientation::Vertical, 15);
    tab_box.set_hexpand(true);

    tab_box.set_margin_top(10);
    tab_box.set_margin_bottom(10);
    tab_box.set_margin_start(10);
    tab_box.set_margin_end(10);

    //title
    let page_img = gtk::Label::new(None);
    page_img.set_markup(&get_distro_img(&dbox.distro));
    let page_title = gtk::Label::new(Some(&dbox.name));
    page_title.add_css_class("title-1");

    let page_status = gtk::Label::new(Some(&dbox.status));
    page_status.set_halign(Align::End);
    page_status.set_hexpand(true);

    let title_box = gtk::Box::new(Orientation::Horizontal, 10);
    title_box.set_margin_start(10);
    title_box.append(&page_img);
    title_box.append(&page_title);
    title_box.append(&page_status);

    // list view
    let boxed_list = gtk::ListBox::new();
    boxed_list.add_css_class("boxed-list");

    // terminal button
    let open_terminal_button = gtk::Button::from_icon_name("utilities-terminal-symbolic");
    open_terminal_button.add_css_class("flat");

    let term_bn_clone = box_name.clone();
    open_terminal_button
        .connect_clicked(move |_btn| on_open_terminal_clicked(term_bn_clone.clone()));

    let open_terminal_row = ActionRow::new();
    open_terminal_row.set_title("Open Terminal");
    open_terminal_row.add_suffix(&open_terminal_button);
    open_terminal_row.set_activatable_widget(Some(&open_terminal_button));

    // upgrade button
    let upgrade_button = gtk::Button::from_icon_name("software-update-available-symbolic");
    upgrade_button.add_css_class("flat");

    let up_bn_clone = box_name.clone();
    upgrade_button.connect_clicked(move |_btn| on_upgrade_clicked(up_bn_clone.clone()));

    let upgrade_row = ActionRow::new();
    upgrade_row.set_title("Upgrade Box");
    upgrade_row.add_suffix(&upgrade_button);
    upgrade_row.set_activatable_widget(Some(&upgrade_button));

    // show applications button
    let show_applications_button = gtk::Button::from_icon_name("application-x-executable-symbolic");
    show_applications_button.add_css_class("flat");

    let show_bn_clone = box_name.clone();
    show_applications_button
        .connect_clicked(move |_btn| on_show_applications_clicked(show_bn_clone.clone()));

    let show_applications_row = ActionRow::new();
    show_applications_row.set_title("View Applications");
    show_applications_row.add_suffix(&show_applications_button);
    show_applications_row.set_activatable_widget(Some(&show_applications_button));

    // Delete Button
    let delete_button = gtk::Button::from_icon_name("user-trash-symbolic");
    delete_button.add_css_class("flat");

    let del_bn_clone = box_name.clone();
    let win_clone = window.clone();
    delete_button.connect_clicked(move |_btn| on_delete_clicked(&win_clone, del_bn_clone.clone()));

    let delete_row = ActionRow::new();
    delete_row.set_title("Delete Box");
    delete_row.add_suffix(&delete_button);
    delete_row.set_activatable_widget(Some(&delete_button));

    // put all into list
    boxed_list.append(&open_terminal_row);
    boxed_list.append(&upgrade_row);
    boxed_list.append(&show_applications_row);
    boxed_list.append(&delete_row);

    tab_box.append(&title_box);
    tab_box.append(&gtk::Separator::new(Orientation::Horizontal));
    tab_box.append(&boxed_list);

    tab_box
}

// callbacks
fn create_new_distrobox() {
    println!("Create new DB clicked");
}
fn show_about_popup() {
    println!("About clicked");
}
fn on_open_terminal_clicked(box_name: String) {
    open_terminal_in_box(box_name);
}
fn on_upgrade_clicked(box_name: String) {
    upgrade_box(box_name)
}
fn on_show_applications_clicked(box_name: String) {
    println!("Show applications clicked");
}
fn on_delete_clicked(window: &ApplicationWindow, box_name: String) {
    let d = adw::MessageDialog::new(
        Some(window),
        Some("Really Delete?"),
        Some(&format!("Are you sure you want to delete {box_name}?")),
    );
    d.set_transient_for(Some(window));
    d.add_response("cancel", "Cancel");
    d.add_response("delete", "Delete");
    d.set_default_response(Some("cancel"));
    d.set_close_response("cancel");
    d.set_response_appearance("delete", adw::ResponseAppearance::Destructive);

    let win_clone = window.clone();

    d.connect_response(None, move |d, res| {
        if res == "delete" {
            delete_box(box_name.clone());
            d.destroy();

            let toast = adw::Toast::new("Box Deleted!");
            if let Some(child) = win_clone.clone().child() {
                let toast_area = child.downcast::<ToastOverlay>(); 
                toast_area.unwrap().add_toast(toast);
            }
        }
    });

    d.present()
}
