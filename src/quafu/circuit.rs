pub struct Operation {
    name: String,
    qubits: Vec<u32>,
    cbits: Vec<u32>,
    params: Vec<f64>,
}

impl Operation {
    fn to_qasm(&self) -> String {
        let mut qasm_str = self.name.clone();
        if !self.qubits.is_empty() {
            qasm_str.push_str(&format!(" q[{}]", self.qubits[0]));
            for q in &self.qubits[1..] {
                qasm_str.push_str(&format!(", q[{}]", q));
            }
        }
        if !self.cbits.is_empty() {
            qasm_str.push_str(&format!(" -> c[{}]", self.cbits[0]));
            for c in &self.cbits[1..] {
                qasm_str.push_str(&format!(", c[{}]", c));
            }
        }
        if !self.params.is_empty() {
            qasm_str.push_str(&format!("({});", self.params[0]));
        }
        qasm_str
    }
}

pub struct Circuit {
    num_qubits: u32,
    measures: Vec<(u32, u32)>,
    ops: Vec<Operation>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            num_qubits: 0,
            measures: Vec::new(),
            ops: Vec::new(),
        }
    }

    pub fn new_with_qubits(num_qubits: u32) -> Circuit {
        Circuit {
            num_qubits,
            measures: Vec::new(),
            ops: Vec::new(),
        }
    }

    pub fn num_qubits(&self) -> u32 {
        self.num_qubits
    }

    pub fn measurements(&self) -> &Vec<(u32, u32)> {
        &self.measures
    }

    pub fn check_qubit_overflow(&self, qubits: &[u32]) {
        for &q in qubits {
            if q >= self.num_qubits {
                panic!("qubit index exceeds the number of qubits in circuit");
            }
        }
    }

    pub fn add_single_qubit_fixed_gate(&mut self, gate_name: &str, qubit: u32) {
        self.check_qubit_overflow(&[qubit]);
        self.ops.push(Operation {
            name: gate_name.to_string(),
            qubits: vec![qubit],
            cbits: Vec::new(),
            params: Vec::new(),
        });
    }

    pub fn add_single_qubit_param_gate(&mut self, gate_name: &str, qubit: u32, theta: f64) {
        self.check_qubit_overflow(&[qubit]);
        self.ops.push(Operation {
            name: gate_name.to_string(),
            qubits: vec![qubit],
            cbits: Vec::new(),
            params: vec![theta],
        });
    }

    pub fn add_double_qubit_fixed_gate(&mut self, gate_name: &str, ctrl: u32, targ: u32) {
        self.check_qubit_overflow(&[ctrl, targ]);
        self.ops.push(Operation {
            name: gate_name.to_string(),
            qubits: vec![ctrl, targ],
            cbits: Vec::new(),
            params: Vec::new(),
        });
    }

    pub fn id(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("Id", qubit);
    }

    pub fn h(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("H", qubit);
    }

    pub fn x(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("X", qubit);
    }

    pub fn y(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("Y", qubit);
    }

    pub fn z(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("Z", qubit);
    }

    pub fn t(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("T", qubit);
    }

    pub fn tdg(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("Tdg", qubit);
    }

    pub fn s(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("S", qubit);
    }

    pub fn sdg(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("Sdg", qubit);
    }

    pub fn sx(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("SX", qubit);
    }

    pub fn sxdg(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("SXdg", qubit);
    }

    pub fn sy(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("SY", qubit);
    }

    pub fn sydg(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("SYdg", qubit);
    }

    pub fn w(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("W", qubit);
    }

    pub fn sw(&mut self, qubit: u32) {
        self.add_single_qubit_fixed_gate("SW", qubit);
    }

    pub fn rx(&mut self, qubit: u32, theta: f64) {
        self.add_single_qubit_param_gate("RX", qubit, theta);
    }

    pub fn ry(&mut self, qubit: u32, theta: f64) {
        self.add_single_qubit_param_gate("RY", qubit, theta);
    }

    pub fn rz(&mut self, qubit: u32, theta: f64) {
        self.add_single_qubit_param_gate("RZ", qubit, theta);
    }

    pub fn p(&mut self, qubit: u32, theta: f64) {
        self.add_single_qubit_param_gate("P", qubit, theta);
    }

    pub fn cnot(&mut self, ctrl: u32, targ: u32) {
        self.add_double_qubit_fixed_gate("CX", ctrl, targ);
    }

    pub fn cx(&mut self, ctrl: u32, targ: u32) {
        self.add_double_qubit_fixed_gate("CX", ctrl, targ);
    }

    pub fn cy(&mut self, ctrl: u32, targ: u32) {
        self.add_double_qubit_fixed_gate("CY", ctrl, targ);
    }

    pub fn cz(&mut self, ctrl: u32, targ: u32) {
        self.add_double_qubit_fixed_gate("CZ", ctrl, targ);
    }

    pub fn cs(&mut self, ctrl: u32, targ: u32) {
        self.add_double_qubit_fixed_gate("CS", ctrl, targ);
    }

    pub fn ct(&mut self, ctrl: u32, targ: u32) {
        self.add_double_qubit_fixed_gate("CT", ctrl, targ);
    }

    pub fn measure(&mut self, qubit_list: Option<Vec<u32>>, cbit_list: Option<Vec<u32>>) {
        if qubit_list.is_none() && cbit_list.is_none() {
            for i in 0..self.num_qubits {
                self.measures.push((i, i));
            }
            let buffer: Vec<u32> = (0..self.num_qubits).collect();
            self.ops.push(Operation {
                name: "Measure".to_string(),
                qubits: buffer.clone(),
                cbits: buffer,
                params: Vec::new(),
            });
            return;
        }

        if let (Some(qubits), Some(cbits)) = (qubit_list, cbit_list) {
            if qubits.len() != cbits.len() {
                panic!("Number of measured bits should equal to the number of classical bits");
            }

            for (q, c) in qubits.iter().zip(cbits.iter()) {
                self.measures.push((*q, *c));
            }
            self.ops.push(Operation {
                name: "Measure".to_string(),
                qubits: qubits,
                cbits: cbits,
                params: Vec::new(),
            });
        } else {
            panic!("Both qubit_list and cbit_list must be provided together or left empty");
        }
    }

    pub fn to_qasm(&self) -> String {
        let mut qasm_str = "OPENQASM 2.0;\ninclude \"qelib1.inc\";\n".to_string();
        qasm_str.push_str(&format!("qreg q[{}];\n", self.num_qubits));
        qasm_str.push_str(&format!("creg c[{}];\n", self.measures.len()));

        for op in &self.ops {
            qasm_str.push_str(&format!("{}\n", op.to_qasm()));
        }

        qasm_str
    }
}
