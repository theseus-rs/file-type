use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111341734: FileType = FileType {
    file_format: &FileFormat {
        id: 111_341_734,
        source_type: SourceType::Wikidata,
        name: "Sound Designer II data forks",
        extensions: &["sd2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
