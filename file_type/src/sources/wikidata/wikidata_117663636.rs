use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117663636: FileType = FileType {
    file_format: &FileFormat {
        id: 117_663_636,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Quick Prints File",
        extensions: &["php"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
