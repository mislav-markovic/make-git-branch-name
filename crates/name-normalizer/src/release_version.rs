use crate::{normalize::normalize_git_name_multi_level, GIT_JOIN_SEP_CHAR};

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

    pub fn version(&self) -> &str {
        &self.0
    }
}

pub(super) fn make_version_prefix(version: &ReleaseVersion) -> String {
    const DELIMITER: &str = ".";

    let version = version.version();

    let rv = if version.contains(DELIMITER) {
        let parts = version
            .split(DELIMITER)
            .map(|p| p.trim())
            .collect::<Vec<_>>();

        let part_count = parts.len();
        let total_size: usize = parts
            .iter()
            .enumerate()
            .map(|(idx, &e)| (part_count - idx) * e.len())
            .sum();

        let mut str_builder = String::with_capacity(total_size);

        for i in 0..part_count {
            let intermediary = parts[..=i].join(DELIMITER);
            str_builder.push_str(&intermediary);
            str_builder.push(GIT_JOIN_SEP_CHAR);
        }

        str_builder
    } else {
        version.to_owned()
    };

    normalize_git_name_multi_level(rv)
}

#[cfg(test)]
mod test {
    use super::*;

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
}
