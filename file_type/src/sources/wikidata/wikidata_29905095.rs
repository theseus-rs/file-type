use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29905095: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_095,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System multi-dimensional database file",
        extensions: &["sas7bmdb", "sm2", "sm7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
