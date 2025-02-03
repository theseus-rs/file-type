use crate::format::source::Source;
use crate::format::{InternalSignature, RelatedFormat};
use std::cmp::Ordering;

/// The source of the file format
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SourceType {
    Custom,
    #[default]
    Default,
    Httpd,
    Linguist,
    Pronom,
    Wikidata,
}

impl SourceType {
    /// Format the provided id with the source type in the format `{source_type}/{id}`
    /// (e.g. `default/1`)
    #[must_use]
    pub fn format_id(&self, id: usize) -> String {
        let source_type = format!("{self:?}").to_lowercase();
        format!("{source_type}/{id}")
    }

    /// Get the priority of the source type
    #[must_use]
    pub fn priority(&self) -> usize {
        match self {
            SourceType::Custom => 1,
            SourceType::Default => 0,
            SourceType::Httpd => 5,
            SourceType::Linguist => 4,
            SourceType::Pronom => 2,
            SourceType::Wikidata => 3,
        }
    }
}

impl PartialOrd for SourceType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority().cmp(&other.priority()))
    }
}

impl Ord for SourceType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

/// A file format and its associated information
#[derive(Clone, Debug, Default)]
pub struct FileFormat {
    pub id: usize,
    pub source_type: SourceType,
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub media_types: &'static [&'static str],
    pub internal_signatures: &'static [InternalSignature],
    pub related_formats: &'static [RelatedFormat],
}

impl FileFormat {
    /// Check if this file format is a match for the given bytes
    #[must_use]
    pub fn is_match(&self, bytes: &[u8]) -> bool {
        let matched = self
            .internal_signatures
            .iter()
            .any(|signature| signature.is_match(bytes));
        matched
    }
}

impl Source for FileFormat {
    fn to_source(&self) -> String {
        let extensions = self
            .extensions
            .iter()
            .map(|extension| format!("{extension:?}"))
            .collect::<Vec<String>>()
            .join(", ");
        let media_types = self
            .media_types
            .iter()
            .map(|media_type| format!("{media_type:?}"))
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "FileFormat {{ id: {}, source_type: SourceType::{:?}, name: {:?}, extensions: &[{}], media_types: &[{}], internal_signatures: &[{}], related_formats: &[{}] }}",
            self.id.to_source(),
            self.source_type,
            self.name,
            extensions,
            media_types,
            self.internal_signatures.iter().map(Source::to_source).collect::<Vec<String>>().join(", "),
            self.related_formats.iter().map(Source::to_source).collect::<Vec<String>>().join(", "),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::{ByteSequence, PositionType};

    #[test]
    fn test_format_id() {
        let source_type = SourceType::Custom;
        assert_eq!(source_type.format_id(1), "custom/1");
    }

    #[test]
    fn test_priority() {
        assert_eq!(SourceType::Custom.priority(), 1);
        assert_eq!(SourceType::Default.priority(), 0);
        assert_eq!(SourceType::Httpd.priority(), 5);
        assert_eq!(SourceType::Linguist.priority(), 4);
        assert_eq!(SourceType::Pronom.priority(), 2);
        assert_eq!(SourceType::Wikidata.priority(), 3);
    }

    #[test]
    fn test_partial_cmp() {
        assert_eq!(
            SourceType::Custom.partial_cmp(&SourceType::Custom),
            Some(Ordering::Equal)
        );
        assert_eq!(
            SourceType::Custom.partial_cmp(&SourceType::Default),
            Some(Ordering::Greater)
        );
        assert_eq!(
            SourceType::Default.partial_cmp(&SourceType::Custom),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn test_cmp() {
        assert_eq!(SourceType::Custom.cmp(&SourceType::Custom), Ordering::Equal);
        assert_eq!(
            SourceType::Custom.cmp(&SourceType::Default),
            Ordering::Greater
        );
        assert_eq!(SourceType::Default.cmp(&SourceType::Custom), Ordering::Less);
    }

    #[test]
    fn test_to_source() {
        let file_format = FileFormat {
            id: 664,
            source_type: SourceType::Default,
            name: "Portable Network Graphics",
            extensions: &["png"],
            media_types: &["image/png"],
            internal_signatures: &[],
            related_formats: &[],
        };

        assert_eq!(
            file_format.to_source(),
            "FileFormat { id: 664, source_type: SourceType::Default, name: \"Portable Network Graphics\", extensions: &[\"png\"], media_types: &[\"image/png\"], internal_signatures: &[], related_formats: &[] }"
        );
    }
}
