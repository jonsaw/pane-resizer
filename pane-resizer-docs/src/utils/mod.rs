pub fn align_indentation(value: String) -> String {
    // Find the minimum indentation across all non-empty lines
    let indents: Vec<usize> = value
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .collect();

    // If we have lines with different indentation levels and the first line has 0 indent,
    // use the second-smallest indentation as the base
    let min_indent = if indents.len() > 1 && indents.contains(&0) {
        indents
            .iter()
            .filter(|&&i| i > 0)
            .min()
            .copied()
            .unwrap_or(0)
    } else {
        indents.iter().min().copied().unwrap_or(0)
    };

    // Remove the minimum indentation from each line
    let lines: Vec<&str> = value
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                ""
            } else if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                line
            }
        })
        .collect();

    // Preserve trailing newline if present
    let mut result = lines.join("\n");
    if value.ends_with('\n') {
        result.push('\n');
    }

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
