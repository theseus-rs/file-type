use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119973449: FileType = FileType {
    file_format: &FileFormat {
        id: 119_973_449,
        source_type: SourceType::Wikidata,
        name: "WebEasy Web Document",
        extensions: &["web"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
