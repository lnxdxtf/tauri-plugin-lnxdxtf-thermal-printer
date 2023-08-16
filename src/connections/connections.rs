pub enum ConnectionsPrinter {
    TCP,
    Bluetooth,
    USB,
}

pub fn parser_conn_to_str(conn: ConnectionsPrinter) -> String {
    return match conn {
        ConnectionsPrinter::TCP => "TCP".to_string(),
        ConnectionsPrinter::Bluetooth => "Bluetooth".to_string(),
        ConnectionsPrinter::USB => "USB".to_string(),
    };
}
pub fn parser_conn_to_enum(conn: &str) -> ConnectionsPrinter {
    return match conn {
        "TCP" => ConnectionsPrinter::TCP,
        "Bluetooth" => ConnectionsPrinter::Bluetooth,
        "USB" => ConnectionsPrinter::USB,
        _ => ConnectionsPrinter::USB,
    };
}
