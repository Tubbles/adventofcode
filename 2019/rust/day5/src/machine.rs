use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::cmp;
use std::io::Write;
use std::collections::BTreeSet;
use std::collections::VecDeque;

pub const PRINT_DEBUG: bool = false;
// pub const PRINT_DEBUG: bool = true;
const MEM_LENGTH: usize = 100000;
const CL_RED: &str = "\x1B[34m";
const CL_FG: &str = "\x1B[0m";

pub const OPS: [OpInfo; 9] = [
    OpInfo {name:"ADD", id:1, n_params:3, _n_inouts:0, func:op_add}, // Addition
    OpInfo {name:"MULT", id:2, n_params:3, _n_inouts:0, func:op_mult}, // Multiplication
    OpInfo {name:"IN", id:3, n_params:1, _n_inouts:1, func:op_in}, // Input
    OpInfo {name:"OUT", id:4, n_params:1, _n_inouts:1, func:op_out}, // Output - print to screen
    OpInfo {name:"JIT", id:5, n_params:2, _n_inouts:0, func:op_jit}, // Jump-if-true
    OpInfo {name:"JIF", id:6, n_params:2, _n_inouts:0, func:op_jif}, // Jump-if-false
    OpInfo {name:"LESS", id:7, n_params:3, _n_inouts:0, func:op_less}, // Less than
    OpInfo {name:"EQ", id:8, n_params:3, _n_inouts:0, func:op_eq}, // Equals
    OpInfo {name:"HALT", id:99, n_params:0, _n_inouts:0, func:op_halt}, // End program
];

pub struct Machine
{
    pos: usize,
    mem: [isize; MEM_LENGTH],
    len: usize,
    halted: bool,
    outputs: VecDeque<isize>,
    inputs: VecDeque<isize>,
    terminal: bool,
    interactive: bool,
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
}

pub struct Op
{
    pub id: isize,
    pub param_modes: Vec<ReadMode>,
}

pub struct OpInfo<'a>
{
    pub name: &'a str, // Name
    pub id: isize, // ID
    pub n_params: usize, // Number of parameters
    pub _n_inouts: usize, // Number of inouts
    pub func: fn(&mut Machine, &mut Vec<usize>)
}

#[derive(Copy, Clone, PartialEq)]
pub enum ReadMode
{
    Indirect,
    Immediate,
}

pub fn to_param_mode(a: isize) -> Option<ReadMode>
{
    match a
    {
        0 => Some(ReadMode::Indirect),
        1 => Some(ReadMode::Immediate),
        _ => None,
    }
}

pub fn machine_pos_and_op_to_string(
    m: &Machine, // Machine to print
) -> String
{
    let mut out: String = "".to_string();
    out += &format!("{:>6}Current pos = {}(/{}):\n", "", m.pos, m.len-1);
    let op = split_opcode(m.mem[m.pos]);
    match op {
        None => { out += &format!("Unkown operation: {}\n", m.mem[m.pos]); }
        Some(_) => {
            let op = op.unwrap();
            let opinfo = opinfo_from_id(op.id).unwrap();
            out += &format!("{:>6}{} ({})", "", op.id, opinfo.name);
            let mut evaled: String = "".to_string();
            for idx in 0..op.param_modes.len() {
                if idx == 0 { out += &format!(" -> ") }
                match op.param_modes[idx] {
                    ReadMode::Indirect => {
                        out += &format!("*");
                        evaled += &format!("{} ", m.mem[m.mem[m.pos+idx+1] as usize]);
                    }
                    ReadMode::Immediate => {
                        out += &format!("#");
                        evaled += &format!("{} ", m.mem[m.pos+idx+1]);
                    }
                }
                out += &format!("{} ", m.mem[m.pos+idx+1]);
            }
            out += &format!("| {}", evaled);
        }
    }
    out
}

pub fn machine_mem_to_string(
    m: &Machine, // Machine to print
    v: Option<&Vec<usize>>, // Vector of adresses to highlight
    range: Option<&BTreeSet<usize>>, // Positions to print, if `None', defaults to only the machine's current position
    radix: Option<usize>, // The width of the print
) -> String
{
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

    fn get_is_highlighted(pos: usize, v: &Vec<usize>)
        -> (&'static str, &'static str)
    {
        // if PRINT_DEBUG {println!("HL: @{} <= {:?}", pos, v)}
        for vecpos in v
        {
            if *vecpos == pos {return (CL_RED, CL_FG)}
        }
        return ("","")
    }
    let mut out: String = "".to_string();
    let mut rows : BTreeSet<usize> = BTreeSet::new();
    // We default to only the machine's current position
    rows.insert(m.pos/radix*radix); // Round down to nearest radix
    match range {
        Some(_) => {
            for pos in range.unwrap() {
                rows.insert(pos/radix*radix); // Round down to nearest radix
            }
        }
        _ => {}
    }

    // Format first row
    out += &format!("{:>5}+", "");
    for idx in 0..radix {
        out += &format!("{:>7}", idx);
    }

    for row in rows {
        out += &format!("\n{:>5}: ", row);
        for pos in row..row+radix {
            let hl = get_is_highlighted(pos, &v);
            let curr_pos = if pos == m.pos { "<" } else { " " };
            out += &format!(" {}{:>5}{}{}", // Print the mem
                hl.0,
                m.mem[pos],
                hl.1,
                curr_pos
            );
        }
    }
    out
}

pub fn opinfo_from_id(id: isize) -> Option<&'static OpInfo<'static>>
{
    for op in OPS.iter()
    {
        if op.id == id
        {
            return Some(op);
        }
    }
    return None;
}

pub fn opinfo_from_name(name: &'static str) -> Option<&OpInfo>
{
    for op in OPS.iter()
    {
        if op.name == name
        {
            return Some(op);
        }
    }
    return None;
}

pub fn split_opcode(op: isize) -> Option<Op>
{
    let mut digits : Vec<_> = op.to_string()
    .chars()
    .map(|d| d.to_digit(10).unwrap() as isize)
    .collect();

    if PRINT_DEBUG {println!("{:?}", digits)}

    let mut opnum : isize;
    if digits.len() < 2
    {
        opnum = digits.pop().unwrap();
    }
    else
    {
        opnum = digits.pop().unwrap();
        opnum += digits.pop().unwrap() * 10;
    }
    if PRINT_DEBUG {println!("opnum = {}", opnum)}
    let num_params = opinfo_from_id(opnum);
    let num_params = match num_params {
        None => { return None; }, // Op does not exist
        Some(_) => {num_params.unwrap().n_params},
    };
    if PRINT_DEBUG {println!("num_params = {}", num_params)}

    let mut param_modes : Vec<ReadMode> = Vec::new();
    for _ in 0..num_params
    {
        let param_mode : Option<isize> = digits.pop();
        if PRINT_DEBUG {println!("matching {:?}", param_mode)}
        match param_mode
        {
            Some(_) => param_modes.push(to_param_mode(param_mode.unwrap()).unwrap()),
            None => param_modes.push(to_param_mode(0).unwrap()), // Missing digits should be leading zeroes
        }
    }

    // param_modes.reverse();
    Some(Op {id:opnum, param_modes:param_modes})
}

pub fn load_machine_from_file(
    m: &mut Machine,
    file: &str,
) -> usize // Number of ints read
{
    let f = File::open(file).expect("Unable to open file");
    let f = BufReader::new(f);

    // Scan the lines of the file
    for line in f.lines()
    {
        let line = line.expect("Unable to read line");
        let splits : Vec<&str> = line.split(",").collect();
        for split in splits
        {
            m.mem[m.len] = split.parse::<isize>().expect("Unable to parse split");
            m.len += 1;
            if m.len == MEM_LENGTH
            {
                println!("Not enough memory to store program");
                std::process::exit(-1);
            }
        }
    }
    m.len
}

pub fn run_machine(m: &mut Machine) // Returns when the machine HALTs or on error
{
    let mut highlight_pos: Vec<usize> = Vec::new();
    let mut rows: BTreeSet<usize> = BTreeSet::new();
    m.halted = false;
    loop
    {
        if PRINT_DEBUG {println!("\tmachine pc: {}", m.pos)}
        if m.interactive && m.terminal // Print machine
        {
            // Collect the previous highlights
            for pos in &highlight_pos {
                rows.insert(*pos);
            }

            // Collect the new operands
            let op = split_opcode(m.mem[m.pos]);
            match op {
                None => {},
                Some(_) => {
                    let param_modes = op.unwrap().param_modes;
                    for idx in 0..param_modes.len() {
                        let addr = match param_modes[idx] {
                            ReadMode::Indirect => {
                                m.mem[m.pos+1+idx] as usize
                            }
                            ReadMode::Immediate => {
                                m.pos+1+idx
                            }
                        };
                        rows.insert(addr);
                    }
                }
            }

            // Print the machine
            println!("{}", machine_mem_to_string(
                &m, Some(&highlight_pos), Some(&rows), None));
            println!("{}", machine_pos_and_op_to_string(&m));

            println!("Press enter to execute next command");
            print!("or send RUN to exit interactive mode > ");
            std::io::stdout().flush().ok();
            let mut line = String::new();
            let _input = std::io::stdin().read_line(&mut line).expect("Failed to read line");
            if line == "RUN\n" { m.interactive = false; }
        }

        // Run one step of the machine
        if m.pos >= m.len {
            if m.terminal { println!("Machine reached end of memory, halted") }
            break;
        }
        let op = split_opcode(m.mem[m.pos]);
        match op {
            None => {
                println!("Error: Illegal operation");
                break;
            },
            _ => {
                if m.interactive {
                    highlight_pos.clear();
                    rows.clear();
                }
                (opinfo_from_id((&op).as_ref().unwrap().id).unwrap().func)(m, &mut highlight_pos);
                if PRINT_DEBUG {println!("highlight_pos is {:?}", highlight_pos)}
                if m.halted {
                    if m.terminal { println!("Machine halted") }
                    break;
                }
            }
        }
    }
}

fn read_value(
    m: &Machine,
    p: usize, // The position to get
    mode: ReadMode, // How to get it
) -> isize
{
    match mode {
        ReadMode::Indirect => {
            m.mem[m.mem[p] as usize]
        }
        ReadMode::Immediate => {
            m.mem[p]
        }
    }
}

fn op_add(m: &mut Machine, v: &mut Vec<usize>)
{
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = read_value(m, m.pos+1, op.param_modes[0]);
    let operand2 = read_value(m, m.pos+2, op.param_modes[1]);
    m.mem[m.mem[m.pos+3] as usize] = operand1 + operand2;
    v.push(m.mem[m.pos+3] as usize);
    m.pos += op.param_modes.len() + 1;
}

fn op_mult(m: &mut Machine, v: &mut Vec<usize>)
{
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = read_value(m, m.pos+1, op.param_modes[0]);
    let operand2 = read_value(m, m.pos+2, op.param_modes[1]);
    m.mem[m.mem[m.pos+3] as usize] = operand1 * operand2;
    v.push(m.mem[m.pos+3] as usize);
    m.pos += op.param_modes.len() + 1;
}

fn op_in(m: &mut Machine, v: &mut Vec<usize>)
{
    fn read_from_terminal(m: &Machine) -> isize {
        print!("INPUT@{}->*{} > ", m.pos, m.mem[m.pos+1]);
        std::io::stdout().flush().ok();
        let mut line = String::new();
        let _input = std::io::stdin().read_line(&mut line).expect("Failed to read line");
        line.trim_end()
            .parse::<isize>()
            .expect("Unable to parse line")
    }

    let op = split_opcode(m.mem[m.pos]).unwrap();
    let read = 
        if m.terminal || m.inputs.is_empty()
            { read_from_terminal(m) }
        else
            { m.inputs.pop_front().unwrap() };

    m.mem[m.mem[m.pos+1] as usize] = read;
    v.push(m.mem[m.pos+1] as usize);
    m.pos += op.param_modes.len() + 1;
}

fn op_out(m: &mut Machine, _v: &mut Vec<usize>)
{
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let out = read_value(&m, m.pos+1, op.param_modes[0]);

    if m.terminal { println!("OUTPUT@{} : {}", m.pos, out) }
    m.outputs.push_back(out);
    m.pos += op.param_modes.len() + 1;
}

fn op_jit(m: &mut Machine, _v: &mut Vec<usize>)
{
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = read_value(m, m.pos+1, op.param_modes[0]);
    let operand2 = read_value(m, m.pos+2, op.param_modes[1]);

    if operand1 != 0 {
        m.pos = operand2 as usize;
    }
    else {
        m.pos += op.param_modes.len() + 1;
    }
}

fn op_jif(m: &mut Machine, _v: &mut Vec<usize>)
{
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = read_value(m, m.pos+1, op.param_modes[0]);
    let operand2 = read_value(m, m.pos+2, op.param_modes[1]);

    if operand1 == 0 {
        m.pos = operand2 as usize;
    }
    else {
        m.pos += op.param_modes.len() + 1;
    }
}

fn op_less(m: &mut Machine, v: &mut Vec<usize>)
{
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = read_value(m, m.pos+1, op.param_modes[0]);
    let operand2 = read_value(m, m.pos+2, op.param_modes[1]);
    let val = if operand1 < operand2 { 1 } else { 0 };

    m.mem[m.mem[m.pos+3] as usize] = val;
    v.push(m.mem[m.pos+3] as usize);
    m.pos += op.param_modes.len() + 1;
}

fn op_eq(m: &mut Machine, v: &mut Vec<usize>)
{
    let op = split_opcode(m.mem[m.pos]).unwrap();
    let operand1 = read_value(m, m.pos+1, op.param_modes[0]);
    let operand2 = read_value(m, m.pos+2, op.param_modes[1]);
    let val = if operand1 == operand2 { 1 } else { 0 };

    m.mem[m.mem[m.pos+3] as usize] = val;
    v.push(m.mem[m.pos+3] as usize);
    m.pos += op.param_modes.len() + 1;
}

fn op_halt(m: &mut Machine, _v: &mut Vec<usize>)
{
    m.halted = true;
}
