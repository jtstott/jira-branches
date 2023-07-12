const FORBIDDEN_CHARS: [&str; 14] = [
    "..",
    "~",
    "^",
    ":",
    "[",
    "]",
    "?",
    "*",
    ".",
    "@{",
    "\\",
    "|",
    "(",
    ")"
];

pub fn remove_forbidden_chars(mut branch_template: String) -> String {
    for char in FORBIDDEN_CHARS {
        branch_template = branch_template.replace(char, "");
    }
    branch_template
}