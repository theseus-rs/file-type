use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29905104: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_104,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System data mining database file",
        extensions: &["s7m", "sas7bdmd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
