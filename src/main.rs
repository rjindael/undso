fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    for arg in args.iter_mut().skip(1) {
        parse(arg.to_string());
    }
}

fn parse(filename: String) {
    let data = std::fs::read_to_string(filename).expect("Unable to read file");
    println!("{}", data);
}