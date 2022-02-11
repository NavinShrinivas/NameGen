use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::env;
use std::{fs::File, io::Read};

fn main() {
    //reading from adjective file
    let mut adjective_f = match File::open("./adjectives.txt") {
        Ok(fd) => fd,
        Err(e) => panic!("Something went wrong : {}", e),
    };
    let mut rawstr = String::new();
    adjective_f
        .read_to_string(&mut rawstr)
        .expect("Something went wrong when reading from file");
    let mut _re = &mut rawstr;
    let adjective_vec: Vec<&str> = rawstr.trim().split("\n").collect();
    //adjective vector consists of refrences to String rawstr, hence needs rawstr to be
    //not mutable throughout its life
    //_re.push_str("hello"); : not allowed

    //reading from noun file
    //reading from file
    let mut noun_f = match File::open("./nouns.txt") {
        Ok(fd) => fd,
        Err(e) => panic!("Something went wrong : {}", e),
    };
    let mut rawstr1 = String::new();
    noun_f
        .read_to_string(&mut rawstr1)
        .expect("Something went wrong when reading from file");
    let noun_vec: Vec<&str> = rawstr1.trim().split("\n").collect();
    //println!("{:?} , {:?}",adjective_vec,noun_vec);

    //processing command line args :
    let mut n: i32 = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        n = match args[1].trim().parse() {
            Ok(i) => i,
            _ => {
                println!("Stop giving garbage values ://");
                println!("Anyways : ");
                1
            }
        };
    }

    //randomly selectiong using
    for _i in 0..n {
        let selected_noun = match noun_vec.iter().choose(&mut thread_rng()) {
            Some(value) => value,
            _ => panic!("AHHH, Something went wrong"),
        };
        let selected_adjective = match adjective_vec.iter().choose(&mut thread_rng()) {
            Some(value) => value,
            _ => panic!("AHHH, Something went wrong"),
        };
        println!(
            "Project generator came up with : {}",
            format!("{}-{}", selected_adjective, selected_noun)
        );
    }
}
