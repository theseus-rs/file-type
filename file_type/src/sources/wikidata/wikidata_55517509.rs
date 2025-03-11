use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_55517509: FileType = FileType {
    file_format: &FileFormat {
        id: 55_517_509,
        source_type: SourceType::Wikidata,
        name: "Versatile Video Coding",
        extensions: &[],
        media_types: &["video/H266"],
        signatures: &[],
        related_formats: &[],
    },
};
