mod client;

use client::QuafuClient;

fn main() {
    let mut c = QuafuClient::new();

    c.load_credential();

    c.get_backends();

    println!("Hello, world!");
}
