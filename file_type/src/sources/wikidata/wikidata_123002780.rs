use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123002780: FileType = FileType {
    file_format: &FileFormat {
        id: 123_002_780,
        source_type: SourceType::Wikidata,
        name: "Scalable Vector Graphics 1.1",
        extensions: &["svg"],
        media_types: &["image/svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
