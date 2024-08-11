use std::{error::Error, fs::OpenOptions, process::exit};

use Ygen::{prelude::*, Support::Cli, Target::initializeAllTargets};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Cli::new("simplelang", "Simple language example for ygen", "1.0", "Cr0a3");
    
    cli.add_opt("ir", "emit-ir", "Emits the ir");
    cli.add_arg("in", "input", "The input file", true);
    cli.add_arg("o", "out", "The output file", false);
    cli.add_arg("triple", "triple", "The target triple", false);

    cli.scan();

    let infile = cli.arg_val("in").expect("we said it was required");
    let outfile;
    
    if let Some(out) = cli.arg_val("out") {
        outfile = out;
    } else {
        outfile = format!("{}.o", infile.split('.').collect::<Vec<&str>>()[0]);
    }

    let mut triple = Triple::host();

    if let Some(target) = cli.arg_val("triple") {
        triple = Triple::parse(&target)?;
    }

    let outfile = match OpenOptions::new().write(true).create(true).open(outfile) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e); 
            exit(-1);
        },
    };

    let mut module = Module();

    if cli.opt("ir") {
        println!("{}", module.dump());
    }

    module.emitMachineCode(
        triple, 
        &mut initializeAllTargets()
    )?.emit(outfile)?;

    Ok(())

}