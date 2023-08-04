use tower_vm::{Builder, Value};

#[test]
fn hello_world() {
    let mut builder = Builder::new();
    builder.print(Some("Hello world!"));
    builder.halt();
    let mut machine = builder.build_machine();
    machine.execute();
    let out = machine.get_stream();
    assert_eq!("Hello world!".to_string(), out);
}