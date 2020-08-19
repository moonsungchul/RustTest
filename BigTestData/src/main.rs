use std::env;

mod generator;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("BittestData <outfile> <size>");
        return;
    }

    let fname = args[1].to_string();
    println!(">>>>> {}", fname);
    let size = args[2].parse::<u64>().unwrap();
    println!(">>>. size {}", size);
    //let rr = generator::GenTestDat{fname:fname, ssize:size};
    let rr = generator::GenTestDat::new(fname, size);
    rr.generator();
}
