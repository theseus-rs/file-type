use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118605987: FileType = FileType {
    file_format: &FileFormat {
        id: 118_605_987,
        source_type: SourceType::Wikidata,
        name: "Visual J# File",
        extensions: &["jsl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
