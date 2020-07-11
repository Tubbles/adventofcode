pub mod machine;
use machine::*;
// use std::io::Write;

const BLACK: usize = 0;
const WHITE: usize = 1;
const BLACK_CHAR: char = '.';
const WHITE_CHAR: char = '#';
const TURN_LEFT: usize = 0;
const TURN_RIGHT: usize = 1;
const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const LENGTH: usize = WIDTH * HEIGHT;
const WALK_HOR: usize = 1;
const WALK_VER: usize = WIDTH;

const fn xy_to_pos(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

const fn _pos_to_xy(pos: usize) -> (usize, usize) {
    (pos % WIDTH, pos / WIDTH)
}

const START: usize = xy_to_pos(60, 60);

#[derive(Copy, Clone, PartialEq, Debug)] // Debug printable
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Copy, Clone, PartialEq, Debug)] // Debug printable
struct Painter {
    pos: usize,
    dir: Direction,
}

fn painter_operate(
    painter: &mut Painter,
    canvas: &mut [char; LENGTH],
    color: usize,
    new_dir: usize,
) {
    canvas[painter.pos] = match color {
        BLACK => BLACK_CHAR,
        WHITE => WHITE_CHAR,
        _ => BLACK_CHAR,
    };
    painter.dir = match new_dir {
        TURN_LEFT => match painter.dir {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        },
        TURN_RIGHT => match painter.dir {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        },
        _ => Direction::Up,
    };
    painter.pos = match painter.dir {
        Direction::Right => painter.pos + WALK_HOR,
        Direction::Up => painter.pos - WALK_VER,
        Direction::Left => painter.pos - WALK_HOR,
        Direction::Down => painter.pos + WALK_VER,
    }
}

fn print_canvas(canvas: [char; LENGTH]) {
    for i in 0..canvas.len() {
        if i % WIDTH == 0 {
            println!();
        }
        print!("{}", canvas[i]);
    }
    println!();
}

fn main() {
    run_asserts();
    let mut m: Machine = Machine::new();
    m.set_terminal(false);
    m.set_interactive(false);
    let ints = load_machine_from_file(&mut m, "input.txt");
    println!("Num ints read = {}", ints);

    let mut painter = Painter {
        pos: START,
        dir: Direction::Up,
    };
    let mut canvas = [' '; LENGTH];
    canvas[START] = '#';

    m.put_input(match canvas[painter.pos] {
        BLACK_CHAR => BLACK,
        WHITE_CHAR => WHITE,
        _ => BLACK,
    } as isize);

    loop {
        run_machine(&mut m);
        if m.is_halted() {
            break;
        }

        let color = m.get_output().unwrap();
        let new_dir = m.get_output().unwrap();
        painter_operate(&mut painter, &mut canvas, color as usize, new_dir as usize);

        m.put_input(match canvas[painter.pos] {
            BLACK_CHAR => BLACK,
            WHITE_CHAR => WHITE,
            _ => BLACK,
        } as isize);
        // print_canvas(canvas);
        // println!();
        // println!("Press enter to execute next command");
        // std::io::stdout().flush().ok();
        // let mut line = String::new();
        // let _input = std::io::stdin()
        //     .read_line(&mut line)
        //     .expect("Failed to read line");
    }

    print_canvas(canvas);
    println!(
        "num painted: {}/{}",
        canvas.iter().filter(|&c| *c == '.' || *c == '#').count(),
        LENGTH
    );
}

fn run_asserts() {
    assert!(0 == 0);
}
