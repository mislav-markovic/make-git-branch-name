use config_cli::args::commands::norm::NormArgs;

use name_normalizer::{
    issue_heading::IssueHeading, issue_type::IssueType, make_branch_name,
    release_version::ReleaseVersion,
};

pub fn exec(args: &NormArgs) {
    let (heading, issue_type, version) = map_args(args);

    let result = make_branch_name(&heading, issue_type.as_ref(), version.as_ref());

    print!("{result}");
}

fn map_args(norm_args: &NormArgs) -> (IssueHeading, Option<IssueType>, Option<ReleaseVersion>) {
    let issue_type = norm_args.r#type.as_ref().and_then(|s| {
        if s.is_empty() {
            None
        } else {
            Some(IssueType::new(s))
        }
    });

    let version = norm_args.version.as_ref().and_then(|s| {
        if s.is_empty() {
            None
        } else {
            Some(ReleaseVersion::new(s))
        }
    });

    let strs = norm_args
        .heading
        .iter()
        .map(String::as_str)
        .collect::<Vec<&str>>();
    let heading = IssueHeading::from_parts(&strs);

    (heading, issue_type, version)
}
