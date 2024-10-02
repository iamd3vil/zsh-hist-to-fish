use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;

const ZSH_HISTORY_FILE: &str = "~/.zsh_history";
const FISH_HISTORY_FILE: &str = "~/.local/share/fish/fish_history";
const ZSH_HISTORY_READER: &str = "zsh -i -c 'fc -R {}; fc -l -t \"%s\" 0'";

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(default_value = ZSH_HISTORY_FILE)]
    input_file: String,

    #[clap(short, long, default_value = FISH_HISTORY_FILE)]
    output_file: String,

    #[clap(short, long)]
    dry_run: bool,

    #[clap(short = 'n', long)]
    no_convert: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("ZSH history to Fish");
    println!("===================");
    println!(
        "{}: {} ({})",
        "input".dimmed(),
        args.input_file.blue(),
        format!("naive-convert={}", !args.no_convert).yellow()
    );
    println!(
        "{}: {}",
        "output".dimmed(),
        if args.dry_run {
            "dry run mode".blue()
        } else {
            args.output_file.blue()
        }
    );

    let input_file = shellexpand::tilde(&args.input_file);
    let output_file = shellexpand::tilde(&args.output_file);

    let history = read_history(&input_file)?;
    let mut changed = Vec::new();
    let mut processed = 0;

    let mut output = if args.dry_run {
        None
    } else {
        Some(File::create(output_file.as_ref()).context("Failed to create output file")?)
    };

    for (timestamp, command_zsh) in history {
        let command_fish = if args.no_convert {
            command_zsh.clone()
        } else {
            naive_zsh_to_fish(&command_zsh)
        };

        if let Some(ref mut file) = output {
            writeln!(file, "- cmd: {}", command_fish)?;
            writeln!(file, "  when: {}", timestamp)?;
        }

        if command_zsh != command_fish {
            changed.push((command_zsh, command_fish));
        }

        processed += 1;
        if processed % 1000 == 0 {
            print!(".");
            std::io::stdout().flush()?;
        }
    }

    println!("\nProcessed {} commands.", processed.to_string().blue());

    if !changed.is_empty() {
        println!("Converted commands:");
        for (zsh, fish) in changed {
            println!("{}zsh {}: {}", "  ".dimmed(), "".normal(), zsh);
            println!("{}fish{}: {}", "  ".dimmed(), "".normal(), fish.yellow());
        }
    }

    if args.dry_run {
        println!("No file has been written.");
    } else {
        println!("\nFile \"{}\" has been written successfully.", output_file);
    }

    Ok(())
}

fn read_history(input_file: &str) -> Result<Vec<(String, String)>> {
    let command = ZSH_HISTORY_READER.replace("{}", input_file);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .context("Failed to execute zsh history reader command")?;

    let reader = BufReader::new(&output.stdout[..]);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() >= 3 {
            let timestamp = parts[1].to_string();
            let command = parts[2].replace("\\n", "\n");
            result.push((timestamp, command));
        }
    }

    Ok(result)
}

fn naive_zsh_to_fish(cmd: &str) -> String {
    cmd.replace(" && ", "&&")
        .replace("&&", "; and ")
        .replace(" || ", "||")
        .replace("||", "; or ")
}
