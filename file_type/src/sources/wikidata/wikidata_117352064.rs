use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117352064: FileType = FileType {
    file_format: &FileFormat {
        id: 117_352_064,
        source_type: SourceType::Wikidata,
        name: "Capture Design",
        extensions: &["dsn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
