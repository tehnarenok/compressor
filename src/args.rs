use clap::{App, Arg, ArgMatches, Command};

pub fn gen_args() -> ArgMatches {
    let args = Command::new("Tehnarenok's compressor")
        .version("0.0.1")
        .author("Mikhail Ershov (tehnarenok)")
        .subcommand(
            App::new("encode")
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
                )
                .arg(
                    Arg::new("show_info")
                        .long("info")
                        .short('i')
                        .help("Show info about encoding")
                        .takes_value(false)
                        .required(false),
                ),
        )
        .subcommand(
            App::new("decode")
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
        .subcommand(
            App::new("info")
                .arg(
                    Arg::new("filename")
                        .long("filename")
                        .short('f')
                        .help("Input compressed filename")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("tree_codes")
                        .long("tree")
                        .short('t')
                        .help("Print tree codes")
                        .takes_value(false),
                ),
        )
        .subcommand(
            App::new("entropy").arg(
                Arg::new("filename")
                    .long("filename")
                    .short('f')
                    .help("Input compressed filename")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .get_matches();

    args
}
