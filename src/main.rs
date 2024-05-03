mod client;

use std::fs;

use client::QuafuClient;

fn main() {
    let mut c = QuafuClient::new();

    c.load_credential();

    c.get_backends();

    if let Ok(qasm) = fs::read_to_string("/root/projects/quantum-computing-resources/benchmarks/QASMBench/small/bell_n4/bell_n4.qasm") {
        c.execute(&qasm, "", false);

    }

    println!("Hello, world!");
}
