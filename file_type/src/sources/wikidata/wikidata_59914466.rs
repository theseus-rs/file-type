use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59914466: FileType = FileType {
    file_format: &FileFormat {
        id: 59_914_466,
        source_type: SourceType::Wikidata,
        name: "Deluxe Paint bitmap",
        extensions: &["lbm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
