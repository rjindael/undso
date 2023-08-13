mod decompiler;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    for arg in args.iter_mut().skip(1) {
        parse(arg.to_string());
    }
}

fn parse(filename: String) {
    let data: String = std::fs::read_to_string(&filename).expect("Unable to read file");
    let decompiled: String = decompiler::decompile(data);

    std::fs::write(format!("{}.decompiled", filename), decompiled).expect("Unable to write file");
}