use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28452000: FileType = FileType {
    file_format: &FileFormat {
        id: 28_452_000,
        source_type: SourceType::Wikidata,
        name: "TERSE",
        extensions: &["trs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
