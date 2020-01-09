use num_traits::pow;
pub mod machine;
use machine::*;
use std::cmp;

fn get_phase_settings(a: usize) -> [usize;5]
{
    [
        (a / pow(5,0)) % 5,
        (a / pow(5,1)) % 5,
        (a / pow(5,2)) % 5,
        (a / pow(5,3)) % 5,
        (a / pow(5,4)) % 5,
    ]
}

fn check_phase_settings(sett: [usize;5]) -> bool
{
    for a in 0..sett.len() {
        for b in a+1..sett.len() {
            if sett[a] == sett[b] { return false; }
        }
    }
    true
}

fn main() {
    run_asserts();
    let mut prog: Machine = Machine::new();
    let ints = load_machine_from_file(&mut prog, "input.txt");
    println!("Num ints read = {}", ints);

    // Part 1
    let mut max: isize = 0;
    for session_number in 0..(pow(5,5))
    {
        let phase_settings = get_phase_settings(session_number);
        if !check_phase_settings(phase_settings) { continue }
        let mut input: isize = 0;
        for machine_number in 0..5
        {
            let mut m: Machine = Machine::new();
            m.memcpy(&prog);
            m.put_input(phase_settings[machine_number] as isize);
            m.put_input(input);
            run_machine(&mut m);
            input = m.get_output().unwrap();
        }
        max = cmp::max(input, max);
    }
    println!("Max (part 1) = {}", max);
    assert!(max == 199988);

    // Part 2
    let mut max: isize = 0;
    for session_number in 0..(pow(5,5))
    {
        let mut phase_settings = get_phase_settings(session_number);
        if !check_phase_settings(phase_settings) { continue }
        for a in &mut phase_settings { *a += 5 }
        // println!("New phase settings: {:?}", phase_settings);
        let mut input: isize = 0;
        let mut m: [Machine; 5] = [
            Machine::new(),
            Machine::new(),
            Machine::new(),
            Machine::new(),
            Machine::new(),
        ];
        for machine_number in 0..5
        {
            m[machine_number].memcpy(&prog);
            m[machine_number].put_input(phase_settings[machine_number] as isize);
        }
        while !m[4].is_halted() {
            for machine_number in 0..5
            {
                m[machine_number].put_input(input);
                run_machine(&mut m[machine_number]);
                input = m[machine_number].get_output().unwrap();
            }
        }
        max = cmp::max(input, max);
    }
    println!("Max (part 2) = {}", max);
    assert!(max == 17519904);
}

fn run_asserts() {
    assert!(get_phase_settings(0) == [0,0,0,0,0]);
}
