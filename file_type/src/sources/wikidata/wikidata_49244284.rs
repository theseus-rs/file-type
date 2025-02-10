use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49244284: FileType = FileType {
    file_format: &FileFormat {
        id: 49_244_284,
        source_type: SourceType::Wikidata,
        name: "form*Z Project File",
        extensions: &["fmz"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
