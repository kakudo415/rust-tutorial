struct User {
    name: String,
}

fn main() {
    let user = User {
        name: String::from("hogehoge"),
    };
    println!("{}", user.name);
}
