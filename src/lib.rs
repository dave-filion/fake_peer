
pub fn print_byte_array_len(header: &str, bytes: &[u8], until: usize) {
    print!("{} => [", header);
    for (i, b) in bytes.iter().enumerate() {
        if i == until {
            break;
        }

        if i == 0 {
            print!("0x{:X?}", b);
        } else {
            print!(", 0x{:X?}", b);
        }
    }
    println!("]");
}

pub fn print_byte_array(header: &str, bytes: &[u8]) {
    print!("{} => [", header);
    for (i, b) in bytes.iter().enumerate() {
        if i == 0 {
            print!("0x{:X?}", b);
        } else {
            print!(", 0x{:X?}", b);
        }
    }
    println!("]");
}


// reads bytes until end reached
pub fn get_bytes_til_end(buf: &[u8], start_i: usize) -> Vec<u8> {
    let mut v = Vec::new();

    for i in start_i.. buf.len()  {
        let byte = buf.get(i).expect("Array index error");
        v.push(byte.clone());
    }

    v
}


// reads n bytes from buf at index and returns vec, returns new index also
pub fn get_n_bytes_at(buf: &Vec<u8>, start_i: usize, n_bytes: usize) -> (Vec<u8>, usize) {
    let mut v = Vec::new();
    let mut j = start_i;
    for i in start_i..start_i + n_bytes {
        let byte = buf.get(i).unwrap();
        v.push(byte.clone());
        j = i;
    }
    (v, j)
}
