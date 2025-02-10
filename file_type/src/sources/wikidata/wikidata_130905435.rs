use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130905435: FileType = FileType {
    file_format: &FileFormat {
        id: 130_905_435,
        source_type: SourceType::Wikidata,
        name: "Sieve file format",
        extensions: &["sieve", "siv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
