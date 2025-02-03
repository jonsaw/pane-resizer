pub fn align_indentation(value: String) -> String {
    // Find the minimum indentation across all non-empty lines
    let min_indent = value
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    // Remove the minimum indentation from each line
    let result = value
        .lines()
        .map(|line| {
            if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                line
            }
        })
        .collect::<Vec<&str>>()
        .join("\n");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_indentation() {
        let input = r#"fn main() {
                println!("Hello, world!");
            }
        "#;
        let expected = r#"fn main() {
    println!("Hello, world!");
}
"#;
        assert_eq!(align_indentation(input.to_string()), expected);
    }
}
