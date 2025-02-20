use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118434883: FileType = FileType {
    file_format: &FileFormat {
        id: 118_434_883,
        source_type: SourceType::Wikidata,
        name: "Form File",
        extensions: &["fff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
