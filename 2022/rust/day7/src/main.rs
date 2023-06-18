use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Debug)]
struct Directory {
    name: String,
    dirs: Vec<Directory>,
    files: Vec<File>,
    size: Option<usize>,
}

fn get_dir_from_path<'lt>(root_dir: &'lt mut Directory, path: &str) -> &'lt mut Directory {
    if path == "/" {
        return root_dir;
    }

    let stems: Vec<&str> = path.split("/").collect();
    let mut current_dir = &mut *root_dir;
    // println!("{:?}", path);
    // println!("{:?}", stems);

    for stem in stems {
        if stem == "" {
            continue;
        }
        let mut candidate = None;
        for dir in &mut current_dir.dirs {
            if dir.name == stem {
                candidate = Some(dir);
                break;
            }
        }
        assert!(candidate.is_some());
        current_dir = candidate.unwrap();
    }

    current_dir
}

fn mkdir(dir: &mut Directory, name: &str) {
    let subdir_exists = dir.dirs.iter_mut().find(|dir| dir.name == name).is_some();
    if !subdir_exists {
        let mut new_dir = Directory::default();
        new_dir.name = name.to_string();
        dir.dirs.push(new_dir);
    }
}

fn touch(dir: &mut Directory, name: &str, size: usize) {
    dir.files.push(File {
        name: String::from(name),
        size: size,
    });
}

fn build_fs(source_file_path: &str) -> Directory {
    let mut root_dir = Directory::default();
    root_dir.name = String::from("/");
    let mut current_dir_path = String::from("/");

    for line in BufReader::new(fs::File::open(source_file_path).unwrap()).lines() {
        let current_dir = get_dir_from_path(&mut root_dir, &current_dir_path);
        let line = line.unwrap();
        let words: Vec<&str> = line.split(" ").collect();
        if line.starts_with("$ ") {
            let args: Vec<&str> = line[2..line.len()].split(" ").collect();
            // println!("{:?}", args);
            match args[0] {
                "cd" => match args[1] {
                    "/" => {
                        current_dir_path = String::from("/");
                        // print_directory(&root_dir);
                    }

                    ".." => {
                        current_dir_path = current_dir_path.rsplit_once("/").unwrap().0.to_string();
                        current_dir_path = current_dir_path.rsplit_once("/").unwrap().0.to_string();
                        current_dir_path.push_str("/");
                    }

                    dir_name => {
                        mkdir(current_dir, dir_name);
                        current_dir_path.push_str(&format!("{}/", dir_name));
                        // print_directory(&root_dir);
                    }
                },

                "dir" => {
                    mkdir(current_dir, args[1]);
                    // print_directory(&root_dir);
                }

                _ => {}
            }
            //
        } else {
            match words[0] {
                "dir" => {
                    mkdir(current_dir, words[1]);
                }

                word => match word.parse::<usize>() {
                    Ok(size) => {
                        touch(current_dir, words[1], size);
                        // print_directory(&root_dir);
                    }

                    _ => assert!(false),
                },
            }
        }
    }

    root_dir
}

fn get_dir_size(dir: &Directory) -> usize {
    let mut sum = 0;

    for sub_dir in &dir.dirs {
        sum += get_dir_size(sub_dir);
    }

    for file in &dir.files {
        sum += file.size;
    }

    sum
}

fn get_dirs_with_sizes_below_threshold(dir: &Directory, threshold: usize) -> Vec<Directory> {
    let mut v = Vec::new();

    for subdir in &dir.dirs {
        v.append(&mut get_dirs_with_sizes_below_threshold(subdir, threshold));
    }

    if get_dir_size(dir) <= threshold {
        v.push(dir.clone());
    }

    v
}

fn get_dir_sizes_below_threshold(dir: &Directory, threshold: usize) -> usize {
    get_dirs_with_sizes_below_threshold(dir, threshold)
        .iter()
        .fold(0, |acc, x| acc + get_dir_size(x))
}

fn main() {
    {
        // Asserts
        let fs = build_fs("test");
        // print_directory(&fs);

        let ans = get_dir_sizes_below_threshold(&fs, 100000);
        assert_eq!(ans, 95437);
    }
    {
        // Part 1
        let fs = build_fs("input");
        // print_directory(&fs);

        let ans = get_dir_sizes_below_threshold(&fs, 100000);
        println!("Part 1: Ans is: {}", ans);
    }
    {
        // Part 2
        // for line in BufReader::new(fs::File::open("input").unwrap()).lines() {
        //     let _line = line.unwrap();
        // }

        let ans = 0;
        println!("Part 2: Ans is: {}", ans);
    }
}

// fn print_directory_with_indent(dir: &Directory, indent: usize, current_indent: usize) {
//     let indent_str = " ".repeat(current_indent);
//     println!(
//         "{}{} {} Bytes total",
//         indent_str,
//         dir.name,
//         get_dir_size(dir)
//     );

//     let current_indent = current_indent + 4;
//     let indent_str = " ".repeat(current_indent);

//     for dir in &dir.dirs {
//         print_directory_with_indent(dir, indent, current_indent);
//     }

//     for file in &dir.files {
//         println!("{}{} {} Bytes", indent_str, file.name, file.size);
//     }
// }

// fn print_directory(dir: &Directory) {
//     print_directory_with_indent(dir, 4, 0);
// }
