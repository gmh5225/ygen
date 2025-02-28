use core::str;
use std::{fs::File, io::{Read, Write}};
use std::process::{exit, Command};

use ygen::Support::{Cli, Colorize};

mod parse;
use parse::*;

fn main() {
    let mut cli = Cli::new(
        "ytest", "The testing tool for ygen", "1.0", "Cr0a3"
    );
    
    cli.add_opt("h", "help", "Displays help");
    cli.add_opt("v", "version", "Displays the version");

    cli.add_opt("no-exit", "no-exit-on-error", "Ytest does not quite when an error occurs");
    cli.add_opt("neg-exit", "exit-code-neg", "Ytest exits automaticly even with `no-exit` if the programm returned with code -1");

    cli.add_arg("t", "test", "The file to the testcase", true);

    cli.scan();

    if cli.opt("h") {
        cli.help();
    } else if cli.opt("v") {
        cli.version();
    }

    let file = cli.arg_val("t").expect("We said it was required");
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(err) => {
            println!("{}: {}", "Error".red().bold(), err);
            exit(-1)
        },
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => {},
        Err(err) => {
            println!("{}: {}", "Error".red().bold(), err);
            exit(-1)
        },
    };

    let parsed = parse(buf);

    let path_str = "./tmp.yl";

    let path = std::path::PathBuf::from(path_str);

    if path.exists() {
        let _ = std::fs::remove_file(&path);
    }

    let mut file = match File::options().write(true).create(true).open(&path) {
        Ok(file) => file,
        Err(err) => {
            println!("{}: {}", "Error".red().bold(), err);
            exit(-1)
        },
    };

    match file.write_all(parsed.input.as_bytes()) {
        Ok(_) => {},
        Err(err) => {
            println!("{}: {}", "Error".red().bold(), err);
            exit(-1)
        },
    }

    let mut found = String::new();

    let mut code = 0;

    for cmd in parsed.cmd {
        let args = cmd.replace("%s", path_str);
        let args = unescaper::unescape(&args).unwrap();
        let args = args.trim();
        let mut args = args.split(" ").collect::<Vec<&str>>();

        let program = args.get(0).expect("expected valid excutable name").to_string();

        args.reverse();
        args.pop();
        args.reverse();


        println!("{}: executing following commandline: '{}{}'", "Info".blue().bold(), program, {
            let mut fmt = String::new();

            for arg in &args {
                fmt.push_str(&format!(" {}", arg));
            }

            fmt
        });

        let mut cmd = Command::new( program );
        
        for arg in args {
            if arg == "" {
                break;
            }

            if arg == " " {
                continue;
            }

            cmd.arg( arg );
        }

        let out = cmd.output().expect("failed to execute the process");
        let stdout = out.stdout;

        let stdout = str::from_utf8(&stdout).unwrap();
        let stdout = stdout.chars().filter(|x| !x.is_whitespace()).collect::<String>();

        found.push_str(&stdout);

        match cmd.status() {
            Ok(status) => {
                if !status.success() {
                    if let Some(exit_code) = status.code() {
                        if parsed.expected_code == 0 || (cli.opt("exit") && code == -1) {
                            println!("{}: the programm didn't exit sucessfull with code {}", "Error".red().bold(), exit_code);
                            if !cli.opt("no-exit") {
                                exit(-1);
                            }
                        } else if cli.opt("neg-exit") && code == -1 {
                            println!("{}: the programm didn't exit sucessfull with code {}", "Error".red().bold(), exit_code);
                            exit(-1);
                        } else {
                            code = exit_code;
                        }
                    } else {
                        println!("{}: the programm didn't exit sucessfull", "Error".red().bold());
                        if !cli.opt("no-exit") {
                            exit(-1)
                        }
                    }
                }
            },
            Err(err) => {
                println!("{}: {}", "Error".red().bold(), err);
                if !cli.opt("no-exit") {
                    exit(-1)
                }
            }
        };
    }

    let _ = std::fs::remove_file(&path);

    if parsed.expected_out != found {
        println!("{}: expected output didn't match actual output", "Error".red().bold());
        println!("found:    {:?}", found);
        println!("expected: {:?}", parsed.expected_out);
        if !cli.opt("no-exit") {
            exit(-1)
        }
    }

    if parsed.expected_code != code {
        println!("{}: expected exit code: {} found {}", "Error".red().bold(), parsed.expected_code, code);
        if !cli.opt("no-exit") {
            exit(-1)
        }
    } else {
        println!("expected exit code {} matched with found one {}", parsed.expected_code, code);
    }
}