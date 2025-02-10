use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_33515158: FileType = FileType {
    file_format: &FileFormat {
        id: 33_515_158,
        source_type: SourceType::Wikidata,
        name: "LAS 1.1",
        extensions: &["las", "laz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
