use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use circular_buffer::CircularBuffer;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn get_sop_marker_index(s: &str) -> usize {
    let mut q = CircularBuffer::<4, char>::new();
    for (i, c) in s.chars().enumerate() {
        q.push_back(c);

        if q.is_full() && has_unique_elements(&q) {
            return i + 1;
        }
    }
    0
}

fn get_som_marker_index(s: &str) -> usize {
    let mut q = CircularBuffer::<14, char>::new();
    for (i, c) in s.chars().enumerate() {
        q.push_back(c);

        if q.is_full() && has_unique_elements(&q) {
            return i + 1;
        }
    }
    0
}

fn main() {
    {
        // Asserts
        assert!(get_sop_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 7);
        assert!(get_sop_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz") == 5);
        assert!(get_sop_marker_index("nppdvjthqldpwncqszvftbrmjlhg") == 6);
        assert!(get_sop_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 10);
        assert!(get_sop_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 11);
        assert!(get_som_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 19);
        assert!(get_som_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz") == 23);
        assert!(get_som_marker_index("nppdvjthqldpwncqszvftbrmjlhg") == 23);
        assert!(get_som_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 29);
        assert!(get_som_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 26);
    }
    {
        // Part 1
        for line in BufReader::new(File::open("input").unwrap()).lines() {
            let _line = line.unwrap();
        }

        let mut line = String::new();
        BufReader::new(File::open("input").unwrap())
            .read_line(&mut line)
            .unwrap();
        let ans = get_sop_marker_index(line.as_str());
        println!("Part 1: Ans is: {}", ans);
    }
    {
        // Part 2
        for line in BufReader::new(File::open("input").unwrap()).lines() {
            let _line = line.unwrap();
        }

        let mut line = String::new();
        BufReader::new(File::open("input").unwrap())
            .read_line(&mut line)
            .unwrap();
        let ans = get_som_marker_index(line.as_str());
        println!("Part 2: Ans is: {}", ans);
    }
}
