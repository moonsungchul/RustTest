
use std::fs::File;
use std::io::{Write};
use rand::{Rng, thread_rng };

pub struct GenTestDat {
    pub fname : String,
    pub ssize : u64,
}

impl GenTestDat {

    pub fn new(name: String, size: u64) -> GenTestDat {
        GenTestDat {
            fname:name, 
            ssize:size, 
        }
    } 

    pub fn generator(&self) {
        let mut ff = File::create(self.fname.to_string()).expect("error file create");
        let mut co = 1;
        write!(&ff, "no,name,age,mail").expect("error write");
        loop {
            let _sno = co;
            let _sname = self.get_rand_string(co);
            let _age = self.get_rand_num(10, 100);
            let _mail = self.get_rand_string(co) + "@gmail.com";
            //println!("{},{},{},{}", sno, sname, age, mail);
            write!(ff, "{},{},{},{}\n", _sno, _sname, _age, _mail).expect("error file write");

            if co % 10000 == 0 {
                println!("generator : {}", co);
            }
            if co > self.ssize {
                break;
            }
            co += 1;
        }
        ff.flush().expect("error file flush");
    }

    pub fn get_rand_string(&self, no : u64) -> String {
        return format!("name{:08}", no);
    }

    fn get_rand_num(&self, start_no : u32, end_no : u32) -> u32 {
        let mut _rng = thread_rng();
        return _rng.gen_range(start_no, end_no);
    }

}





