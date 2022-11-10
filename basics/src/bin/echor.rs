fn main() {
    let omit_line_flag = clap::Arg::new("omitline")
        .short('n')
        .help("This is used for omitting the INPUT TEXT trailing line")
        .action(clap::ArgAction::SetTrue)
        .required(false);

    let text_arg = clap::Arg::new("text")
        .value_name("INPUT TEXT")
        .help("This is the input text that will and standard output")
        .action(clap::ArgAction::Append)
        .required(true);

    let command = clap::Command::new("echor")
        .version(env!("CARGO_PKG_VERSION"))
        .about("This is the same as the Linux echo command")
        .author("Najibullah Khoda Rahim")
        .arg(omit_line_flag)
        .arg(text_arg);

    let matches = command.get_matches();
    let omit_line = matches.get_flag("omitline");

    let text = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    print!("{text}{}", if omit_line { "" } else { "\n" });
}
