use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_79243141: FileType = FileType {
    file_format: &FileFormat {
        id: 79_243_141,
        source_type: SourceType::Wikidata,
        name: "Affinity Design document",
        extensions: &["afdesign"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
