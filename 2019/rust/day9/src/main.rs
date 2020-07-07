pub mod machine;
use machine::*;

fn main() {
    run_asserts();
    let mut m: Machine = Machine::new();
    m.set_terminal(true);
    m.set_interactive(true);
    let ints = load_machine_from_file(&mut m, "input.txt");
    println!("Num ints read = {}", ints);

    run_machine(&mut m);
}

fn run_asserts() {
    assert!(0 == 0);
}
