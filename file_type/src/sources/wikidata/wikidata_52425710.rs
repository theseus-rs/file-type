use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
