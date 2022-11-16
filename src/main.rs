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
