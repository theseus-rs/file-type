use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135710782: FileType = FileType {
    file_format: &FileFormat {
        id: 135_710_782,
        source_type: SourceType::Wikidata,
        name: "Java Modeling Language file format",
        extensions: &["jml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
