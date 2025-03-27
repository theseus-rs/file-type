use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28449155: FileType = FileType {
    file_format: &FileFormat {
        id: 28_449_155,
        source_type: SourceType::Wikidata,
        name: "JSON Patch",
        extensions: &["json-patch"],
        media_types: &["application/json-patch+json"],
        signatures: &[],
        related_formats: &[],
    },
};
