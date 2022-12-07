/// Implements a solution for day 7 of the 2022 Advent of Code.

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use fsobject::*;
use putback_iter::*;

mod fsobject;
mod putback_iter;

fn main() -> Result<(), Box<dyn Error>> {
    println!("part_1: {:>10}", part_1()?);
    println!("part_2: {:>10}", part_2()?);
    Ok(())
}

/// Find the total size of all directories that have a size less than or equal 
/// to 100,000 and return the sum of their sizes.
/// 
fn part_1() -> Result<usize, Box<dyn Error>> {
    let root = build_fs()?;
    let dirs = root.find_dirs_recurs_by(&|d| d.size() <= 100000);
    let sum  = dirs.iter().map(|d| d.size()).sum();
    Ok(sum)
}

/// Find the size of the smallest directory needs to be deleted to accommodate
/// a 30MB update.
/// 
fn part_2() -> Result<usize, Box<dyn Error>> {
    let device_size = 70_000_000_usize;
    let update_size = 30_000_000_usize;

    let root  = build_fs()?;
    let taken = root.size();
    let avail = device_size - taken;
    let need  = update_size.saturating_sub(avail);

    let dirs  = root.find_dirs_recurs_by(&|d| d.size() >= need);
    let dmin  = dirs.iter().min_by_key(|d| d.size()).unwrap();

    Ok(dmin.size())
}

/// Build the file system from the data file.
/// 
fn build_fs() -> Result<FSDir, Box<dyn Error>> {
    let     file     = File::open("data/data.txt")?;
    let     reader   = BufReader::new(file);
    let     fs_root  = FSDir::new("/".into());
    let mut fs_stack = vec![fs_root.clone()];
    let mut cur_dir  = fs_root.clone();
    let mut lines    = PutBack::new(reader.lines());

    fn split(line: &str) -> Vec<&str> {
        line.split_whitespace().collect()
    }

    while let Some(line) = lines.next() {
        let line  = line?;
        let parts = split(&line);
        
        match parts[0] {
            "$" => {
                match parts[1] {
                    "cd" => {
                        let name = parts[2];
                        match name {
                            ".." => {
                                if cur_dir.name() != "/" {
                                    fs_stack.pop().unwrap();
                                    cur_dir = fs_stack.last().unwrap().clone();
                                }
                            },
                            "/" => {
                                fs_stack.truncate(1);
                                cur_dir = fs_root.clone();
                            },
                            _ => {
                                if let Some(d) = cur_dir.get_dir(name) {
                                    fs_stack.push(d.clone());
                                    cur_dir = d.clone();
                                }
                                else {
                                    let d = FSDir::new(name.into());
                                    cur_dir.add_dir(d.clone());
                                    fs_stack.push(d.clone());
                                    cur_dir = d;
                                }
                            },
                        }
                    },
                    "ls" => {
                        while let Some(line) = lines.next() {
                            let line  = line?;
                            let parts = split(&line);
                            match parts[0] {
                                "dir" => {
                                    let name = parts[1];
                                    if !cur_dir.contains(name) {
                                        let d = FSDir::new(name.into());
                                        cur_dir.add_dir(d);
                                    }
                                },
                                "$" => {
                                    lines.put_back(Ok(line));
                                    break;
                                },
                                size => {
                                    let size = size.parse::<usize>()?;
                                    let name = parts[1];
                                    if !cur_dir.contains(name) {
                                        let f = FSFile::new(name.into(), size);
                                        cur_dir.add_file(f);
                                    }
                                },
                            }
                        }
                    },
                    _ => panic!("Unknown command: {}", parts[1]),
                }
            },
            _ => panic!("Unknown command: {}", parts[0])
        }
    }
    Ok(fs_root)
}
