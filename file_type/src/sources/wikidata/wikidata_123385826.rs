use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123385826: FileType = FileType {
    file_format: &FileFormat {
        id: 123_385_826,
        source_type: SourceType::Wikidata,
        name: "Object Animation file",
        extensions: &["can"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
