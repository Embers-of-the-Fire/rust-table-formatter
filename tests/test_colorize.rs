#[test]
fn test_colorize() {
    use colored::Colorize;

    println!("{}", "format works as expected. This will be padded".blue());
    println!("{:.3}", "and this will be green but truncated to 3 chars".green());
    let b = "一二三".yellow();
    let bd = format!("{}", b);
    println!("{}", bd);
}
