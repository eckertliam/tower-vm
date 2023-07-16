use tower_vm::{Builder, Value};

#[test]
fn hello_world() {
    let mut builder = Builder::new();
    let mut chars: Vec<Value> = "Hello world!".chars().map(|ch| Value::from(ch)).collect();
    builder.push_collect(chars.as_mut());
    for _ in 0..chars.len() {
        builder.print();
    }
    let code = builder.build();
    let mut machine = tower_vm::Machine::new();
    machine.write_to_stream();
    machine.execute(code);
    let out = machine.get_stream();
    assert_eq!("Hello world!".to_string(), out.unwrap());
}