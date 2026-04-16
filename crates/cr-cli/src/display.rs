// Display helpers for terminal output

/// Format a score as a colored progress bar
#[allow(dead_code)]
pub fn score_bar(score: u8, width: usize) -> String {
    let filled = (score as usize * width) / 100;
    let empty = width.saturating_sub(filled);
    format!(
        "[{}{}] {}/100",
        "#".repeat(filled),
        "-".repeat(empty),
        score
    )
}

/// Print a section header
#[allow(dead_code)]
pub fn print_header(title: &str) {
    println!("\n{}", title);
    println!("{}", "=".repeat(title.len()));
}
