use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_8062943: FileType = FileType {
    file_format: &FileFormat {
        id: 8_062_943,
        source_type: SourceType::Wikidata,
        name: "ZAP File",
        extensions: &["zap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
