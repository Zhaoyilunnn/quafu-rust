use clap::Parser;
use std::fs;

use quafu::client::QClient;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to qasm file
    #[arg(short, long)]
    qasm: String,

    /// Backend name
    #[arg(short, long, default_value_t = String::from("Dongling"))]
    backend: String,
}

fn main() {
    let args = Args::parse();
    let mut c = QClient::new();
    let _ = c.load_credential();
    let _ = c.get_backends();

    if let Ok(qasm) = fs::read_to_string(args.qasm) {
        c.set_backend_name(args.backend);
        c.execute(&qasm, "", false);
    }
}
