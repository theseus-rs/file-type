use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_333915481: FileType = FileType {
    file_format: &FileFormat {
        id: 333_915_481,
        source_type: SourceType::Iana,
        name: "enriched",
        extensions: &[],
        media_types: &["text/enriched"],
        signatures: &[],
        related_formats: &[],
    },
};
