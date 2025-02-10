use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114235968: FileType = FileType {
    file_format: &FileFormat {
        id: 114_235_968,
        source_type: SourceType::Wikidata,
        name: "SYBYL format",
        extensions: &["sml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
