use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105857362: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_362,
        source_type: SourceType::Wikidata,
        name: "QMK keymap",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
