mod install;
mod list;
mod parse_name;
mod preview;
mod remove;
mod suggest_category;
mod update;

pub use install::install_appimage;
pub use list::list_installed_apps;
pub use parse_name::parse_app_name;
pub use preview::preview_install;
pub use remove::remove_installed_app;
pub use suggest_category::suggest_app_category;
pub use update::update_installed_app;
