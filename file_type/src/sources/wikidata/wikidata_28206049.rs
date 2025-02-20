use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206049: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_049,
        source_type: SourceType::Wikidata,
        name: "ERDAS Imagine IMG",
        extensions: &["img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
