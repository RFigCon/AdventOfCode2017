use read_file;

fn part1(input : &[u8], len : usize) -> u32 {

    if input.len() < 2 {
        return 0;
    }

    let first: u8 = input[0];
    let mut prev: u8 = first;
    let mut curr: u8;
    let mut res: u32 = 0;

    for index in 1..len {
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

fn part2(input : &[u8], len : usize) -> u32 {

    if input.len() < 2 {
        return 0;
    }

    let offset : usize = len/2;

    let mut curr: u8;
    let mut mirror: u8;
    let mut res: u32 = 0;

    for index in 0..offset {
        if input[index] == 0 {
            break;
        }
        curr = input[index];
        mirror = input[ (index+offset)%len ];
        if mirror == curr {
            res += ((curr - b'0')*2) as u32;
        }
    }

    return res;
}

pub fn run() {
    const MAX_ARR_SIZE : usize = 2048;
    let mut arr : [u8; MAX_ARR_SIZE] = [0; MAX_ARR_SIZE];

    let len = read_file::get_input(&mut arr, MAX_ARR_SIZE, "day1.txt");

    println!("The result is: {}", part1(&arr, len));
    println!("The result is: {}", part2(&arr, len));
}
