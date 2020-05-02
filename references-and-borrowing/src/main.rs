fn main() {
    let s1 = String::from("hogefuga");
    let len = calc_len(&s1); // reference
    println!("The length of \"{}\" is {}", s1, len);
}

fn calc_len(s: &String) -> usize {
    s.len()
}
