use crate::{GIT_JOIN_SEP_CHAR, GIT_JOIN_SEP_STR, GIT_REPLACE_CHAR, GIT_REPLACE_STR};

enum HierarchyHandling {
    AllowMultipleLevels,
    ForceOneLevel,
}

fn trim_git_start(mut s: &str) -> &str {
    const MATCHES: [char; 4] = ['.', GIT_JOIN_SEP_CHAR, GIT_REPLACE_CHAR, '-'];

    let mut last_len = s.len() + 1; // + 1 so that we enter while loop and try to trim
    while last_len > s.len() {
        last_len = s.len();
        s = s.trim_start_matches(MATCHES).trim_start();
    }

    s
}

fn trim_git_end(mut s: &str) -> &str {
    const MATCHES: [char; 3] = [GIT_JOIN_SEP_CHAR, GIT_REPLACE_CHAR, '.'];

    let mut last_len = s.len() + 1; // + 1 so that we enter while loop and try to trim
    while last_len > s.len() {
        last_len = s.len();
        s = s
            .trim_end_matches(".lock")
            .trim_end_matches(MATCHES)
            .trim_end();
    }

    s
}

fn replace_unallowed_single_chars(s: &str) -> String {
    const MATCHES: [char; 8] = ['^','@', '~', ':', '?', '*', '[', '\\'];
    let mut s = s.to_string();
    s.retain(|c| !c.is_ascii_control());
    s = s.replace(char::is_whitespace, GIT_REPLACE_STR);
    s = s.replace(MATCHES, GIT_REPLACE_STR);

    s
}

fn replace_unallowed_multi_chars(s: &str) -> String {
    let matches = ["@{", ".."];
    let mut s = s.to_string();

    for m in matches {
        s = s.replace(m, GIT_REPLACE_STR);
    }

    s
}

fn collaps_replace_chars(s: &str) -> String {
    s.split(GIT_REPLACE_STR)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(GIT_REPLACE_STR)
}

fn sanitize_component(s: &str) -> String {
    // its important to first replace multi-chars values, since that is more expected
    // and some multi-chars values have overlap with single char values which will be replaced
    // so replacing single chars first can cause multi-chars to fail to replace
    // e.g. if name contains `@{` and we first replace single-chars `@` is lost and we leave `{` in
    // name which is undesirable
    let s = replace_unallowed_multi_chars(s);
    let s = replace_unallowed_single_chars(&s);

    let s = collaps_replace_chars(&s);

    let s = trim_git_start(&s);
    let s = trim_git_end(s);

    s.to_string()
}

fn normalize_git_name(name: &str, hierarchy: HierarchyHandling) -> String {
    let components = name
        .split(GIT_JOIN_SEP_CHAR)
        .filter(|part| !part.is_empty());

    let join_sep = match hierarchy {
        HierarchyHandling::ForceOneLevel => GIT_REPLACE_STR,
        HierarchyHandling::AllowMultipleLevels => GIT_JOIN_SEP_STR,
    };

    components
        .map(sanitize_component)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(join_sep)
}

pub fn normalize_git_name_multi_level(name: impl AsRef<str>) -> String {
    normalize_git_name(name.as_ref(), HierarchyHandling::AllowMultipleLevels)
}

pub fn normalize_git_name_to_one_level(name: impl AsRef<str>) -> String {
    normalize_git_name(name.as_ref(), HierarchyHandling::ForceOneLevel)
}

#[cfg(test)]
mod test {
    use super::*;

    fn ascii_controll_characters() -> impl Iterator<Item = char> {
        (0..=1024u32)
            .filter_map(|i| char::from_u32(i))
            .filter(char::is_ascii_control)
    }

    #[test]
    fn normalize_ignores_ascii_controll_characters() {
        let expected_onelevel = "JIRA-1234_SomeText";
        let expected_multi_level = "JIRA-1234_Some/Text";

        for c in ascii_controll_characters() {
            let issue_onelevel = format!("JIRA-1234/ Some{}Text", c);
            let actual_one_level =
                normalize_git_name(&issue_onelevel, HierarchyHandling::ForceOneLevel);

            let issue_multi_level = format!("JIRA-1234{}_Some/{}Text", c, c);
            let actual_multi_level =
                normalize_git_name(&issue_multi_level, HierarchyHandling::AllowMultipleLevels);

            assert_eq!(expected_onelevel, actual_one_level);
            assert_eq!(expected_multi_level, actual_multi_level);
        }
    }

    #[test]
    fn normalize_removes_ascii_controll_characters() {
        let expected_onelevel = "JIRA-1234_Some_issue_heading";
        let expected_multi_level = "JIRA-1234_Some/issue_heading";

        for c in ascii_controll_characters() {
            let issue_onelevel = format!("JIRA-1234/ Some {} issue heading", c);
            let actual_one_level =
                normalize_git_name(&issue_onelevel, HierarchyHandling::ForceOneLevel);

            let issue_multi_level = format!("JIRA-1234 {} Some/issue {} heading", c, c);
            let actual_multi_level =
                normalize_git_name(&issue_multi_level, HierarchyHandling::AllowMultipleLevels);

            assert_eq!(expected_onelevel, actual_one_level);
            assert_eq!(expected_multi_level, actual_multi_level);
        }
    }
}
