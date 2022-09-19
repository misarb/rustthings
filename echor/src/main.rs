use clap::{App, Arg};
fn main() {
    let matches = App::new("echor")
        .version("1.0.0")
        .author("Boulbalah Lahcen <lahssane12@gmail.com>")
        .about("echo Command with Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("newline")
                .short("n")
                .help("don'tmake a newLine")
                .takes_value(false),
        )
        .get_matches();
    
        let text = matches.values_of_lossy("text").unwrap();
        let omit_newline = matches.is_present("newline");
    
        print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    
}
