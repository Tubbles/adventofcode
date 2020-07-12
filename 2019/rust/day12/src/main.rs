use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Body {
    name: String,
    pos: [isize; 3],
    vel: [isize; 3],
}

fn simulate(bodies: &mut Vec<Body>) {
    // Apply gravity
    for i in 0..bodies.len() {
        for k in 0..3 {
            for j in 0..bodies.len() {
                if i == j {
                    continue;
                }
                if bodies[i].pos[k] < bodies[j].pos[k] {
                    bodies[i].vel[k] += 1;
                    // println!(
                    //     "{}({}) < {}({})  (+1)",
                    //     bodies[i].name, bodies[i].pos[k], bodies[j].name, bodies[j].pos[k],
                    // )
                }
                if bodies[i].pos[k] > bodies[j].pos[k] {
                    bodies[i].vel[k] -= 1;
                    // println!(
                    //     "{}({}) > {}({})  (-1)",
                    //     bodies[i].name, bodies[i].pos[k], bodies[j].name, bodies[j].pos[k],
                    // )
                }
            }
        }
    }

    // Apply velocity
    for i in 0..bodies.len() {
        for k in 0..3 {
            bodies[i].pos[k] += bodies[i].vel[k];
        }
    }
}

fn total_energy(body: &Body) -> usize {
    let mut sum_abs_pos = 0;
    let mut sum_abs_vel = 0;
    for i in 0..3 {
        sum_abs_pos += body.pos[i].abs() as usize;
        sum_abs_vel += body.vel[i].abs() as usize;
    }
    sum_abs_pos * sum_abs_vel
}

fn main() {
    let re = Regex::new(r"=(-?\d+)").unwrap();
    let names = ["Io", "Europa", "Ganymede", "Callisto"];
    let mut moons = Vec::new();

    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    // Scan the lines of the file
    let mut i = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut caps = re.captures_iter(&line);

        moons.push(Body {
            name: names[i].to_string(),
            pos: [
                caps.next().unwrap()[1].parse().unwrap(),
                caps.next().unwrap()[1].parse().unwrap(),
                caps.next().unwrap()[1].parse().unwrap(),
            ],
            vel: [0; 3],
        });

        i += 1;
    }

    // Start the simulation
    let mut interactive = true;
    for moon in &moons {
        println!("{:?}, energy: {}", moon, total_energy(&moon));
    }
    for i in 0..1000 {
        if interactive {
            println!("Press enter to execute next time step");
            print!("or send RUN to exit interactive mode > ");
            std::io::stdout().flush().ok();
            let mut line = String::new();
            let _input = std::io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            if line == "RUN\n" {
                interactive = false;
            }
        }

        simulate(&mut moons);

        println!("Step {}:", i + 1);
        for moon in &moons {
            println!("{:?}, energy: {}", moon, total_energy(&moon));
        }
        println!(
            "total energy: {}",
            total_energy(&moons[0])
                + total_energy(&moons[1])
                + total_energy(&moons[2])
                + total_energy(&moons[3])
        );
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut moons = Vec::new();
        moons.push(Body {
            name: "Io".to_string(),
            pos: [-1, 0, 2],
            vel: [0, 0, 0],
        });
        moons.push(Body {
            name: "Europa".to_string(),
            pos: [2, -10, -7],
            vel: [0, 0, 0],
        });
        moons.push(Body {
            name: "Ganymede".to_string(),
            pos: [4, -8, 8],
            vel: [0, 0, 0],
        });
        moons.push(Body {
            name: "Callisto".to_string(),
            pos: [3, 5, -1],
            vel: [0, 0, 0],
        });

        for _ in 0..10 {
            simulate(&mut moons);
        }

        assert!(moons[0].pos[0] == 2 && moons[0].pos[1] == 1 && moons[0].pos[2] == -3);
        assert!(moons[1].pos[0] == 1 && moons[1].pos[1] == -8 && moons[1].pos[2] == 0);
        assert!(moons[2].pos[0] == 3 && moons[2].pos[1] == -6 && moons[2].pos[2] == 1);
        assert!(moons[3].pos[0] == 2 && moons[3].pos[1] == 0 && moons[3].pos[2] == 4);

        assert!(moons[0].vel[0] == -3 && moons[0].vel[1] == -2 && moons[0].vel[2] == 1);
        assert!(moons[1].vel[0] == -1 && moons[1].vel[1] == 1 && moons[1].vel[2] == 3);
        assert!(moons[2].vel[0] == 3 && moons[2].vel[1] == 2 && moons[2].vel[2] == -3);
        assert!(moons[3].vel[0] == 1 && moons[3].vel[1] == -1 && moons[3].vel[2] == -1);

        assert!(total_energy(&moons[0]) == 36);
        assert!(total_energy(&moons[1]) == 45);
        assert!(total_energy(&moons[2]) == 80);
        assert!(total_energy(&moons[3]) == 18);
    }

    #[test]
    fn test_2() {
        let mut moons = Vec::new();
        moons.push(Body {
            name: "Io".to_string(),
            pos: [-8, -10, 0],
            vel: [0, 0, 0],
        });
        moons.push(Body {
            name: "Europa".to_string(),
            pos: [5, 5, 10],
            vel: [0, 0, 0],
        });
        moons.push(Body {
            name: "Ganymede".to_string(),
            pos: [2, -7, 3],
            vel: [0, 0, 0],
        });
        moons.push(Body {
            name: "Callisto".to_string(),
            pos: [9, -8, -3],
            vel: [0, 0, 0],
        });

        for _ in 0..100 {
            simulate(&mut moons);
        }

        assert!(moons[0].pos[0] == 8 && moons[0].pos[1] == -12 && moons[0].pos[2] == -9);
        assert!(moons[1].pos[0] == 13 && moons[1].pos[1] == 16 && moons[1].pos[2] == -3);
        assert!(moons[2].pos[0] == -29 && moons[2].pos[1] == -11 && moons[2].pos[2] == -1);
        assert!(moons[3].pos[0] == 16 && moons[3].pos[1] == -13 && moons[3].pos[2] == 23);

        assert!(moons[0].vel[0] == -7 && moons[0].vel[1] == 3 && moons[0].vel[2] == 0);
        assert!(moons[1].vel[0] == 3 && moons[1].vel[1] == -11 && moons[1].vel[2] == -5);
        assert!(moons[2].vel[0] == -3 && moons[2].vel[1] == 7 && moons[2].vel[2] == 4);
        assert!(moons[3].vel[0] == 7 && moons[3].vel[1] == 1 && moons[3].vel[2] == 1);

        assert!(total_energy(&moons[0]) == 290);
        assert!(total_energy(&moons[1]) == 608);
        assert!(total_energy(&moons[2]) == 574);
        assert!(total_energy(&moons[3]) == 468);
    }
}
