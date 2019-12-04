use std::fs::File;
use std::io::{BufRead, BufReader};

fn run_machine(noun: usize, verb: usize) -> usize
{
    const MEM_LENGTH: usize = 999;
    // let cl_red: &str = "\x1B[34m";
    // let cl_fg: &str = "\x1B[0m";

    struct Machine
    {
        pos: usize,
        mem: [usize; MEM_LENGTH],
        len: usize,
    }

    struct Commit
    {
        run: bool,
        pos: usize,
        val: usize,
    }
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut m = Machine{pos:0, mem:[0; MEM_LENGTH], len:0};

    // Scan the lines of the file
    for line in f.lines()
    {
        let line = line.expect("Unable to read line");
        let splits : Vec<&str> = line.split(",").collect();
        for split in splits
        {
            m.mem[m.len] = split.parse::<usize>().expect("Unable to parse split");
            m.len += 1;
            if m.len == MEM_LENGTH
            {
                println!("Not enough memory to store program");
                std::process::exit(-1);
            }
        }
    }

    // State reset
    m.mem[1] = noun;
    m.mem[2] = verb;

    // Run the program
    let mut opsize: usize = 0;
    // let mut commit = Commit{run:false, pos:0, val:0};
    let mut commit: Commit;
    let mut next_commit = Commit{run:false, pos:0, val:0};
    loop
    {
        let mut i: usize = 0;
        if m.pos == 0
        {
            // println!("Press enter to execute next instruction");
        }
        while i < m.len
        {
            if i == m.pos
            {
                // print!("{}", cl_red);
                // print!("--> ");
                // print!("{}", cl_fg);
            }
            else
            {
                // print!("    ");
            }
            let op = m.mem[i];
            match op
            {
                1 => { // Add
                    if i == m.pos {next_commit = Commit{run:true, pos:m.mem[i+3], val:m.mem[m.mem[i+1]]+m.mem[m.mem[i+2]]};}
                    // print!("{}{:>5}{} :  [{}{:>5}{}] + [{}{:>5}{}] -> [{}{:>5}{}]  ({}+{}={})",
                    //     if commit.run==true && i==commit.pos {cl_red} else {""},
                    //     op,
                    //     cl_fg,
                    //     if commit.run==true && i+1==commit.pos {cl_red} else {""},
                    //     if i+1<m.len {m.mem[i+1]} else {0},
                    //     cl_fg,
                    //     if commit.run==true && i+2==commit.pos {cl_red} else {""},
                    //     if i+2<m.len {m.mem[i+2]} else {0},
                    //     cl_fg,
                    //     if commit.run==true && i+3==commit.pos {cl_red} else {""},
                    //     if i+3<m.len {m.mem[i+3]} else {0},
                    //     cl_fg,
                    //     if i+1<m.len {m.mem[m.mem[i+1]]} else {0},
                    //     if i+2<m.len {m.mem[m.mem[i+2]]} else {0},
                    //     if i+2<m.len {m.mem[m.mem[i+1]]+m.mem[m.mem[i+2]]} else {0});
                    opsize = 4;
                    i += opsize;
                },
                2 => { // Multiply
                    if i == m.pos {next_commit = Commit{run:true, pos:m.mem[i+3], val:m.mem[m.mem[i+1]]*m.mem[m.mem[i+2]]};}
                    // print!("{}{:>5}{} :  [{}{:>5}{}] * [{}{:>5}{}] -> [{}{:>5}{}]  ({}*{}={})",
                    //     if commit.run==true && i==commit.pos {cl_red} else {""},
                    //     op,
                    //     cl_fg,
                    //     if commit.run==true && i+1==commit.pos {cl_red} else {""},
                    //     if i+1<m.len {m.mem[i+1]} else {0},
                    //     cl_fg,
                    //     if commit.run==true && i+2==commit.pos {cl_red} else {""},
                    //     if i+2<m.len {m.mem[i+2]} else {0},
                    //     cl_fg,
                    //     if commit.run==true && i+3==commit.pos {cl_red} else {""},
                    //     if i+3<m.len {m.mem[i+3]} else {0},
                    //     cl_fg,
                    //     if i+1<m.len {m.mem[m.mem[i+1]]} else {0},
                    //     if i+2<m.len {m.mem[m.mem[i+2]]} else {0},
                    //     if i+2<m.len {m.mem[m.mem[i+1]]*m.mem[m.mem[i+2]]} else {0});
                    opsize = 4;
                    i += opsize;
                },
                99 => { // Halt
                    if i == m.pos {next_commit = Commit{run:false, pos:0, val:0};}
                    // print!("{}{:>5}{} (HALT)", if i==commit.pos {cl_red} else {""}, op, cl_fg);
                    opsize = 1;
                    i += opsize;
                },
                _ => {
                    if i == m.pos {next_commit = Commit{run:false, pos:0, val:0};}
                    // print!("{}{:>5}{} (?)   ", if i==commit.pos {cl_red} else {""}, op, cl_fg);
                    opsize = 1;
                    i += opsize;
                },
            }
            // println!("");
        }
        // println!("Current pos = {}(/{})", m.pos, m.len);
        // let mut line = String::new();
        // let _input = std::io::stdin().read_line(&mut line).expect("Failed to read line");
        commit = next_commit;
        next_commit = Commit{run:false, pos:0, val:0};
        if commit.run == true
        {
            m.mem[commit.pos] = commit.val;
        }
        else
        {
            break;
        }
        m.pos += opsize;
        if m.pos >= m.len
        {
            break;
        }
        // print!("{}[2J", 27 as char); // Clear screen
    }
    // println!("");
    m.mem[0]
}

fn main() {
    // Part 1
    println!("Part 1: {}", run_machine(12, 02));
    for noun in 0..99
    {
        for verb in 0..99
        {
            if run_machine(noun, verb) == 19690720
            {
                println!("Part 2: {}", noun*100+verb);
                break;
            }
        }
    }
}
