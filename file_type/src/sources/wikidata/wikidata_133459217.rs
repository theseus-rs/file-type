use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133459217: FileType = FileType {
    file_format: &FileFormat {
        id: 133_459_217,
        source_type: SourceType::Wikidata,
        name: ".fastresume",
        extensions: &["fastresume"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
