pub enum IssueType {
    Feature,
    Defect,
}

pub struct Issue {
    heading: String,
    r#type: IssueType,
}

impl Issue {
    pub fn new(heading: impl Into<String>, r#type: IssueType) -> Self {
        let heading: String = heading.into();
        Self { heading, r#type }
    }
}

pub struct ReleaseVersion(String);

impl From<String> for ReleaseVersion {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ReleaseVersion {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

pub fn normalize_to_path(issue: Issue, for_version: Option<ReleaseVersion>) -> String {
    let norm_heading = make_normalized_heading(&issue.heading);
    let type_prefix = make_type_prefix(&issue.r#type);
    let release_prefix = for_version
        .map(|r| make_version_prefix(&r))
        .unwrap_or_default();

    combine_parts(&release_prefix, &type_prefix, &norm_heading)
}

fn make_normalized_heading(heading: &str) -> String {
    todo!()
}

fn make_version_prefix(version: &ReleaseVersion) -> String {
    todo!()
}

fn make_type_prefix(r#type: &IssueType) -> String {
    todo!()
}

fn combine_parts(release: &str, r#type: &str, issue: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defect_type_correctly_maps() {
        let defect_type = IssueType::Defect;
        let norm_type = make_type_prefix(&defect_type);

        assert_eq!("defect".to_string(), norm_type);
    }

    #[test]
    fn feature_type_correctly_maps() {
        let defect_type = IssueType::Feature;
        let norm_type = make_type_prefix(&defect_type);

        assert_eq!("feature".to_string(), norm_type);
    }

    #[test]
    fn release_style_version_string_correctly_made_into_prefix() {
        let input_version: ReleaseVersion = "R6.4".into();
        let version_str = make_version_prefix(&input_version);

        assert_eq!("R6/R6.4".to_string(), version_str);
    }

    #[test]
    fn semver_style_just_numbers_version_string_correctly_made_into_prefix() {
        let input_version: ReleaseVersion = "2.4.1".into();
        let version_str = make_version_prefix(&input_version);

        assert_eq!("2/2.4/2.4.1".to_string(), version_str);
    }

    #[test]
    fn semver_style_v_prefix_version_string_correctly_made_into_prefix() {
        let input_version: ReleaseVersion = "v1.169.420".into();
        let version_str = make_version_prefix(&input_version);

        assert_eq!("v1/v1.169/v1.169.420".to_string(), version_str);
    }

    #[test]
    fn printable_characters_sanitized() {
        let input = "A:B?C[D\\E^F~G".to_string();
        let expected = "A_B_C_D_E_F_G".to_string();

        let actual = make_normalized_heading(input.as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn whitespaces_sanitized() {
        let input = "A B\tC\r\nD\rE\nF".to_string();
        let expected = "A_B_C_D_E_F".to_string();

        let actual = make_normalized_heading(input.as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple_consecutive_replaced_characters_replaced_with_single_character() {
        let input = "A    B:?[  \\^  ~  C \r\n D\r   E    \n  F".to_string();
        let expected = "A_B_C_D_E_F".to_string();

        let actual = make_normalized_heading(input.as_str());
        assert_eq!(expected, actual);
    }

    #[test]
    fn combine_parts_without_release_string_works() {
        let release = "";
        let r#type = "feature";
        let issue = "JIRA-1234_Some_issue_heading";

        let expected = format!("{type}/{issue}");
        let actual = combine_parts(release, r#type, issue);

        assert_eq!(expected, actual);
    }

    #[test]
    fn combine_parts_with_release_string_works() {
        let release = "R6/R6.4";
        let r#type = "feature";
        let issue = "JIRA-1234_Some_issue_heading";

        let expected = format!("{release}/{type}/{issue}");
        let actual = combine_parts(release, r#type, issue);

        assert_eq!(expected, actual);
    }

    #[test]
    fn combine_parts_works_with_redundant_slashes() {
        let release = "R6/R6.4";
        let r#type = "feature";
        let issue = "JIRA-1234_Some_issue_heading";

        let expected = format!("{release}/{type}/{issue}");

        let release = format!("/{release}/");
        let r#type = format!("{type}///");
        let issue = format!("//{issue}//");
        let actual = combine_parts(&release, &r#type, &issue);

        assert_eq!(expected, actual);
    }
}
