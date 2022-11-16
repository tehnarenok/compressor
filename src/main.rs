#[cfg(feature = "full")]
fn main() {
    let args = compressor::args::gen_args();

    match args.subcommand() {
        Some(("compress", args)) => {
            let filename = args.value_of("filename").unwrap();
            let out = args.value_of("out").unwrap();

            compressor::file::compress(filename, out);
        }
        Some(("decompress", args)) => {
            let filename = args.value_of("filename").unwrap();
            let out = args.value_of("out").unwrap();

            compressor::file::decompress(filename, out);
        }
        _ => {
            println!("Please use subcommand \"compress\" or \"decompress\"; You can see help (-h)");
        }
    }
}

#[cfg(feature = "encoder")]
fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("You need enter input filename and out filename");
    }

    compressor::file::compress(&args[1], &args[2]);
}

#[cfg(feature = "decoder")]
fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("You need enter input filename and out filename");
    }

    compressor::file::decompress(&args[1], &args[2]);
}
