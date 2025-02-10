use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3888665: FileType = FileType {
    file_format: &FileFormat {
        id: 3_888_665,
        source_type: SourceType::Wikidata,
        name: "Package interchange format",
        extensions: &["pif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
