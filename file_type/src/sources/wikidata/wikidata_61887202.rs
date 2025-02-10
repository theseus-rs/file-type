use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61887202: FileType = FileType {
    file_format: &FileFormat {
        id: 61_887_202,
        source_type: SourceType::Wikidata,
        name: "EndNote Style File",
        extensions: &["ens"],
        media_types: &["application/x-endnote-style"],
        signatures: &[],
        related_formats: &[],
    },
};
