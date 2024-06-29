use crate::normalize::normalize_git_name_to_one_level;

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

    pub fn issue_type(&self) -> &str {
        &self.0
    }
}

pub(super) fn make_type_prefix(r#type: &IssueType) -> String {
    let normalized_type_name = normalize_git_name_to_one_level(r#type.issue_type());
    normalized_type_name.trim().to_lowercase()
}

#[cfg(test)]
mod test {
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
}
