use getopts::Options;
use rand::{Rng, RngExt};
use std::{env, process::ExitCode};

const BASE62_DIGITS: &[u8; 62] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version");
    opts.optflag("c", "clean", "print nothing but the objid");
    opts
}

fn usage(program: &str, opts: &Options) -> String {
    let brief = format!("Usage: {program} [options]");
    opts.usage(&brief)
}

fn print_version() {
    println!("objid {}", env!("CARGO_PKG_VERSION"));
}

fn object_id<R>(rng: &mut R) -> String
where
    R: Rng + ?Sized,
{
    encode_base62(rng.random())
}

fn encode_base62(mut value: u64) -> String {
    let mut id = String::with_capacity(12);
    id.push('_');

    loop {
        let digit = (value % BASE62_DIGITS.len() as u64) as usize;
        id.push(BASE62_DIGITS[digit] as char);
        value /= BASE62_DIGITS.len() as u64;

        if value == 0 {
            return id;
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let program = args.first().map_or("objid", String::as_str);
    let opts = options();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(error) => {
            eprintln!("{error}");
            eprint!("{}", usage(program, &opts));
            return ExitCode::FAILURE;
        }
    };

    if matches.opt_present("h") {
        print!("{}", usage(program, &opts));
        return ExitCode::SUCCESS;
    }

    if matches.opt_present("v") {
        print_version();
        return ExitCode::SUCCESS;
    }

    let mut rng = rand::rng();
    let text = if matches.opt_present("c") {
        ""
    } else {
        "Random object id: "
    };

    println!("{text}{}", object_id(&mut rng));
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::encode_base62;

    #[test]
    fn encodes_zero_with_identifier_prefix() {
        assert_eq!(encode_base62(0), "_0");
    }

    #[test]
    fn encodes_base62_boundary_values() {
        assert_eq!(encode_base62(61), "_Z");
        assert_eq!(encode_base62(62), "_01");
    }

    #[test]
    fn encoded_ids_are_ascii_identifier_safe() {
        let id = encode_base62(u64::MAX);

        assert!(id.starts_with('_'));
        assert!(id.chars().skip(1).all(|ch| ch.is_ascii_alphanumeric()));
    }
}
