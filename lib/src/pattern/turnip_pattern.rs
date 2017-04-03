use pattern::Pattern;

use regex::{Captures, Regex};

struct TurnipPattern {
    pattern: String,
}

impl Pattern for TurnipPattern {
    fn to_regex(&self) -> Regex {
        let re = Regex::new(r":(\w+)").unwrap();

        let pattern = self.pattern.as_str();
        let regex = re.replace_all(pattern, |caps: &Captures| {
            format!("[\"']?(?P<{}>[^\"]+|[^']+|{})['\"]?",
                    &caps[1],
                    r"\-?[\w\.,]+")
        });

        Regex::new(regex.into_owned().as_str()).unwrap()
    }

    fn to_string(&self) -> String {
        self.pattern.clone()
    }
}

impl<'a> From<&'a str> for TurnipPattern {
    fn from(pattern: &'a str) -> Self {
        TurnipPattern { pattern: pattern.to_string() }
    }
}

#[cfg(test)]
mod test {
    use super::TurnipPattern;
    use pattern::Pattern;

    const SIMPLE_PATTERN: &str = "it's a beautiful day";
    const PATTERN_WITH_ARGUMENTS: &str = "I have a :home in :city";

    #[test]
    fn it_converts_a_simple_pattern_to_a_regex_matching_the_same_description() {
        let pattern = TurnipPattern::from(SIMPLE_PATTERN);

        let regex = pattern.to_regex();

        assert!(regex.is_match(SIMPLE_PATTERN));
    }

    #[test]
    fn it_converts_a_turnip_pattern_with_arguments_to_a_regex_matching_description_with_single_word_arguments() {
        let pattern = TurnipPattern::from(PATTERN_WITH_ARGUMENTS);

        let regex = pattern.to_regex();

        let description = "I have a house in Quebec";
        assert!(regex.is_match(description));
    }

    #[test]
    fn it_converts_a_turnip_pattern_with_arguments_to_a_regex_matching_description_with_single_quote_arguments() {
        let pattern = TurnipPattern::from(PATTERN_WITH_ARGUMENTS);

        let regex = pattern.to_regex();

        let description = "I have a house in 'Quebec City'";
        assert!(regex.is_match(description));
    }

    #[test]
    fn it_converts_a_turnip_pattern_with_arguments_to_a_regex_matching_description_with_double_quote_arguments() {
        let pattern = TurnipPattern::from(PATTERN_WITH_ARGUMENTS);

        let regex = pattern.to_regex();

        let description = "I have a \"beautiful appartment\" in Quebec";
        assert!(regex.is_match(description));
    }

    #[test]
    fn it_converts_a_turnip_pattern_with_arguments_to_a_regex_matching_description_with_unsigned_number_arguments() {
        let pattern = TurnipPattern::from(PATTERN_WITH_ARGUMENTS);

        let regex = pattern.to_regex();

        let description = "I have a home in -4";
        assert!(regex.is_match(description));
    }

    #[test]
    fn it_converts_a_turnip_pattern_with_arguments_to_a_regex_matching_description_with_signed_number_arguments() {
        let pattern = TurnipPattern::from(PATTERN_WITH_ARGUMENTS);

        let regex = pattern.to_regex();

        let description = "I have a home in 101";
        assert!(regex.is_match(description));
    }
}
