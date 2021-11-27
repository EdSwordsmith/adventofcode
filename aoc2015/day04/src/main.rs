use md5;

fn main() {
    let input = String::from("iwrupvqb");
    
    println!("Part 1: {}", find_number(input.clone(), "00000"));
    println!("Part 2: {}", find_number(input.clone(), "000000"));
}

fn find_number(input: String, zeros: &str) -> u32 {
    let mut num: u32 = 1;
    let zeros = String::from(zeros);
    let num_zeros = zeros.len();

    loop {
        let s = input.to_owned() + num.to_string().as_str();
        let hash = md5::compute(s.as_bytes());
        let result = format!("{:x}", hash);

        if zeros.eq(&result[..num_zeros]) {
            return num;
        }

        num = num + 1;
    }
}
