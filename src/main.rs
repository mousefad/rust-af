use gumdrop::Options;
use anyhow::Result;

#[derive(Debug, gumdrop::Options)]
struct ProgramOptions {
    #[options(free, help="field specs and options input paths")]
    positional_params: Vec<String>,

    #[options(help="this cruft")]
    help: bool,

    #[options(short='F', help="field separator regex", default="\\s+")]
    input_delimiter: String,

    #[options(short='d', help="specify output field separator", default=" ")]
    output_delimiter: String,
}

fn main() -> Result<()> {
    let opts = ProgramOptions::parse_args_default_or_exit();
    eprintln!("{opts:?}");

    let mut fields: Vec<i8> = vec![];
    let mut input_paths: Vec<String> = vec![];

    for param in opts.positional_params {
        match param.parse::<i8>() {
            Ok(n) => {
                fields.push(n);
            },
            Err(_) => {
                input_paths.push(param);
            }
        }
    }

    eprintln!("fields: {:?}", fields);
    eprintln!("input_paths: {:?}", input_paths);
    Ok(())
}
