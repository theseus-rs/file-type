use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4043373: FileType = FileType {
    file_format: &FileFormat {
        id: 4_043_373,
        source_type: SourceType::Wikidata,
        name: "MAGMA",
        extensions: &["magma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
