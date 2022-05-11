use sudokusolver_rs;
use std::fs::File;
use std::path::Path;
use std::env;

const DEBUG: bool = false;



fn main() {

    let dfile: &str = "problems/testprob1.txt";
    // from the rust programming language: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = env::args().collect();

    // from rust by example: https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
    let mut path = Path::new(dfile);
    if args.len() >= 2 {
        path = Path::new(&args[1]);
    };
    let file = match File::open(&path) {
            Err(_e) => panic!("couldn't open file!"),
            Ok(f) => f,
    };

    let mut arr: [usize; 81] = [0; 81];
    let mut i: usize = 0;
    let mut csvread = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut noerr: bool = true;
    for res in csvread.records() {
        let record = res.unwrap();
        for x in record.iter() {
            if i < arr.len() {
                let y = x.parse::<usize>();
                match y {
                    Ok(z) => {arr[i] = z; i += 1;},
                    Err(_e) => {noerr = false; break;},
                };
            };
        };
        if noerr == false {
            break;
        };
    };

    if noerr == false {
        panic!("error reading file");
    } else {

        if DEBUG == true {
            for y in arr.chunks(9) {
                    println!("{:?}", y);
                };
        };
        let soln = sudokusolver_rs::solve(&arr);
        match soln {
            Some(x) => {
                for y in x.chunks(9) {
                    println!("{:?}", y);
                };
            },
            None => println!("no solution found"),
        };
    };
}
