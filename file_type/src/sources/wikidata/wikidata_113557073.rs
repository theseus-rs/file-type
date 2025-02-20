use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113557073: FileType = FileType {
    file_format: &FileFormat {
        id: 113_557_073,
        source_type: SourceType::Wikidata,
        name: "CloneCD Image",
        extensions: &["img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
