use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_33515561: FileType = FileType {
    file_format: &FileFormat {
        id: 33_515_561,
        source_type: SourceType::Wikidata,
        name: "LAS 1.3 file format",
        extensions: &["las", "laz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
