pub mod machine;
use machine::*;
use std::cmp::Ordering;
// use std::io::Write;
// use std::thread;
// use std::time::Duration;

const WIDTH: usize = 40;
const HEIGHT: usize = 21;
const LENGTH: usize = WIDTH * HEIGHT;

const EMPTY_DAT: isize = 0;
const WALL_DAT: isize = 1;
const BLOCK_DAT: isize = 2;
const PADDLE_DAT: isize = 3;
const BALL_DAT: isize = 4;
const EMPTY_CHAR: char = ' ';
const WALL_CHAR: char = '#';
const BLOCK_CHAR: char = '=';
const PADDLE_CHAR: char = '-';
const BALL_CHAR: char = 'o';

const fn xy_to_pos(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

const fn pos_to_xy(pos: usize) -> (usize, usize) {
    (pos % WIDTH, pos / WIDTH)
}

struct Board {
    canvas: [char; LENGTH],
    score: isize,
}

fn print_board(b: &Board) {
    for i in 0..LENGTH {
        if i % WIDTH == 0 {
            println!("");
        }
        print!("{}", b.canvas[i]);
    }
    println!("");
    println!("Score: {}", b.score);
    println!(
        "bricks left: {}",
        b.canvas.iter().filter(|&c| *c == BLOCK_CHAR).count()
    );
}

fn parse_output(b: &mut Board, (x, y, d): (isize, isize, isize)) {
    if x < 0 {
        b.score = d;
    } else {
        match d {
            EMPTY_DAT => b.canvas[xy_to_pos(x as usize, y as usize)] = EMPTY_CHAR,
            WALL_DAT => b.canvas[xy_to_pos(x as usize, y as usize)] = WALL_CHAR,
            BLOCK_DAT => b.canvas[xy_to_pos(x as usize, y as usize)] = BLOCK_CHAR,
            PADDLE_DAT => b.canvas[xy_to_pos(x as usize, y as usize)] = PADDLE_CHAR,
            BALL_DAT => b.canvas[xy_to_pos(x as usize, y as usize)] = BALL_CHAR,
            _ => b.canvas[xy_to_pos(x as usize, y as usize)] = EMPTY_CHAR,
        }
    }
}

fn main() {
    let mut m: Machine = Machine::new();
    println!(
        "Num ints read = {}",
        load_machine_from_file(&mut m, "input.txt")
    );
    m.set_terminal(false);
    m.set_interactive(false);
    m.mem[0] = 2; // Hack the machine so we can play the game for free
    let mut b = Board {
        canvas: [EMPTY_CHAR; LENGTH],
        score: 0,
    };
    // print_board(&b);
    // run_machine(&mut m);
    loop {
        run_machine(&mut m);

        // Parse the output
        while m.output_waiting() > 0 {
            parse_output(
                &mut b,
                (
                    m.get_output().unwrap(),
                    m.get_output().unwrap(),
                    m.get_output().unwrap(),
                ),
            );
        }
        if m.is_halted() {
            break;
        }
        // print_board(&b);

        // Get next input
        let (ball_x, _) = pos_to_xy(b.canvas.iter().position(|&c| c == BALL_CHAR).unwrap());
        let (paddle_x, _) = pos_to_xy(b.canvas.iter().position(|&c| c == PADDLE_CHAR).unwrap());
        let mov = match ball_x.partial_cmp(&paddle_x) {
            Some(Ordering::Less) => -1,
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => 1,
            _ => 0,
        };
        // println!("ball x: {}, paddle x: {}", ball_x, paddle_x);

        // print!("left=-1, right=1, neutral=0 ?> ");
        // std::io::stdout().flush().ok();
        // let mut line = String::new();
        // let _input = std::io::stdin()
        //     .read_line(&mut line)
        //     .expect("Failed to read line");
        // let read = line
        //     .trim_end()
        //     .parse::<isize>()
        //     .expect("Unable to parse line");
        m.put_input(mov);
        // thread::sleep(Duration::from_millis(1000))
    }
    print_board(&b);
}
