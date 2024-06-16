pub struct IssueHeading(String);

impl From<String> for IssueHeading {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for IssueHeading {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl IssueHeading {
    pub fn new(heading: impl Into<String>) -> Self {
        let heading = heading.into();
        Self(heading)
    }

    pub fn from_parts(parts: &[&str]) -> Self {
        let heading = parts
            .iter()
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        Self::new(heading)
    }

    pub fn heading(&self) -> &str {
        &self.0
    }
}

pub(super) fn make_normalized_heading(heading: &IssueHeading) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn printable_characters_sanitized() {
        let input: IssueHeading = "A:B?C[D\\E^F~G".into();
        let expected = "A_B_C_D_E_F_G".to_string();

        let actual = make_normalized_heading(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn whitespaces_sanitized() {
        let input: IssueHeading = "A B\tC\r\nD\rE\nF".into();
        let expected = "A_B_C_D_E_F".to_string();

        let actual = make_normalized_heading(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple_consecutive_replaced_characters_replaced_with_single_character() {
        let input: IssueHeading = "A    B:?[  \\^  ~  C \r\n D\r   E    \n  F".into();
        let expected = "A_B_C_D_E_F".to_string();

        let actual = make_normalized_heading(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn git_check_ref_log_compatibility() {
        let input_expected_pairs: &[(IssueHeading, String)] = &[
            // easy happy path
            (
                "JIRA-1234 Some Information Here".into(),
                "JIRA-1234_Some_Information_Here".into(),
            ),
            // start and end trimmed correctly, and not replaced with safe char
            ("  JIRA-1234 Text   ".into(), "JIRA-1234_Text".into()),
            ("/JIRA-1234 Text/".into(), "JIRA-1234_Text".into()),
            ("/JIRA-1234 Text".into(), "JIRA-1234_Text".into()),
            ("JIRA-1234 Text/".into(), "JIRA-1234_Text".into()),
            (" /JIRA-1234 Text/ ".into(), "JIRA-1234_Text".into()),
            ("//JIRA-1234 Text//".into(), "JIRA-1234_Text".into()),
            ("/ JIRA-1234 Text /".into(), "JIRA-1234_Text".into()),
            // ===== single character replacement cases
            // middle slash also replaced
            ("JIRA-1234/Text".into(), "JIRA-1234_Text".into()),
            // multiple slashes replaced with one character
            ("JIRA-1234////Text".into(), "JIRA-1234_Text".into()),
            // single dots are allowed... except at the start
            (
                ".JIRA-1234 .. Some.Text ..".into(),
                "JIRA-1234_Some.Text".into(),
            ),
            // tilde replaced
            ("JIRA-1234 Some ~ Text".into(), "JIRA-1234_Some_Text".into()),
            // caret replaced
            ("JIRA-1234 Some ^ Text".into(), "JIRA-1234_Some_Text".into()),
            // colon replaced
            ("JIRA-1234 Some : Text".into(), "JIRA-1234_Some_Text".into()),
            // question mark replaced
            ("JIRA-1234 Some ? Text".into(), "JIRA-1234_Some_Text".into()),
            // asterisk replaced
            ("JIRA-1234 Some * Text".into(), "JIRA-1234_Some_Text".into()),
            // open square bracket replaced
            ("JIRA-1234 Some [ Text".into(), "JIRA-1234_Some_Text".into()),
            // `at` character replaced
            ("JIRA-1234 Some @ Text".into(), "JIRA-1234_Some_Text".into()),
            // backslash replaced
            (
                "JIRA-1234 Some \\ Text".into(),
                "JIRA-1234_Some_Text".into(),
            ),
            // ===== start specific rules
            // single dots are allowed
            (
                "JIRA-1234 .. Some.Text ..".into(),
                "JIRA-1234_Some.Text".into(),
            ),
            // can not start with dash
            ("-JIRA-1234 Some Text.".into(), "JIRA-1234_Some_Text".into()),
            // ending specific rules
            // can not end with sequence `.lock`
            (
                "JIRA-1234 Some  Text.lock".into(),
                "JIRA-1234_Some_Text".into(),
            ),
            // can not end with dot
            ("JIRA-1234 Some  Text.".into(), "JIRA-1234_Some_Text".into()),
            // ===== multi char rules
            // can not containt `@{`
            (
                "JIRA-1234 Some @{ Text".into(),
                "JIRA-1234_Some_Text".into(),
            ),
            // consecutive dots replaced
            (
                "JIRA-1234 Some .. Text".into(),
                "JIRA-1234_Some_Text".into(),
            ),
        ];

        for (input, expected) in input_expected_pairs {
            let actual = make_normalized_heading(input);

            assert_eq!(expected, &actual);
        }
    }
}
