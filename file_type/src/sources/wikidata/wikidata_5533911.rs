use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5533911: FileType = FileType {
    file_format: &FileFormat {
        id: 5_533_911,
        source_type: SourceType::Wikidata,
        name: "GeoPDF",
        extensions: &["pdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
