use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};
use rand::{Rng, thread_rng};


fn print_type_of<T>(_: &T) {
    println!(">>>> data type :  {}", std::any::type_name::<T>())
}

fn write_fs(ff : &mut File) {
    let mut rng = thread_rng();
    for n in 1..100001 {
        let temp = rng.gen_range(10, 50);
        let aa = rng.gen_range(10, 100);
        let vv = rng.gen_range(100, 1000);
        write!(ff, "{},{},{},{}\n", n, temp, aa, vv).expect("error write");
    }
    ff.flush().expect("error file flush");
}  

fn main() -> Result<(), Error> {

    let args: Vec<String> = env::args().collect();

    if args.len() !=2 {
        println!("SensorData <outfile>");
        return Ok(());
    }

    let fname = &args[1];
    let mut ff = File::create(fname).expect("error file create");
    write!(ff, "no,temp,a,v\n").expect("error file write");
    write_fs(&mut ff);

    println!("create file : {}", fname);
    Ok(())
}
