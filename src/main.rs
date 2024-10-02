use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;

const ZSH_HISTORY_FILE: &str = "~/.zsh_history";
const FISH_HISTORY_FILE: &str = "~/.local/share/fish/fish_history";

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

    let input_file = shellexpand::tilde(&args.input_file).into_owned();
    let output_file = shellexpand::tilde(&args.output_file).into_owned();

    let history = parse_history(&input_file)?;
    let mut changed = Vec::new();
    let mut processed = 0;

    let mut output = if args.dry_run {
        None
    } else {
        Some(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(&output_file)
                .context("Failed to open output file")?,
        )
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
            io::stdout().flush()?;
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

fn read_history(input_file: &str) -> Result<Vec<String>> {
    let command = format!("fc -R {}; fc -l -t \"%s\" 0", input_file);

    let output = Command::new("zsh")
        .arg("-i")
        .arg("-c")
        .arg(&command)
        .output()
        .context("Failed to execute ZSH_HISTORY_READER command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "ZSH command failed with status: {}",
            output.status
        ));
    }

    let stdout = String::from_utf8(output.stdout)
        .context("Failed to decode stdout from ZSH_HISTORY_READER command")?;

    let lines: Vec<String> = stdout.lines().map(|s| s.replace("\\n", "\n")).collect();

    Ok(lines)
}

fn parse_history(input_file: &str) -> Result<Vec<(String, String)>> {
    let lines = read_history(input_file)?;
    let mut result = Vec::new();

    for line in lines {
        let mut parts = line.trim_start().split_whitespace();
        let _number = parts.next();
        let timestamp = parts.next();
        let command = parts.collect::<Vec<&str>>().join(" ");

        if let Some(timestamp) = timestamp {
            result.push((timestamp.to_string(), command));
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
