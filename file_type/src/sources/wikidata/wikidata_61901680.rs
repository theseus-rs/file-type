use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61901680: FileType = FileType {
    file_format: &FileFormat {
        id: 61_901_680,
        source_type: SourceType::Wikidata,
        name: "EndNote Connection File",
        extensions: &["enz"],
        media_types: &["application/x-endnote-connection"],
        signatures: &[],
        related_formats: &[],
    },
};
