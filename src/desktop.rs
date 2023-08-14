use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<LnxdxtfThermalPrinter<R>> {
  Ok(LnxdxtfThermalPrinter(app.clone()))
}

/// Access to the lnxdxtf-thermal-printer APIs.
pub struct LnxdxtfThermalPrinter<R: Runtime>(AppHandle<R>);

impl<R: Runtime> LnxdxtfThermalPrinter<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
