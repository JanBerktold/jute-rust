extern crate jute_rust_codegen;

use jute_rust_codegen::Runner;

fn main() {
    Runner::new()
        .add_file("src/clean_zookeeper.jute".to_string())
        .set_output("src/generated.rs".to_string())
        .run()
        .expect("codegen to succeed");
}
