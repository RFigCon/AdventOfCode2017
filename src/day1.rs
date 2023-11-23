use std::fs::File;
use std::io::Read;

pub fn get_input<'a>(arr: &'a mut [u8], file_name: &str) -> &'a mut [u8] {
    let file : File = File::open(file_name).unwrap();

    let mut idx : usize = 0;
    for byte in file.bytes(){
        if idx>=arr.len() {
            return &mut arr[..idx];
        }
        arr[idx] = byte.unwrap();
        idx += 1;
    }

    return &mut arr[..idx];
}

fn part1(input : &[u8]) -> u32 {

    if input.len() < 2 {
        return 0;
    }

    let first: u8 = input[0];
    let mut prev: u8 = first;
    let mut curr: u8;
    let mut res: u32 = 0;

    for index in 1..input.len() {
        if input[index] == 0 {
            break;
        }
        curr = input[index];
        if prev == curr {
            res += (prev - b'0') as u32;
        }
        prev = curr;
    }

    if first == prev {
        res += (first - b'0') as u32;
    }

    return res;
}

fn part2(input : &[u8]) -> u32 {

    if input.len() < 2 {
        return 0;
    }

    let offset : usize = input.len()/2;

    let mut curr: u8;
    let mut mirror: u8;
    let mut res: u32 = 0;

    for index in 0..offset {
        if input[index] == 0 {
            break;
        }
        curr = input[index];
        mirror = input[ (index+offset)%input.len() ];
        if mirror == curr {
            res += ((curr - b'0')*2) as u32;
        }
    }

    return res;
}

pub fn run() {
    const MAX_ARR_SIZE : usize = 2048;
    let mut arr : [u8; MAX_ARR_SIZE] = [0; MAX_ARR_SIZE];

    let slice = get_input(&mut arr[..], "resources/day1/input.txt");

    println!("\t-Part 1: {}", part1(&slice[..]));
    println!("\t-Part 2: {}", part2(&slice[..]));
}
