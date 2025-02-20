use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111262844: FileType = FileType {
    file_format: &FileFormat {
        id: 111_262_844,
        source_type: SourceType::Wikidata,
        name: "AKAI S5000/S6000 program",
        extensions: &["akai"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
