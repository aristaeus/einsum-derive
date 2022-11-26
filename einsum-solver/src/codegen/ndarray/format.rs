use std::{
    io::Write,
    process::{Command, Stdio},
};

/// Format generated Rust code using `rustfmt` run as external process.
pub fn format_block(tt: String) -> String {
    let tt = format!("fn main() {{ {} }}", tt);

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustfmt process");

    // Write input from another thread for avoiding deadlock.
    // See https://doc.rust-lang.org/std/process/index.html#handling-io
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(tt.as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child
        .wait_with_output()
        .expect("Failed to wait output of rustfmt process");

    // non-UTF8 comment should be handled in the tokenize phase,
    // and not be included in IR.
    let out = String::from_utf8(output.stdout).expect("rustfmt output contains non-UTF8 input");

    let formatted_lines: Vec<&str> = out
        .lines()
        .filter_map(|line| match line {
            "fn main() {" | "}" => None,
            _ => line.strip_prefix("    "),
        })
        .collect();
    formatted_lines.join("\n")
}
