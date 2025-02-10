use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49243488: FileType = FileType {
    file_format: &FileFormat {
        id: 49_243_488,
        source_type: SourceType::Wikidata,
        name: "License file",
        extensions: &["lic"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
