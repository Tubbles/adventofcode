use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub const PRINT_DEBUG: bool = false;

const MEM_LENGTH: usize = 10000;
const CL_RED: &str = "\x1B[34m";
const CL_FG: &str = "\x1B[0m";

pub const OPS: [OpInfo; 10] = [
    OpInfo {
        name: "ADD",
        id: 1,
        n_params: 3,
        _n_inouts: 0,
        func: op_add,
    }, // Addition
    OpInfo {
        name: "MULT",
        id: 2,
        n_params: 3,
        _n_inouts: 0,
        func: op_mult,
    }, // Multiplication
    OpInfo {
        name: "IN",
        id: 3,
        n_params: 1,
        _n_inouts: 1,
        func: op_in,
    }, // Input
    OpInfo {
        name: "OUT",
        id: 4,
        n_params: 1,
        _n_inouts: 1,
        func: op_out,
    }, // Output - print to screen
    OpInfo {
        name: "JIT",
        id: 5,
        n_params: 2,
        _n_inouts: 0,
        func: op_jit,
    }, // Jump-if-true
    OpInfo {
        name: "JIF",
        id: 6,
        n_params: 2,
        _n_inouts: 0,
        func: op_jif,
    }, // Jump-if-false
    OpInfo {
        name: "LESS",
        id: 7,
        n_params: 3,
        _n_inouts: 0,
        func: op_less,
    }, // Less than
    OpInfo {
        name: "EQ",
        id: 8,
        n_params: 3,
        _n_inouts: 0,
        func: op_eq,
    }, // Equals
    OpInfo {
        name: "RBASE",
        id: 9,
        n_params: 1,
        _n_inouts: 0,
        func: op_rbase,
    }, // Change relative base
    OpInfo {
        name: "HALT",
        id: 99,
        n_params: 0,
        _n_inouts: 0,
        func: op_halt,
    }, // End program
];

pub struct Machine {
    pos: usize,
    mem: [isize; MEM_LENGTH],
    len: usize,
    halted: bool,
    outputs: VecDeque<isize>,
    inputs: VecDeque<isize>,
    terminal: bool,
    interactive: bool,
    relative_base: isize,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            pos: 0,
            mem: [0; MEM_LENGTH],
            len: 0,
            halted: true,
            outputs: VecDeque::new(),
            inputs: VecDeque::new(),
            terminal: false, // Connect the output to the terminal
            interactive: false,
            relative_base: 0,
        }
    }
    pub fn put_input(&mut self, input: isize) {
        self.inputs.push_back(input);
    }
    pub fn get_output(&mut self) -> Option<isize> {
        self.outputs.pop_front()
    }
    pub fn set_terminal(&mut self, b: bool) {
        self.terminal = b;
    }
    pub fn set_interactive(&mut self, b: bool) {
        self.interactive = b;
    }
    pub fn memcpy(&mut self, other: &Machine) {
        self.mem = other.mem;
        self.len = other.len;
    }
    pub fn is_halted(&self) -> bool {
        self.halted && (self.mem[self.pos] == opinfo_from_name("HALT").unwrap().id)
    }
    pub fn reset(&mut self) {
        self.pos = 0;
    }
}

pub struct Op {
    pub id: isize,
    pub param_modes: Vec<ParamMode>,
}

pub struct OpInfo<'a> {
    pub name: &'a str,    // Name
    pub id: isize,        // ID
    pub n_params: usize,  // Number of parameters
    pub _n_inouts: usize, // Number of inouts
    pub func: fn(
        &mut Machine,
        Option<&mut Vec<usize>>, // Optional list of positions to print with color to terminal
    ) -> bool, // Returns whether to automatically increase the machine position
}

#[derive(Copy, Clone, PartialEq)]
pub enum ParamMode {
    Indirect, // aka. Position Mode
    Immediate,
    Relative,
}

pub fn to_param_mode(a: usize) -> Option<ParamMode> {
    match a {
        0 => Some(ParamMode::Indirect),
        1 => Some(ParamMode::Immediate),
        2 => Some(ParamMode::Relative),
        _ => None,
    }
}

pub fn parammode_to_string(mode: ParamMode) -> String {
    match mode {
        ParamMode::Indirect => "*".to_string(),
        ParamMode::Immediate => "#".to_string(),
        ParamMode::Relative => "~".to_string(),
    }
}

pub fn machine_pos_and_op_to_string(m: &Machine, // Machine to print
) -> String {
    let mut out: String = "".to_string();
    out += &format!("{:>6}Current pos = {}(/{}):\n", "", m.pos, m.len - 1);
    let op = split_opcode(m.mem[m.pos]);
    match op {
        None => {
            out += &format!("Unkown operation: {}\n", m.mem[m.pos]);
        }
        Some(op) => {
            let opinfo = opinfo_from_id(op.id).unwrap();
            out += &format!("{:>6}{} ({}) -> ", "", op.id, opinfo.name);
            for idx in 0..op.param_modes.len() {
                out += &parammode_to_string(op.param_modes[idx]);
                out += &format!("{} ", m.mem[m.pos + idx + 1]);
            }
            out += &format!("| ");
            for idx in 0..op.param_modes.len() {
                out += &format!(
                    "{} ",
                    m.mem[unroll_parammode(m, m.pos + idx + 1, op.param_modes[idx])]
                );
            }
        }
    }
    out
}

pub fn machine_mem_to_string(
    m: &Machine,                     // Machine to print
    v: Option<&Vec<usize>>,          // Vector of adresses to highlight
    range: Option<&BTreeSet<usize>>, // Positions to print, if `None', defaults to only the machine's current position
    radix: Option<usize>,            // The width of the print
) -> String {
    // if PRINT_DEBUG {println!("v is {:?}", v)}
    // Some input validation
    let new_vec = Vec::<usize>::new();
    let v: &Vec<usize> = match v {
        None => &new_vec,
        Some(_) => v.unwrap(),
    };
    // if PRINT_DEBUG {println!("v is {:?}", v)}
    let radix = match radix {
        None => 10,
        Some(0) => 10,
        Some(_) => radix.unwrap(),
    };

    fn get_is_highlighted(pos: usize, v: &Vec<usize>) -> (&'static str, &'static str) {
        // if PRINT_DEBUG {println!("HL: @{} <= {:?}", pos, v)}
        for vecpos in v {
            if *vecpos == pos {
                return (CL_RED, CL_FG);
            }
        }
        return ("", "");
    }
    let mut out: String = "".to_string();
    let mut rows: BTreeSet<usize> = BTreeSet::new();
    // We default to only the machine's current position
    rows.insert(m.pos / radix * radix); // Round down to nearest radix
    match range {
        Some(_) => {
            for pos in range.unwrap() {
                rows.insert(pos / radix * radix); // Round down to nearest radix
            }
        }
        _ => {}
    }

    // Format first row
    out += &format!("{:>20}+", "");
    for idx in 0..radix {
        out += &format!("{:>22}", idx);
    }

    for row in rows {
        out += &format!("\n{:>20}: ", row);
        for pos in row..row + radix {
            let hl = get_is_highlighted(pos, &v);
            let curr_pos = if pos == m.pos { "<" } else { " " };
            out += &format!(
                " {}{:>20}{}{}", // Print the mem
                hl.0, m.mem[pos], hl.1, curr_pos
            );
        }
    }
    out
}

pub fn machine_state_to_string(m: &Machine, // Machine to print
) -> String {
    let mut out: String = "".to_string();
    out += &format!("{:>6}Relative base = {}", "", m.relative_base);
    out
}

pub fn opinfo_from_id(id: isize) -> Option<&'static OpInfo<'static>> {
    for op in OPS.iter() {
        if op.id == id {
            return Some(op);
        }
    }
    return None;
}

pub fn opinfo_from_name(name: &'static str) -> Option<&OpInfo> {
    for op in OPS.iter() {
        if op.name == name {
            return Some(op);
        }
    }
    return None;
}

pub fn split_opcode(op: isize) -> Option<Op> {
    let mut digits: Vec<_> = op
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as isize)
        .collect();

    if PRINT_DEBUG {
        println!("{:?}", digits)
    }

    let mut opnum: isize;
    if digits.len() < 2 {
        opnum = digits.pop().unwrap();
    } else {
        opnum = digits.pop().unwrap();
        opnum += digits.pop().unwrap() * 10;
    }
    if PRINT_DEBUG {
        println!("opnum = {}", opnum)
    }
    let num_params = opinfo_from_id(opnum);
    let num_params = match num_params {
        None => {
            return None;
        } // Op does not exist
        Some(_) => num_params.unwrap().n_params,
    };
    if PRINT_DEBUG {
        println!("num_params = {}", num_params)
    }

    let mut param_modes: Vec<ParamMode> = Vec::new();
    for _ in 0..num_params {
        let param_mode: Option<isize> = digits.pop();
        if PRINT_DEBUG {
            println!("matching {:?}", param_mode)
        }
        match param_mode {
            Some(_) => param_modes.push(to_param_mode(param_mode.unwrap() as usize).unwrap()),
            None => param_modes.push(to_param_mode(0).unwrap()), // Missing digits should be leading zeroes
        }
    }

    // param_modes.reverse();
    Some(Op {
        id: opnum,
        param_modes: param_modes,
    })
}

pub fn load_machine_from_file(m: &mut Machine, file: &str) -> usize // Number of ints read
{
    let f = File::open(file).expect("Unable to open file");
    let f = BufReader::new(f);

    // Scan the lines of the file
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let splits: Vec<&str> = line.split(",").collect();
        for split in splits {
            m.mem[m.len] = split.parse::<isize>().expect("Unable to parse split");
            m.len += 1;
            if m.len == MEM_LENGTH {
                println!("Not enough memory to store program");
                std::process::exit(-1);
            }
        }
    }
    m.len
}

// Returns when the machine HALTs or on error
// Also returns when the machine is out of input values in non-terminal mode
pub fn run_machine(m: &mut Machine) {
    if m.len == 0 {
        println!("Error: Machine not loaded");
        return;
    }
    let mut highlight_pos: Vec<usize> = Vec::new();
    let mut rows: BTreeSet<usize> = BTreeSet::new();
    m.halted = false;
    loop {
        if PRINT_DEBUG {
            println!("\tmachine pc: {}", m.pos)
        }
        if m.interactive && m.terminal
        // Print machine
        {
            // Collect the previous highlights
            for pos in &highlight_pos {
                rows.insert(*pos);
            }

            // Collect the new operands
            let op = split_opcode(m.mem[m.pos]);
            match op {
                None => {}
                Some(_) => {
                    let param_modes = op.unwrap().param_modes;
                    for idx in 0..param_modes.len() {
                        let addr = match param_modes[idx] {
                            ParamMode::Indirect => m.mem[m.pos + idx + 1] as usize,
                            ParamMode::Immediate => m.pos + idx + 1,
                            ParamMode::Relative => {
                                (m.mem[m.pos + idx + 1] + m.relative_base) as usize
                            }
                        };
                        rows.insert(addr);
                    }
                }
            }

            // Print the machine
            println!(
                "{}",
                machine_mem_to_string(&m, Some(&highlight_pos), Some(&rows), None)
            );
            println!("{}", machine_state_to_string(&m));
            println!("{}", machine_pos_and_op_to_string(&m));

            println!("Press enter to execute next command");
            print!("or send RUN to exit interactive mode > ");
            std::io::stdout().flush().ok();
            let mut line = String::new();
            let _input = std::io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            if line == "RUN\n" {
                m.interactive = false;
            }
        }

        // Run one step of the machine
        if m.pos >= m.len {
            if m.terminal {
                println!("Machine reached end of memory, halted")
            }
            break;
        }
        match split_opcode(m.mem[m.pos]) {
            None => {
                println!("Error: Illegal operation");
                break;
            }
            Some(op) => {
                if m.interactive {
                    highlight_pos.clear();
                    rows.clear();
                }
                let auto_inc = (opinfo_from_id(op.id).unwrap().func)(m, Some(&mut highlight_pos));
                if auto_inc {
                    m.pos += op.param_modes.len() + 1;
                }
                if PRINT_DEBUG {
                    println!("highlight_pos is {:?}", highlight_pos)
                }
                if m.halted {
                    if m.terminal {
                        println!("Machine halted")
                    }
                    break;
                }
            }
        }
    }
}

fn unroll_parammode(
    m: &Machine,
    p: usize,        // The position to get
    mode: ParamMode, // How to get it
) -> usize {
    match mode {
        ParamMode::Indirect => m.mem[p] as usize,
        ParamMode::Immediate => p,
        ParamMode::Relative => (m.mem[p] + m.relative_base) as usize,
    }
}

fn op_add(m: &mut Machine, v: Option<&mut Vec<usize>>) -> bool {
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = m.mem[unroll_parammode(m, m.pos + 1, op.param_modes[0])];
    let operand2 = m.mem[unroll_parammode(m, m.pos + 2, op.param_modes[1])];
    let res = operand1 + operand2;

    let actual_pos = unroll_parammode(m, m.pos + 3, op.param_modes[2]);
    m.mem[actual_pos] = res;

    // Highlight mutated position
    if v.is_some() {
        v.unwrap().push(actual_pos);
    }

    true // Automatically increment the machine position
}

fn op_mult(m: &mut Machine, v: Option<&mut Vec<usize>>) -> bool {
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = m.mem[unroll_parammode(m, m.pos + 1, op.param_modes[0])];
    let operand2 = m.mem[unroll_parammode(m, m.pos + 2, op.param_modes[1])];
    let res = operand1 * operand2;

    let actual_pos = unroll_parammode(m, m.pos + 3, op.param_modes[2]);
    m.mem[actual_pos] = res;

    // Highlight mutated position
    if v.is_some() {
        v.unwrap().push(actual_pos);
    }

    true // Automatically increment the machine position
}

fn op_in(m: &mut Machine, v: Option<&mut Vec<usize>>) -> bool {
    if !m.terminal && m.inputs.is_empty() {
        m.halted = true;
        return false; // Do not automatically increment the machine position
    } // Halt to simulate that it needs more input

    let op = split_opcode(m.mem[m.pos]).unwrap();
    let actual_pos = unroll_parammode(m, m.pos + 1, op.param_modes[0]);

    let read = if m.terminal {
        print!(
            "INPUT@{}->{}{} > ",
            m.pos,
            parammode_to_string(op.param_modes[0]),
            m.mem[m.pos + 1]
        );
        std::io::stdout().flush().ok();
        let mut line = String::new();
        let _input = std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        line.trim_end()
            .parse::<isize>()
            .expect("Unable to parse line")
    } else {
        m.inputs.pop_front().unwrap()
    };

    m.mem[actual_pos] = read;

    // Highlight mutated position
    if v.is_some() {
        v.unwrap().push(actual_pos);
    }

    true // Automatically increment the machine position
}

fn op_out(m: &mut Machine, _v: Option<&mut Vec<usize>>) -> bool {
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let out = m.mem[unroll_parammode(m, m.pos + 1, op.param_modes[0])];

    if m.terminal {
        println!("OUTPUT@{} : {}", m.pos, out)
    }
    m.outputs.push_back(out);

    true // Automatically increment the machine position
}

fn op_jit(m: &mut Machine, _v: Option<&mut Vec<usize>>) -> bool {
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = m.mem[unroll_parammode(m, m.pos + 1, op.param_modes[0])];
    let operand2 = m.mem[unroll_parammode(m, m.pos + 2, op.param_modes[1])];

    if operand1 != 0 {
        m.pos = operand2 as usize;
        return false; // Do not automatically increment the machine position
    } else {
        return true; // Automatically increment the machine position
    }
}

fn op_jif(m: &mut Machine, _v: Option<&mut Vec<usize>>) -> bool {
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = m.mem[unroll_parammode(m, m.pos + 1, op.param_modes[0])];
    let operand2 = m.mem[unroll_parammode(m, m.pos + 2, op.param_modes[1])];

    if operand1 == 0 {
        m.pos = operand2 as usize;
        return false; // Do not automatically increment the machine position
    } else {
        return true; // Automatically increment the machine position
    }
}

fn op_less(m: &mut Machine, v: Option<&mut Vec<usize>>) -> bool {
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = m.mem[unroll_parammode(m, m.pos + 1, op.param_modes[0])];
    let operand2 = m.mem[unroll_parammode(m, m.pos + 2, op.param_modes[1])];
    let val = if operand1 < operand2 { 1 } else { 0 };

    let actual_pos = unroll_parammode(m, m.pos + 3, op.param_modes[2]);
    m.mem[actual_pos] = val;
    if v.is_some() {
        v.unwrap().push(actual_pos);
    }

    true // Automatically increment the machine position
}

fn op_eq(m: &mut Machine, v: Option<&mut Vec<usize>>) -> bool {
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = m.mem[unroll_parammode(m, m.pos + 1, op.param_modes[0])];
    let operand2 = m.mem[unroll_parammode(m, m.pos + 2, op.param_modes[1])];
    let val = if operand1 == operand2 { 1 } else { 0 };

    let actual_pos = unroll_parammode(m, m.pos + 3, op.param_modes[2]);
    m.mem[actual_pos] = val;
    if v.is_some() {
        v.unwrap().push(actual_pos);
    }

    true // Automatically increment the machine position
}

fn op_rbase(m: &mut Machine, _v: Option<&mut Vec<usize>>) -> bool {
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = m.mem[unroll_parammode(m, m.pos + 1, op.param_modes[0])];

    m.relative_base += operand1;
    // We keep this below for now, in case the above naive solution does not work (unclear from spec)
    // if operand1 > 0 {
    //     m.relative_base += operand1 as usize;
    // }
    // else {
    //     m.relative_base -= -operand1 as usize;
    // }

    true // Automatically increment the machine position
}

fn op_halt(m: &mut Machine, _v: Option<&mut Vec<usize>>) -> bool {
    m.halted = true;
    false // Do not automatically increment the machine position
}
