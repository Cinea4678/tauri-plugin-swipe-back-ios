use tauri::{
  plugin::{Builder, TauriPlugin},
  Runtime,
};

#[cfg(target_os = "ios")]
use tauri::Manager;

pub use models::*;

#[cfg(target_os = "ios")]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};


#[cfg(target_os = "ios")]
use mobile::SwipeBackIos;

#[cfg(target_os = "ios")]
/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the swipe-back-ios APIs.
pub trait SwipeBackIosExt<R: Runtime> {
  fn swipe_back_ios(&self) -> &SwipeBackIos<R>;
}

#[cfg(target_os = "ios")]
impl<R: Runtime, T: Manager<R>> crate::SwipeBackIosExt<R> for T {
  fn swipe_back_ios(&self) -> &SwipeBackIos<R> {
    self.state::<SwipeBackIos<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("swipe-back-ios")
    .invoke_handler(tauri::generate_handler![])
    .setup(|_app, _api| {
      #[cfg(target_os = "ios")]
      {
        let swipe_back_ios = mobile::init(_app, _api)?;
        _app.manage(swipe_back_ios);
      }
      Ok(())
    })
    .build()
}
