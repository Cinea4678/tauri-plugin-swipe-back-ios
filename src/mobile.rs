use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_swipe_back_ios);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<SwipeBackIos<R>> {
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_swipe_back_ios)?;
  Ok(SwipeBackIos(handle))
}

/// Access to the swipe-back-ios APIs.
pub struct SwipeBackIos<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> SwipeBackIos<R> {
}
