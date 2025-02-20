use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119819171: FileType = FileType {
    file_format: &FileFormat {
        id: 119_819_171,
        source_type: SourceType::Wikidata,
        name: "Final Draft AV Template",
        extensions: &["xavt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
