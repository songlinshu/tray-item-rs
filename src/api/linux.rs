use crate::TIError;
use libappindicator::{AppIndicator, AppIndicatorStatus};
use gtk::prelude::*;

pub(crate) struct TrayItemLinux {
    tray: AppIndicator,
    menu: gtk::Menu
}

impl TrayItemLinux {

    pub(crate) fn new(title: &str, icon: &str) -> Result<Self, TIError> {

        let mut t = Self {
            tray: AppIndicator::new(title, icon),
            menu: gtk::Menu::new()
        };

        t.set_icon(icon)?;

        Ok(t)

    }

    pub(crate) fn set_icon(&mut self, icon: &str) -> Result<(), TIError> {

        self.tray.set_icon(icon);
        self.tray.set_status(AppIndicatorStatus::Active);

        Ok(())

    }

    pub(crate) fn add_label(&mut self, label: &str) -> Result<(), TIError> {

        let item = gtk::MenuItem::new_with_label(label.as_ref());
        item.set_sensitive(false);
        self.menu.append(&item);
        self.menu.show_all();
        self.tray.set_menu(&mut self.menu);

        Ok(())

    }

    pub(crate) fn add_menu_item<F>(&mut self, label: &str, cb: F) -> Result<(), TIError>
        where F: Fn() -> () + Send + Sync + 'static {

        let item = gtk::MenuItem::new_with_label(label.as_ref());
        item.connect_activate(move |_| {
            cb();
        });
        self.menu.append(&item);
        self.menu.show_all();
        self.tray.set_menu(&mut self.menu);

        Ok(())

    }

}
