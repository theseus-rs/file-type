use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124987792: FileType = FileType {
    file_format: &FileFormat {
        id: 124_987_792,
        source_type: SourceType::Wikidata,
        name: "Dr.Geo document",
        extensions: &["fgeo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
