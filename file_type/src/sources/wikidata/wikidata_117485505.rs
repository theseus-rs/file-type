use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117485505: FileType = FileType {
    file_format: &FileFormat {
        id: 117_485_505,
        source_type: SourceType::Wikidata,
        name: "MacCaption Project",
        extensions: &["cca"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
