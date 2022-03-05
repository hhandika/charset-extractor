use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use rayon::prelude::*;

fn main() {
    let args = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .help("Nexus file to extract.")
                .multiple_values(true)
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let inputs: Vec<_> = args.values_of("input").unwrap().collect();
    inputs.par_iter().for_each(|i| {
        extract_partition(i);
    });

    println!("DONE!");
}

fn extract_partition<P: AsRef<Path>>(input: &P) {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    let output = construct_output_path(input);
    let mut writer = BufWriter::new(File::create(output).unwrap());
    let mut part = false;
    writeln!(writer, "#NEXUS").unwrap();
    reader.lines().filter_map(|ok| ok.ok()).for_each(|line| {
        if line.trim().to_lowercase().starts_with("begin sets") {
            part = true;
        }
        if line.trim().to_lowercase().starts_with("charpartition") {
            part = false;
        }
        if part {
            if !line.trim().is_empty() {
                writeln!(writer, "{}", line).unwrap();
            }
        }
    });

    writeln!(writer, "end;").unwrap();
    writer.flush().unwrap();
}

fn construct_output_path<P: AsRef<Path>>(input: &P) -> PathBuf {
    let fstem = input.as_ref().file_stem().unwrap().to_str().unwrap();
    let mut fname = PathBuf::from(format!("{}_partition", fstem));
    fname.set_extension("nex");
    let parent_path = input.as_ref().parent().unwrap();
    parent_path.join(fname)
}
