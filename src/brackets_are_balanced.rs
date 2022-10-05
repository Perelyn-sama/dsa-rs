pub fn run(string: &str) -> bool {
    let mut stack:Vec<char> = Vec::new();
    let mut trash:Vec<char> = Vec::new();
    let string_chars = string.chars();

    for letter in string_chars {
        match letter {
            '[' => {
                stack.push(letter);
            },
            '{' => {
                stack.push(letter);
            },
            '(' => {
                stack.push(letter);
            },
            ']' => {
                if stack.len() != 0 && stack.get(stack.len() - 1).unwrap() == &'['{
                    stack.pop();
                } else {
                    return  false;
                }
            },
            '}' => {
                if stack.len() != 0 && stack.get(stack.len() - 1).unwrap() == &'{'{
                    stack.pop();
                }else {
                    return false;
                }
            },
            ')' => {
                if stack.len() != 0 && stack.get(stack.len() - 1).unwrap() == &'('{
                    stack.pop();
                } else {
                    return false;
                }
            },
            _ => trash.push(letter),
        }
    }
    stack.len() == 0
}

#[test]
#[ignore]
fn paired_square_brackets() {
    assert!(run("[]"));
}
#[test]
#[ignore]
// #[ignore]
fn empty_string() {
    assert!(run(""));
}
#[test]
#[ignore]
// #[ignore]
fn unpaired_brackets() {
    assert!(!run("[["));
}
#[test]
#[ignore]
// #[ignore]
fn wrong_ordered_brackets() {
    assert!(!run("}{"));
}
#[test]
#[ignore]
// #[ignore]
fn wrong_closing_bracket() {
    assert!(!run("{]"));
}
#[test]
#[ignore]
// #[ignore]
fn paired_with_whitespace() {
    assert!(run("{ }"));
}
#[test]
#[ignore]
// #[ignore]
fn partially_paired_brackets() {
    assert!(!run("{[])"));
}
#[test]
#[ignore]
// #[ignore]
fn simple_nested_brackets() {
    assert!(run("{[]}"));
}
#[test]
#[ignore]
// #[ignore]
fn several_paired_brackets() {
    assert!(run("{}[]"));
}
#[test]
#[ignore]
// #[ignore]
fn paired_and_nested_brackets() {
    assert!(run("([{}({}[])])"));
}
#[test]
#[ignore]
// #[ignore]
fn unopened_closing_brackets() {
    assert!(!run("{[)][]}"));
}
#[test]
#[ignore]
// #[ignore]
fn unpaired_and_nested_brackets() {
    assert!(!run("([{])"));
}
#[test]
#[ignore]
// #[ignore]
fn paired_and_wrong_nested_brackets() {
    assert!(!run("[({]})"));
}
#[test]
#[ignore]
// #[ignore]
fn paired_and_incomplete_brackets() {
    assert!(!run("{}["));
}
#[test]
#[ignore]
// #[ignore]
fn too_many_closing_brackets() {
    assert!(!run("[]]"));
}
#[test]
#[ignore]
// #[ignore]
fn early_incomplete_brackets() {
    assert!(!run(")()"));
}
#[test]
#[ignore]
// #[ignore]
fn early_mismatched_brackets() {
    assert!(!run("{)()"));
}
#[test]
#[ignore]
// #[ignore]
fn math_expression() {
    assert!(run("(((185 + 223.85) * 15) - 543)/2"));
}
#[test]
#[ignore]
// #[ignore]
fn complex_latex_expression() {
    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
    assert!(run(input));
}
