fn calc_check_sum() -> (u32, u32) {
    let file_path = "resources/day2/input.txt";
    
    let contents : String;
    let contents_res : Result<String, std::io::Error> = std::fs::read_to_string(file_path);

    match contents_res {
        Ok(val) => contents = val,
        Err(_) => {
            println!("Could not find file \"{}\".", file_path);
            return (0, 0);
        },
    }

    let mut num : u32 = 0;
    let mut larg : u32 = 0;
    let mut small : u32 = !(0);
    let mut chs_p1 : u32 = 0;
    let mut chs_p2 : u32 = 0;
    let mut ended_on_nl : bool = false;

    let mut list : Vec<u32> = Vec::with_capacity(15);

    for c in contents.chars(){
        ended_on_nl = false;

        if c == '\r' {
            continue;
        }
        
        if c>='0' && c<='9' {
            num *= 10;
            num += (c as u32) - (b'0' as u32);
            continue;
        }
        
        list.push(num);
        
        if num > larg {
            larg = num;
        }
        
        if num < small {
            small = num;
        }

        num = 0;
        
        if c == '\n' {
            chs_p1 += larg - small;
            chs_p2 += get_even_div(&list);

            list.truncate(0);
            larg = 0;
            small = !(0);

            ended_on_nl = true;
        }
        
    }

    if !ended_on_nl {
        if num > larg {
            larg = num;
        }
        
        if num < small {
            small = num;
        }

        chs_p1 += larg - small;
        chs_p2 += get_even_div(&list);
        
    }
    
    return (chs_p1, chs_p2);
    
}

fn get_even_div(list : &Vec<u32>) -> u32 {
    
    for (i, &elem_a) in list[..].iter().enumerate() {
        for &elem_b in &list[i+1..] {

            if elem_a != 0 && elem_b%elem_a == 0 {
                return elem_b/elem_a; 
            }

            if elem_b != 0 && elem_a%elem_b == 0 {
                return elem_a/elem_b;
            }

        }
    }
    
    return 0;
}

pub fn run(){

    let (part1, part2) : (u32, u32) = calc_check_sum();

    println!("\t-Part 1: {}", part1);
    println!("\t-Part 2: {}", part2);

}