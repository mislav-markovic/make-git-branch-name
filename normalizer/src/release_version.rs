use crate::GIT_JOIN_SEP;

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
    const DELIMITER: &'static str = ".";

    let version = version.version();

    if version.contains(DELIMITER) {
        let parts = version.split(DELIMITER);
        parts
            .map(|p| p.trim())
            .collect::<Vec<_>>()
            .join(&GIT_JOIN_SEP.to_string())
    } else {
        version.to_owned()
    }
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
