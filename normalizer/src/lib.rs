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
}

pub struct IssueType(String);

impl From<String> for IssueType {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for IssueType {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl IssueType {
    pub fn new(r#type: impl Into<String>) -> Self {
        let heading = r#type.into();
        Self(heading)
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

impl ReleaseVersion {
    pub fn new(version: impl Into<String>) -> Self {
        let heading = version.into();
        Self(heading)
    }
}
pub fn normalize_to_path(
    issue: &IssueHeading,
    for_type: Option<&IssueType>,
    for_version: Option<&ReleaseVersion>,
) -> String {
    let norm_heading = make_normalized_heading(issue);
    let type_prefix = for_type.map(make_type_prefix).unwrap_or_default();
    let release_prefix = for_version.map(make_version_prefix).unwrap_or_default();

    combine_parts(&release_prefix, &type_prefix, &norm_heading)
}

fn make_normalized_heading(heading: &IssueHeading) -> String {
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
        let defect_type = IssueType::new("defect");
        let norm_type = make_type_prefix(&defect_type);

        assert_eq!("defect".to_string(), norm_type);
    }

    #[test]
    fn feature_type_correctly_maps() {
        let defect_type = IssueType::new("feature");
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
