use convert_case::{Case, Casing};
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

/// Generate a kebab-cased slug based on the first 30 characters of the name
/// and the first 8 bytes of the UUID.
pub fn generate_slug(name: &str, id: &Uuid) -> String {
    let normalized_name = name
        .chars()
        .nfkd()
        .filter(|&c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == ' ')
        .take(30)
        .collect::<String>()
        .to_case(Case::Kebab);

    let id_fields = id.as_fields();
    format!("{}-{:x}", normalized_name, id_fields.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        let name = "DitÏsÉên _ & % test";
        let id = Uuid::parse_str("b144dca4-b7e2-4651-97bc-7fa5125ab04e").unwrap();
        let expected = "dit-is-een-test-b144dca4";
        assert_eq!(generate_slug(name, &id), expected);
    }
}
