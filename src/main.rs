use gumdrop::Options;
use anyhow::{anyhow, Result};
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use regex::Regex;

#[derive(Debug, gumdrop::Options)]
struct ProgramOptions {
    #[options(free)]
    /// field specs and option input filenames.
    positional_params: Vec<String>,

    #[options(help="this cruft")]
    help: bool,

    #[options(short='F', help="field separator regex", default="\\s+")]
    input_delimiter: String,

    #[options(short='d', help="specify output field separator", default=" ")]
    output_delimiter: String,

    #[options(short='b', help="print blank output lines [default is not to]")]
    print_blanks: bool,
}

fn main() -> Result<()> {
    let opts = ProgramOptions::parse_args_default_or_exit();
    let split_rx =  Regex::new(&opts.input_delimiter)?;
    let mut field_selection: Vec<i8> = vec![];
    let mut input_paths: Vec<String> = vec![];

    for param in opts.positional_params {
        if param == "NF" {
            field_selection.push(-1);
        } else {
            match param.parse::<i8>() {
                Ok(n) => {
                    field_selection.push(n);
                },
                Err(_) => {
                    input_paths.push(param);
                }
            }
        }
    }

    if field_selection.len() == 0 {
        return Err(anyhow!("no indexes specified"));
    }

    for reader in get_readers(&input_paths)? {
        process_file(reader, &split_rx, &opts.output_delimiter, &field_selection, opts.print_blanks)?;
    }

    Ok(())
}

fn get_readers(input_paths: &[String]) -> Result<Vec<Box<dyn BufRead>>> {
    let readers: Vec<Box<dyn BufRead>> = match input_paths.len() {
        0 => {
            vec![Box::new(BufReader::new(io::stdin().lock())) as Box<dyn BufRead>]
        },
        _ => {
            input_paths
                .iter()
                .map(|path| -> Result<Box<dyn BufRead>> {
                    let file = File::open(path)?;
                    Ok(Box::new(BufReader::new(file)) as Box<dyn BufRead>)
                })
                .collect::<Result<Vec<Box<dyn BufRead>>>>()? 
        }
    };
    Ok(readers)
}

fn process_file<R: BufRead>(reader: R, split_rx: &Regex, output_delimiter: &str, field_selection: &Vec<i8>, print_blanks: bool) -> Result<()> {
    for line in reader.lines() {
        let content = line?;
        let split_fields: Vec<_> = split_rx.split(&content).collect();
        let output_line = field_selection
            .into_iter()
            .filter_map(|ndex| get_field_ndex(&split_fields, *ndex)) 
            .copied()
            .collect::<Vec<_>>()
            .join(output_delimiter);
        if output_line.len() > 0 || print_blanks {
            println!("{}", output_line);
        }
    }
    Ok(())
}

fn get_field_ndex<'a>(fields: &'a Vec<&'a str>, ndex: i8) -> Option<&'a &'a str> {
    if ndex == 0 || (ndex < 0 && -ndex as usize > fields.len()) {
        return None;
    }
    let idx = if ndex < 0 {

        fields.len() - (-ndex as usize)
    } else {
        ndex as usize - 1
    };
    fields.get(idx)
}
