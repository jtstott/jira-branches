use inquire::autocompletion::{Replacement};
use inquire::{Autocomplete, CustomUserError};

#[derive(Clone, Default)]
pub struct JiraTemplateCompleter {
    input: String,
    tokens: Vec<String>,
    lcp: String,
}

impl JiraTemplateCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        if input == self.input {
            return Ok(());
        }

        self.input = input.to_owned();
        self.tokens.clear();

        let mut entries: Vec<String> = vec![];

        if input.contains('[') {
            entries = vec!["[id]".into(), "[type]".into(), "[summary]".into()];
        }

        let mut idx = 0;
        let limit = 15;

        while idx < entries.len() && self.tokens.len() < limit {
            let entry = entries.get(idx).unwrap();

            let parts: Vec<&str> = self.input.split("[").collect();
            let mut current_input = String::from("");
            if let Some(ci) = parts.last() {
                current_input = format!("[{}", ci);
            }
            if entry.starts_with(current_input.as_str()) && entry.len() != current_input.len() {
                self.tokens.push(entry.into());
            }

            idx = idx.saturating_add(1);
        }

        self.lcp = self.longest_common_prefix();

        Ok(())
    }

    fn longest_common_prefix(&self) -> String {
        let mut ret: String = String::new();

        let mut sorted = self.tokens.clone();
        sorted.sort();
        if sorted.is_empty() {
            return ret;
        }

        let mut first_word = sorted.first().unwrap().chars();
        let mut last_word = sorted.last().unwrap().chars();

        loop {
            match (first_word.next(), last_word.next()) {
                (Some(c1), Some(c2)) if c1 == c2 => {
                    ret.push(c1);
                }
                _ => return ret,
            }
        }
    }
}

impl Autocomplete for JiraTemplateCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;

        Ok(self.tokens.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.update_input(input)?;
        Ok(match highlighted_suggestion {
            Some(suggestion) => {
                match self.input.to_owned().strip_suffix('[') {
                    None => Replacement::None,
                    Some(cur) => {
                        Replacement::Some(format!("{}{}", cur, suggestion))
                    }
                }
            }
            None => Replacement::None
        })
    }
}