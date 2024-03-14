fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s = wiredify::wiredify(s);
    println!("{}", s);
}
