fn main() {
    let mut wd: String = String::new();
    let mut s: String = String::new();
    println!("{}", "wiredify(w) or dewiredify(d)? : ");
    std::io::stdin().read_line(&mut wd).ok();
    println!("{}", "Input original sentence: ");
    std::io::stdin().read_line(&mut s).ok();
    if wd == "d" {
        s = wiredify::dewiredify(s);
    }
    else {
        s = wiredify::wiredify(s);
    }
    println!("{}", s);
}
