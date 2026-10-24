#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageTag(String);

impl LanguageTag {
    pub fn parse(value: &str) -> Option<Self> {
        let value = value.trim();
        if value.is_empty()
            || value.len() > 35
            || !value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return None;
        }
        Some(Self(value.to_ascii_lowercase()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
