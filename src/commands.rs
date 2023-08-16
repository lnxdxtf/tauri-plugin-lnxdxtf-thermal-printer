use tauri::{command, AppHandle, Runtime, State, Window};

use crate::{
    connections::connections::{parser_conn_to_str, ConnectionsPrinter},
    PrinterState, Result,
};

#[command]
pub(crate) async fn execute<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, PrinterState>,
) -> Result<String> {
    // Connection State Initialization
    #[cfg(mobile)]
    let init_printer_state = parser_conn_to_str(ConnectionsPrinter::Bluetooth);

    #[cfg(desktop)]
    let init_printer_state = parser_conn_to_str(ConnectionsPrinter::USB);

    state
        .0
        .lock()
        .unwrap()
        .insert("connection".into(), init_printer_state.into());

    Ok("success".to_string())
}

#[command]
pub(crate) fn test<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, PrinterState>,
) {
    println!("test");
    let z = state.0.lock().unwrap();
    println!("{:?}", z);
}
