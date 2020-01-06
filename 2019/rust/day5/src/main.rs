#[macro_use]
extern crate clap;
use clap::App;
mod machine;
use machine::*;

fn main() {
    let matches = App::new("AdventOfCode 2019 day5")
        .author("Tubbles")
        .version(crate_version!())
        .args_from_usage(
            "-i, --interactive 'Run the intcode machine in interactive mode'")
        .get_matches();

    let interactive = matches.is_present("interactive");

    if interactive || PRINT_DEBUG {println!("Running in {}{}mode",
        if interactive {"interactive "} else {""},
        if PRINT_DEBUG {"debug "} else {""}
    )}

    run_asserts();
    println!("Asserts passed");

    let mut m = Machine::new();
    m.set_terminal(true);
    m.set_interactive(interactive);
    load_machine_from_file(&mut m, "input.txt");
    m.put_input(1);
    run_machine(&mut m);
}

fn run_asserts()
{
    // assert!(opinfo_from_id(0).unwrap().name == "ILL");
    assert!(opinfo_from_id(1).unwrap().name == "ADD");
    assert!(opinfo_from_id(2).unwrap().name == "MULT");
    assert!(opinfo_from_id(3).unwrap().name == "IN");
    assert!(opinfo_from_id(4).unwrap().name == "OUT");
    assert!(opinfo_from_id(99).unwrap().name == "HALT");

    // assert!(opinfo_from_name("ILL").unwrap().id == 0);
    assert!(opinfo_from_name("ADD").unwrap().id == 1);
    assert!(opinfo_from_name("MULT").unwrap().id == 2);
    assert!(opinfo_from_name("IN").unwrap().id == 3);
    assert!(opinfo_from_name("OUT").unwrap().id == 4);
    assert!(opinfo_from_name("HALT").unwrap().id == 99);

    // assert!(split_opcode(0).unwrap().id == 0);
    assert!(split_opcode(1).unwrap().id == 1);
    assert!(split_opcode(2).unwrap().id == 2);
    assert!(split_opcode(3).unwrap().id == 3);
    assert!(split_opcode(4).unwrap().id == 4);
    assert!(split_opcode(99).unwrap().id == 99);

    let op = split_opcode(11101).unwrap();
    let mut modes = op.param_modes;
    assert!(modes.pop().unwrap() == ReadMode::Immediate);
    assert!(modes.pop().unwrap() == ReadMode::Immediate);
    assert!(modes.pop().unwrap() == ReadMode::Immediate);
    let op = split_opcode(1001).unwrap();
    let mut modes = op.param_modes;
    assert!(modes.pop().unwrap() == ReadMode::Indirect);
    assert!(modes.pop().unwrap() == ReadMode::Immediate);
    assert!(modes.pop().unwrap() == ReadMode::Indirect);
    let op = split_opcode(10101).unwrap();
    let mut modes = op.param_modes;
    assert!(modes.pop().unwrap() == ReadMode::Immediate);
    assert!(modes.pop().unwrap() == ReadMode::Indirect);
    assert!(modes.pop().unwrap() == ReadMode::Immediate);
    let op = split_opcode(00101).unwrap();
    let mut modes = op.param_modes;
    assert!(modes.pop().unwrap() == ReadMode::Indirect);
    assert!(modes.pop().unwrap() == ReadMode::Indirect);
    assert!(modes.pop().unwrap() == ReadMode::Immediate);
    let op = split_opcode(101).unwrap();
    let mut modes = op.param_modes;
    assert!(modes.pop().unwrap() == ReadMode::Indirect);
    assert!(modes.pop().unwrap() == ReadMode::Indirect);
    assert!(modes.pop().unwrap() == ReadMode::Immediate);

    let mut m = Machine::new();
    // m.set_terminal(true);
    // m.set_interactive(true);
    load_machine_from_file(&mut m, "input.txt");
    m.put_input(1); // Test the first program
    run_machine(&mut m);
    loop { // Read the results
        let out = m.get_output();
        if out == None { break }
        let out = out.unwrap();
        match out {
            0 => {} // 0 is OK
            _ => { assert!(out == 15314507) } // Non zero can only be the diagnostics code
        }
    }
}
