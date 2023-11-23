use std::fs::File;
use std::io::Read;

pub fn get_input(arr: &mut [u8], MAX_ARR_SIZE: usize, file_name: &str) -> usize {
    let file : File = File::open(file_name).unwrap();

    let mut idx : usize = 0;
    for byte in file.bytes(){
        arr[idx] = byte.unwrap();
        idx += 1;
    }

    return idx;
}