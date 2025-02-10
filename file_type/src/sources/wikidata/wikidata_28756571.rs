use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28756571: FileType = FileType {
    file_format: &FileFormat {
        id: 28_756_571,
        source_type: SourceType::Wikidata,
        name: "Forth script",
        extensions: &["fth"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
