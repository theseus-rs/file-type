use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000664: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_664,
        source_type: SourceType::Wikidata,
        name: "Processed Volume",
        extensions: &["pvl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
