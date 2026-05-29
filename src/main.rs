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
    Ok(())
}
