use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29644119: FileType = FileType {
    file_format: &FileFormat {
        id: 29_644_119,
        source_type: SourceType::Wikidata,
        name: "ISO/IEC 8211 Data Descriptive File",
        extensions: &["ddf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
