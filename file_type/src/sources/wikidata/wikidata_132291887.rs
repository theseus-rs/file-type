use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132291887: FileType = FileType {
    file_format: &FileFormat {
        id: 132_291_887,
        source_type: SourceType::Wikidata,
        name: "functional mock-up unit",
        extensions: &["fmu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
