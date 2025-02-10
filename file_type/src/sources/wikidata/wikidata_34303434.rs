use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34303434: FileType = FileType {
    file_format: &FileFormat {
        id: 34_303_434,
        source_type: SourceType::Wikidata,
        name: "SYSDOOM script",
        extensions: &["doom"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
