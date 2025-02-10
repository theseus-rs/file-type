use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58630708: FileType = FileType {
    file_format: &FileFormat {
        id: 58_630_708,
        source_type: SourceType::Wikidata,
        name: "Scalable Vector Graphics Tiny",
        extensions: &["svg"],
        media_types: &["image/svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
