use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61887390: FileType = FileType {
    file_format: &FileFormat {
        id: 61_887_390,
        source_type: SourceType::Wikidata,
        name: "EndNote Library format 1-9",
        extensions: &["enl"],
        media_types: &["application/x-endnote-library"],
        signatures: &[],
        related_formats: &[],
    },
};
