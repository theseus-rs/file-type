use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123002751: FileType = FileType {
    file_format: &FileFormat {
        id: 123_002_751,
        source_type: SourceType::Wikidata,
        name: "Scalable Vector Graphics 1.0",
        extensions: &["svg"],
        media_types: &["image/svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
