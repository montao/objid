extern crate getopts;
extern crate rand;
use getopts::Options;
use rand::Rng;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    let brief = format!(
        "Version: {}",
        "$Id$"
            .replace("$Id: ", "")
            .replace(" $", "")
    );
    print!("{}", &brief);
}

fn r(x: i64) -> i64 {
    x & (std::i64::MAX - 1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version id");
    opts.optflag("c", "clean", "print nothing but the objid");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_f) => panic!("Error"),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("v") {
        print_version();
        return;
    }

    let b62digits = String::from("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut rng = rand::thread_rng();
    let mut rng63 = r(rng.gen::<i64>());
    let mut str = String::from("_");
    while rng63 != 0 {
        let d: i64 = rng63 % 62;
        rng63 = rng63 / 62;
        str.push(b62digits.chars().nth(d as usize).unwrap());
    }

    let mut text = format!("{}", "Random object id: ");

    if matches.opt_present("c") {
        text = format!("{}", "");
    }

    println!("{}{}", text, str);
}
