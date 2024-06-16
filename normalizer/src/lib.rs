use issue_heading::{make_normalized_heading, IssueHeading};
use issue_type::{make_type_prefix, IssueType};
use release_version::{make_version_prefix, ReleaseVersion};

pub mod issue_heading;
pub mod issue_type;
mod normalize;
pub mod release_version;

const GIT_JOIN_SEP_CHAR: char = '/';
const GIT_JOIN_SEP_STR: &'static str = "/";
const GIT_REPLACE_CHAR: char = '_';
const GIT_REPLACE_STR: &'static str = "_";

pub fn make_branch_name(
    issue: &IssueHeading,
    for_type: Option<&IssueType>,
    for_version: Option<&ReleaseVersion>,
) -> String {
    let norm_heading = make_normalized_heading(issue);
    let type_prefix = for_type.map(make_type_prefix).unwrap_or_default();
    let release_prefix = for_version.map(make_version_prefix).unwrap_or_default();

    combine_parts(&release_prefix, &type_prefix, &norm_heading)
}

fn combine_parts(release: &str, r#type: &str, issue: &str) -> String {
    fn trim_fn(c: char) -> bool {
        char::is_whitespace(c) || c == GIT_JOIN_SEP_CHAR || c == GIT_REPLACE_CHAR
    }

    let version = release.trim_matches(trim_fn);
    let r#type = r#type.trim_matches(trim_fn);
    let heading = issue.trim_matches(trim_fn);

    let total_size = version.len() + r#type.len() + heading.len() + 3;
    let mut str_builder = String::with_capacity(total_size);

    if !version.is_empty() {
        str_builder.push_str(version);
        str_builder.push(GIT_JOIN_SEP_CHAR)
    }

    if !r#type.is_empty() {
        str_builder.push_str(r#type);
        str_builder.push(GIT_JOIN_SEP_CHAR)
    }

    str_builder.push_str(heading);

    str_builder.shrink_to_fit();
    str_builder
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn make_branch_name_without_release_string_works() {
        let release: Option<ReleaseVersion> = None;
        let r#type: Option<IssueType> = Some("feature".into());
        let issue: IssueHeading = "JIRA-1234 Some issue heading".into();

        let expected = "feature/JIRA-1234_Some_issue_heading";
        let actual = make_branch_name(&issue, r#type.as_ref(), release.as_ref());

        assert_eq!(expected, actual);
    }

    #[test]
    fn make_branch_name_with_release_string_works() {
        let release: Option<ReleaseVersion> = Some("R6.4".into());
        let r#type: Option<IssueType> = Some("feature".into());
        let issue: IssueHeading = "JIRA-1234 Some issue heading".into();

        let expected = "R6/R6.4/feature/JIRA-1234_Some_issue_heading";
        let actual = make_branch_name(&issue, r#type.as_ref(), release.as_ref());

        assert_eq!(expected, actual);
    }

    #[test]
    fn make_branch_name_works_with_redundant_slashes() {
        let release: Option<ReleaseVersion> = Some("/R6.4/".into());
        let r#type: Option<IssueType> = Some("/feature/".into());
        let issue: IssueHeading = "/JIRA-1234 Some issue heading/".into();

        let expected = "R6/R6.4/feature/JIRA-1234_Some_issue_heading";

        let actual = make_branch_name(&issue, r#type.as_ref(), release.as_ref());

        assert_eq!(expected, actual);
    }
}
