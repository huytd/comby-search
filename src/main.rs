use clap::Parser;
use colored::*;
use std::process::Command;

#[derive(Parser, Debug)]
struct Cli {
    /// Your code search query
    query: String,

    /// File filter pattern, for example: '*.ts,!*.spec.ts'
    #[arg(short, long, value_name = "PATTERN")]
    filter: Option<String>,

    /// Number of lines to display before and after the matched search result
    #[arg(short, long, value_name = "MARGIN")]
    line_margin: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let path_filter = cli.filter.unwrap_or(String::new());
    let line_margin = cli.line_margin.unwrap_or(3);
    if let Ok(output) = Command::new("comby")
        .args([
            "-match-only",
            &cli.query,
            "''",
            "-rg",
            &format!("-g '{}'", path_filter),
        ])
        .output()
    {
        let stdout = std::str::from_utf8(&output.stdout).expect("Cannot parse stdout!");
        for output_line in stdout.lines() {
            let mut parts = output_line.split(':');
            let file_name = parts.next().unwrap();
            let at_line = parts.next().unwrap().parse::<usize>().unwrap();
            let line_count = parts
                .collect::<String>()
                .split(r#"\n"#)
                .count()
                .saturating_sub(1);
            let preview_from = at_line.saturating_sub(line_margin).max(1);
            let preview_to = at_line + line_count + line_margin;
            let highlight_from = at_line;
            let highlight_to = at_line + line_count;
            let buf = Command::new("sed")
                .args([
                    "-n",
                    &format!("{},{}p", preview_from, preview_to),
                    file_name,
                ])
                .output()
                .unwrap()
                .stdout;
            let preview_lines = std::str::from_utf8(&buf).unwrap().lines();
            println!("{}", file_name.color("green"));
            for (i, line_text) in preview_lines.enumerate() {
                let current_line = preview_from + i;
                let line_number = format!("{:>4}", current_line);
                if current_line >= highlight_from && current_line <= highlight_to {
                    println!(
                        "{} {} {}",
                        line_number.color("red"),
                        "│".dimmed(),
                        line_text.color("red")
                    );
                } else {
                    println!(
                        "{} {} {}",
                        line_number.dimmed(),
                        "│".dimmed(),
                        line_text.normal()
                    );
                }
            }
        }
    }
}
