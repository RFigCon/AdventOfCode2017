fn get_coor(val : i32) -> (i32, i32) {

    let mut coor : (i32, i32) = (0, 0);
    if val == 1 { return coor };

    let mut width : i32 = 1;
    let mut height : i32 = 1;
    let mut count : i32 = 1;
    let mut jump : i32;

    loop {

        //Right
        jump = width;
        if count + jump >= val {
            coor.0 += val - count;
            return coor;
        }
        coor.0 += jump;
        count += jump;
        width += 1;

        //Up
        jump = height;
        if count + jump >= val {
            coor.1 += val - count;
            return coor;
        }
        coor.1 += jump;
        count += jump;
        height += 1;

        //Left
        jump = width;
        if count + jump >= val {
            coor.0 -= val - count;
            return coor;
        }
        coor.0 -= jump;
        count += jump;
        width += 1;

        //Up
        jump = height;
        if count + jump >= val {
            coor.1 -= val - count;
            return coor;
        }
        coor.1 -= jump;
        count += jump;
        height += 1;

    }
}

pub fn run(){
    const INPUT : i32 = 312051;

    // Tests:
    // println!("Coor of 1: {:?}\n", get_coor(1));
    // println!("Coor of 2: {:?}\n", get_coor(2));
    // println!("Coor of 3: {:?}\n", get_coor(3));
    // println!("Coor of 4: {:?}\n", get_coor(4));
    // println!("Coor of 5: {:?}\n", get_coor(5));
    // println!("Coor of 7: {:?}\n", get_coor(7));
    // println!("Coor of 9: {:?}\n", get_coor(9));
    // println!("Coor of 10: {:?}\n", get_coor(10));
    // println!("Coor of 11: {:?}\n", get_coor(11));
    // println!("Coor of 21: {:?}\n", get_coor(21));
    // println!("Coor of 23: {:?}\n", get_coor(23));

    let coor = get_coor(INPUT);
    let x = if coor.0<0 {-1*coor.0} else {coor.0};
    let y = if coor.1<0 {-1*coor.1} else {coor.1};
    println!("\t-Coor of {INPUT} : {:?}; Distance from center is {}", coor, x+y);
}