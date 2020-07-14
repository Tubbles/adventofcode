pub mod machine;
use machine::*;
use std::collections::VecDeque;
use std::io::Write;
use std::thread;
use std::time::Duration;

const fn xy_to_pos(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

const fn pos_to_xy(pos: usize) -> (usize, usize) {
    (pos % WIDTH, pos / WIDTH)
}

// Map settings
const VECDEQUE_DEPTH: usize = 10;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Input {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Output {
    Wall = 0,
    Floor = 1,
    Tank = 2,
}

// Canvas settings
const WIDTH: usize = 41;
const HEIGHT: usize = 41;
const LENGTH: usize = WIDTH * HEIGHT;

const DROID_CHAR: char = 'D';
const FRAME_CHAR: char = '#';

#[derive(Copy, Clone, PartialEq, Debug)]
enum SquareType {
    Unknown,
    Wall,
    Floor,
    Tank,
    Start,
    Dead,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Square {
    stype: SquareType,
    is_walkable: bool,
    is_quest: bool,
    display: char,
}

const SQUARES: [Square; 6] = [
    Square {
        stype: SquareType::Unknown,
        is_walkable: true,
        is_quest: false,
        display: '?',
    },
    Square {
        stype: SquareType::Wall,
        is_walkable: false,
        is_quest: false,
        display: ' ',
    },
    Square {
        stype: SquareType::Floor,
        is_walkable: true,
        is_quest: false,
        display: 'o',
    },
    Square {
        stype: SquareType::Tank,
        is_walkable: true,
        is_quest: true,
        display: 'T',
    },
    Square {
        stype: SquareType::Start,
        is_walkable: true,
        is_quest: true,
        display: 'S',
    },
    Square {
        stype: SquareType::Dead,
        is_walkable: false,
        is_quest: false,
        display: 'x',
    },
];

fn isize_to_output(i: isize) -> Option<Output> {
    match i {
        0 => Some(Output::Wall),
        1 => Some(Output::Floor),
        2 => Some(Output::Tank),
        _ => None,
    }
}

fn output_to_square(output: Output) -> Square {
    match output {
        Output::Wall => SQUARES[SquareType::Wall as usize],
        Output::Floor => SQUARES[SquareType::Floor as usize],
        Output::Tank => SQUARES[SquareType::Tank as usize],
    }
}

fn input_to_pos(input: Input) -> isize {
    match input {
        Input::North => -(WIDTH as isize),
        Input::South => WIDTH as isize,
        Input::West => -1,
        Input::East => 1,
    }
}

struct Droid {
    pos: usize,
    reversing: bool,
    ops: VecDeque<Input>,
    outs: VecDeque<Output>,
}
struct Map {
    canvas: Vec<Square>,
    interactive: bool,
    finished: bool,
}

fn print_map(map: &Map, droid: &Droid) {
    println!("{:?}:", *droid.ops.front().unwrap());
    for _ in 0..WIDTH + 1 {
        print!("{}", FRAME_CHAR);
    }
    for i in 0..LENGTH {
        if i % WIDTH == 0 {
            println!("{}", FRAME_CHAR);
            print!("{}", FRAME_CHAR);
        }
        print!(
            "{}",
            if i != droid.pos {
                map.canvas[i].display
            } else {
                DROID_CHAR
            }
        );
    }
    println!("{}", FRAME_CHAR);
    for _ in 0..WIDTH + 2 {
        print!("{}", FRAME_CHAR);
    }
    println!("");
}

fn parse_output(map: &mut Map, droid: &mut Droid) {
    let next_pos = get_next_pos_in_direction(&droid, *droid.ops.front().unwrap());

    // Check for reversing from dead end
    if droid.reversing == false && map.canvas[droid.pos].is_quest == false {
        // Three walls means dead end
        let mut adjacent_solids = 0;
        if map.canvas[get_next_pos_in_direction(droid, Input::North)].is_walkable == false {
            adjacent_solids += 1;
        }
        if map.canvas[get_next_pos_in_direction(droid, Input::South)].is_walkable == false {
            adjacent_solids += 1;
        }
        if map.canvas[get_next_pos_in_direction(droid, Input::West)].is_walkable == false {
            adjacent_solids += 1;
        }
        if map.canvas[get_next_pos_in_direction(droid, Input::East)].is_walkable == false {
            adjacent_solids += 1;
        }
        if adjacent_solids == 3 {
            droid.reversing = true;
        }
    }

    if droid.reversing == true {
        // Overwrite the square to mark a dead end
        map.canvas[droid.pos] = SQUARES[SquareType::Dead as usize];
    }

    // Check if we should update the map
    if map.canvas[next_pos].stype == SquareType::Unknown {
        match droid.outs.front().unwrap() {
            Output::Wall => map.canvas[next_pos] = SQUARES[SquareType::Wall as usize],
            Output::Floor => map.canvas[next_pos] = SQUARES[SquareType::Floor as usize],
            Output::Tank => map.canvas[next_pos] = SQUARES[SquareType::Tank as usize],
        }
    }

    if map.canvas[next_pos] == SQUARES[SquareType::Tank as usize] {
        map.interactive = true;
        println!(
            "Found the tank at {:?}, switching to interactive mode",
            pos_to_xy(next_pos)
        );
    }

    if map.canvas[next_pos].is_walkable {
        droid.pos = next_pos;
    }

    if droid.reversing == true {
        // Check if reversing should stop
        let mut adjacent_solids = 0;
        if map.canvas[get_next_pos_in_direction(droid, Input::North)].is_walkable == false {
            adjacent_solids += 1;
        }
        if map.canvas[get_next_pos_in_direction(droid, Input::South)].is_walkable == false {
            adjacent_solids += 1;
        }
        if map.canvas[get_next_pos_in_direction(droid, Input::West)].is_walkable == false {
            adjacent_solids += 1;
        }
        if map.canvas[get_next_pos_in_direction(droid, Input::East)].is_walkable == false {
            adjacent_solids += 1;
        }
        if adjacent_solids != 3 {
            droid.reversing = false;
        }
    }
}

fn get_reachable_unseen_squares(map: &Map) -> usize {
    let mut out = 0;
    for i in 0..map.canvas.len() {
        if map.canvas[i] != SQUARES[SquareType::Unknown as usize] {
            continue; // Only check checked squares
        }
        let (x, y) = pos_to_xy(i);

        // check for corners and edges
        if y != 0 && map.canvas[xy_to_pos(x, y - 1)] != SQUARES[SquareType::Wall as usize] {
            out += 1;
            continue;
        }
        if y != HEIGHT - 1 && map.canvas[xy_to_pos(x, y + 1)] != SQUARES[SquareType::Wall as usize]
        {
            out += 1;
            continue;
        }
        if x != 0 && map.canvas[xy_to_pos(x - 1, y)] != SQUARES[SquareType::Wall as usize] {
            out += 1;
            continue;
        }
        if x != WIDTH - 1 && map.canvas[xy_to_pos(x + 1, y)] != SQUARES[SquareType::Wall as usize] {
            out += 1;
            continue;
        }
    }
    out
}

fn get_next_pos_in_front(droid: &Droid) -> usize // returns pos in canvas space
{
    (droid.pos as isize + input_to_pos(*droid.ops.front().unwrap())) as usize
}

fn get_next_pos_in_direction(droid: &Droid, dir: Input) -> usize // returns pos in canvas space
{
    (droid.pos as isize + input_to_pos(dir)) as usize
}

fn vecdeque_insert_and_trim<T>(deque: &mut VecDeque<T>, insert: T, trimlen: usize) -> Option<T> {
    deque.push_front(insert);
    if deque.len() > trimlen {
        return deque.pop_back();
    }
    return None;
}

fn turn_left(droid: &mut Droid) {
    match droid.ops.pop_front().unwrap() {
        Input::North => {
            droid.ops.push_front(Input::West);
        }
        Input::South => {
            droid.ops.push_front(Input::East);
        }
        Input::West => {
            droid.ops.push_front(Input::South);
        }
        Input::East => {
            droid.ops.push_front(Input::North);
        }
    };
}

fn turn_right(droid: &mut Droid) {
    match droid.ops.pop_front().unwrap() {
        Input::North => {
            droid.ops.push_front(Input::East);
        }
        Input::South => {
            droid.ops.push_front(Input::West);
        }
        Input::West => {
            droid.ops.push_front(Input::North);
        }
        Input::East => {
            droid.ops.push_front(Input::South);
        }
    };
}

fn next_right_hand_rule(droid: &mut Droid) {
    if output_to_square(*droid.outs.front().unwrap()).is_walkable {
        turn_right(droid);
    } else {
        turn_left(droid);
    }
}

fn next_left_hand_rule(droid: &mut Droid) {
    if output_to_square(*droid.outs.front().unwrap()).is_walkable {
        turn_left(droid);
    } else {
        turn_right(droid);
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

    let mut map = Map {
        canvas: vec![SQUARES[SquareType::Unknown as usize]; LENGTH],
        interactive: true,
        finished: false,
    };

    let start_pos = xy_to_pos(WIDTH / 2 + 1, HEIGHT / 2 + 1);

    map.canvas[start_pos] = SQUARES[SquareType::Start as usize];

    let mut droid = Droid {
        pos: start_pos,
        reversing: false,
        ops: VecDeque::new(),
        outs: VecDeque::new(),
    };

    droid.ops.push_front(Input::North);

    let mut iter = 0;
    print_map(&map, &droid);
    while !m.is_halted() {
        // Take input
        // print!("NORTH = 1, SOUTH = 2, WEST = 3, EAST = 4 ?> ");
        // std::io::stdout().flush().ok();
        // let mut line = String::new();
        // let _input = std::io::stdin()
        //     .read_line(&mut line)
        //     .expect("Failed to read line");
        // let read = line.trim_end().parse().expect("Unable to parse line");
        // let read = match read {
        //     'h' => WEST,
        //     'j' => SOUTH,
        //     'k' => NORTH,
        //     'l' => EAST,
        //     _ => WEST,
        // };

        {
            let next_input = *droid.ops.front().unwrap();
            m.put_input(next_input as isize);
            // Duplicate the front to massage it into the next operation
            vecdeque_insert_and_trim(&mut droid.ops, next_input, VECDEQUE_DEPTH);
        }
        run_machine(&mut m);

        // Get output
        vecdeque_insert_and_trim(
            &mut droid.outs,
            isize_to_output(m.get_output().unwrap()).unwrap(),
            VECDEQUE_DEPTH,
        );
        parse_output(&mut map, &mut droid);

        let unseen = get_reachable_unseen_squares(&map);

        if !map.interactive {
            if iter % 1000 == 0 {
                println!("unseen: {}", unseen);
                print_map(&map, &droid);
            }
            thread::sleep(Duration::from_millis(1));
        } else {
            // Take input
            println!("unseen: {}", unseen);
            print_map(&map, &droid);
            print!("Press enter for next move, or type RUN to exit interactive mode ?> ");
            std::io::stdout().flush().ok();
            let mut line = String::new();
            let _input = std::io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let read: &str = line.trim_end();
            if read == "RUN" {
                map.interactive = false;
            }
        }

        // Start calculating the next step
        next_right_hand_rule(&mut droid);
        // Skip the already seen unwalkables
        while map.canvas[get_next_pos_in_front(&droid)].is_walkable == false {
            next_left_hand_rule(&mut droid); // inverted logic ¯\_( ͡° ͜ʖ ͡°)_/¯
        }

        iter += 1;
        if unseen == 0 && !map.finished {
            println!("map is completed, switching to interactive mode");
            println!(
                "distance from start to tank: {}",
                map.canvas
                    .iter()
                    .filter(|&a| *a == SQUARES[SquareType::Floor as usize])
                    .count()
                    + 1
            );
            map.interactive = true;
            map.finished = true;
        }
    }
    print_map(&map, &droid);
}
