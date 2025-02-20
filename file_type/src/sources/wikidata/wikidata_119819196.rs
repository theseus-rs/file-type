use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119819196: FileType = FileType {
    file_format: &FileFormat {
        id: 119_819_196,
        source_type: SourceType::Wikidata,
        name: "Final Draft AV Script",
        extensions: &["av"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
