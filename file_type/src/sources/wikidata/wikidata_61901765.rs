use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61901765: FileType = FileType {
    file_format: &FileFormat {
        id: 61_901_765,
        source_type: SourceType::Wikidata,
        name: "EndNote Import File",
        extensions: &["enr", "enw"],
        media_types: &["application/x-endnote-refer"],
        signatures: &[],
        related_formats: &[],
    },
};
