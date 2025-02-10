use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131413865: FileType = FileType {
    file_format: &FileFormat {
        id: 131_413_865,
        source_type: SourceType::Wikidata,
        name: "Vyper file format",
        extensions: &["vy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
