use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3563777: FileType = FileType {
    file_format: &FileFormat {
        id: 3_563_777,
        source_type: SourceType::Wikidata,
        name: "MicroDVD",
        extensions: &["sub"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
