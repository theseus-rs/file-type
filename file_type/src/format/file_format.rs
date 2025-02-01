use crate::format::source::Source;
use crate::format::{InternalSignature, RelatedFormat};

/// The types of file format
#[derive(Clone, Debug, Default)]
pub enum FormatTypes {
    Aggregate,
    Audio,
    Database,
    #[default]
    Dataset,
    Font,
    ImageRaster,
    ImageVector,
    Presentation,
}

/// A file format and its associated information
#[derive(Clone, Debug, Default)]
pub struct FileFormat {
    pub id: usize,
    pub puid: &'static str,
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
            "FileFormat {{ id: {}, puid: {:?}, name: {:?}, extensions: &[{}], media_types: &[{}], internal_signatures: &[{}], related_formats: &[{}] }}",
            self.id.to_source(),
            self.puid,
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
    fn test_to_source() {
        let file_format = FileFormat {
            id: 664,
            puid: "fmt/11",
            name: "Portable Network Graphics",
            extensions: &["png"],
            media_types: &["image/png"],
            internal_signatures: &[],
            related_formats: &[],
        };

        assert_eq!(
            file_format.to_source(),
            "FileFormat { id: 664, puid: \"fmt/11\", name: \"Portable Network Graphics\", extensions: &[\"png\"], media_types: &[\"image/png\"], internal_signatures: &[], related_formats: &[] }"
        );
    }
}
