use std::fs;

use quafu::circuit::Circuit;
use quafu::client::QClient;

fn main() {
    let mut c = QClient::new();

    let _ = c.load_credential();

    let _ = c.get_backends();

    // if let Ok(qasm) = fs::read_to_string("/root/projects/quantum-computing-resources/benchmarks/QASMBench/small/bell_n4/bell_n4.qasm") {
    //     c.execute(&qasm, "", false);
    // }

    let mut q = Circuit::new_with_qubits(2);
    q.h(0);
    q.cx(0, 1);
    q.measure(None, None);

    println!("{}", q.to_qasm());

    c.execute(&q.to_qasm(), "", false);
}
