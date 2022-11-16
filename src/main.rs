#[cfg(feature = "full")]
fn main() {
    let args = compressor::args::gen_args();

    match args.subcommand() {
        Some(("encode", args)) => {
            let filename = args.value_of("filename").unwrap();
            let out = args.value_of("out").unwrap();
            let show_info = args.is_present("show_info");

            compressor::file::compress(filename, out, show_info);
        }
        Some(("decode", args)) => {
            let filename = args.value_of("filename").unwrap();
            let out = args.value_of("out").unwrap();

            compressor::file::decompress(filename, out);
        }
        Some(("info", args)) => {
            let filename = args.value_of("filename").unwrap();
            let print_tree = args.is_present("tree_codes");

            compressor::file::print_info(filename, print_tree);
        }
        Some(("entropy", args)) => {
            let filename = args.value_of("filename").unwrap();

            compressor::entropy::entropy(filename);
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

    compressor::file::compress(&args[1], &args[2], true);
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
