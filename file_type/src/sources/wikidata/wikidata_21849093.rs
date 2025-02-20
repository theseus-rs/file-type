use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_21849093: FileType = FileType {
    file_format: &FileFormat {
        id: 21_849_093,
        source_type: SourceType::Wikidata,
        name: "DIMACS standard format",
        extensions: &["col", "col.b"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
