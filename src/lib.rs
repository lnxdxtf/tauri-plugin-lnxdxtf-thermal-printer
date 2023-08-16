use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

pub use models::*;
mod connections;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::LnxdxtfThermalPrinter;
#[cfg(mobile)]
use mobile::LnxdxtfThermalPrinter;

#[derive(Default)]
struct PrinterState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the lnxdxtf-thermal-printer APIs.
pub trait LnxdxtfThermalPrinterExt<R: Runtime> {
    fn lnxdxtf_thermal_printer(&self) -> &LnxdxtfThermalPrinter<R>;
}

impl<R: Runtime, T: Manager<R>> crate::LnxdxtfThermalPrinterExt<R> for T {
    fn lnxdxtf_thermal_printer(&self) -> &LnxdxtfThermalPrinter<R> {
        self.state::<LnxdxtfThermalPrinter<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("lnxdxtf-thermal-printer")
        .invoke_handler(tauri::generate_handler![commands::execute, commands::test])
        .setup(|app, api| {
            #[cfg(mobile)]
            let lnxdxtf_thermal_printer = mobile::init(app, api)?;

            #[cfg(desktop)]
            let lnxdxtf_thermal_printer = desktop::init(app, api)?;
            app.manage(lnxdxtf_thermal_printer);

            // manage state so it is accessible by the commands
            app.manage(PrinterState::default());
            Ok(())
        })
        .build()
}
