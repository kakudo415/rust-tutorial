fn main() {
    let s1 = String::from("hogefuga");
    let s2 = s1; // Move!
    // println!("{}", s1); ERROR!
    let s3 = s2.clone();
    println!("{}", s2);
}
