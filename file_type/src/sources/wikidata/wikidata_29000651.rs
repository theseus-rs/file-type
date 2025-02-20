use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000651: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_651,
        source_type: SourceType::Wikidata,
        name: "WLD",
        extensions: &["wld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
