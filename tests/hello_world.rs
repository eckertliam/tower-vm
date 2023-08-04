use tower_vm::{Builder, TypeFlag};

#[test]
fn hello_world() {
    let mut builder = Builder::new();
    builder.set_type(TypeFlag::Char);
    builder.print(Some("Hello world!"));
    builder.halt();
    let mut machine = builder.build_machine();
    machine.execute();
    let out = machine.get_stream().unwrap();
    assert_eq!("Hello world!".to_string(), out);
}
