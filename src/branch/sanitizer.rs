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

pub fn remove_forbidden_chars(mut branch_name: String) -> String {
    for char in FORBIDDEN_CHARS {
        branch_name = branch_name.replace(char, "");
    }
    branch_name
}

pub fn replace_chars(mut branch_name: String) -> String {
    let replacement_chars = [
        (" ", "-"),
        ("---", "-"),
        ("--", "-"),
        ("&", "and"),
        (">", "gt"),
        ("/", "-"),
    ];

    for replacement in replacement_chars {
        branch_name = branch_name.replace(replacement.0, replacement.1);
    }
    branch_name
}