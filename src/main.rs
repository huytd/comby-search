use std::process::Command;
use colored::*;

fn main() {
    let mut args = std::env::args();
    let query = args.nth(1);
    let path = args.collect::<String>();
    if query.is_some() {
        if let Ok(output) = Command::new("comby")
            .args(["-match-only", &query.unwrap(), "''", "-rg", &format!("-g '{}'", path)])
            .output()
        {
            let stdout = std::str::from_utf8(&output.stdout).expect("Cannot parse stdout!");
            for output_line in stdout.lines() {
                let mut parts = output_line.split(':');
                let file_name = parts.next().unwrap();
                let at_line = parts.next().unwrap().parse::<usize>().unwrap();
                let line_count = parts.collect::<String>().split(r#"\n"#).count();
                let preview_from = at_line.saturating_sub(3).max(1);
                let preview_to = at_line + line_count + 3;
                let highlight_from = at_line;
                let highlight_to = at_line + line_count;
                let buf = Command::new("sed").args(["-n", &format!("{},{}p", preview_from, preview_to), file_name]).output().unwrap().stdout;
                let preview_lines = std::str::from_utf8(&buf).unwrap().lines();
                println!("{}", file_name.color("green"));
                for (i, line) in preview_lines.enumerate() {
                    let current_line = preview_from + i;
                    println!("{} {}", format!("{:>4} │", current_line).dimmed(), if current_line >= highlight_from && current_line < highlight_to {
                        line.color("red")
                    } else {
                        line.normal()
                    });
                }
            }
        }
    } else {
        println!("ERROR: Please enter your search query and file extensions!\nSyntax:\n    cb <query> [file extensions]\nFor example:\n    cb 'struct :[1] {{ :[2] }}' '*.rs'");
    }
}