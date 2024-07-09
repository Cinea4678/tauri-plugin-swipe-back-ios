use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<SwipeBackIos<R>> {
  Ok(SwipeBackIos(app.clone()))
}

/// Access to the swipe-back-ios APIs.
pub struct SwipeBackIos<R: Runtime>(AppHandle<R>);

impl<R: Runtime> SwipeBackIos<R> {
}
