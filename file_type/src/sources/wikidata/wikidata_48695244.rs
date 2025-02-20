use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48695244: FileType = FileType {
    file_format: &FileFormat {
        id: 48_695_244,
        source_type: SourceType::Wikidata,
        name: "DEC Data Exchange File",
        extensions: &["dx"],
        media_types: &["application/dec-dx"],
        signatures: &[],
        related_formats: &[],
    },
};
