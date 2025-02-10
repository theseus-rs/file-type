use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52425710: FileType = FileType {
    file_format: &FileFormat {
        id: 52_425_710,
        source_type: SourceType::Wikidata,
        name: "VisiCalc Database",
        extensions: &["dif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
