use clap::{App, Arg, ArgMatches, Command};

pub fn gen_args() -> ArgMatches {
    let args = Command::new("Tehnarenok's compressor")
        .version("0.0.1")
        .author("Mikhail Ershov (tehnarenok)")
        .subcommand(
            App::new("compress")
                .arg(
                    Arg::new("filename")
                        .long("filename")
                        .short('f')
                        .help("Input filename")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("out")
                        .long("out")
                        .short('o')
                        .help("Output compressed filename")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("decompress")
                .arg(
                    Arg::new("filename")
                        .long("filename")
                        .short('f')
                        .help("Input compressed filename")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("out")
                        .long("out")
                        .short('o')
                        .help("Output decompressed filename")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    args
}
