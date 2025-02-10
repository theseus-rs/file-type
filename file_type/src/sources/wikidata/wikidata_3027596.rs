use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3027596: FileType = FileType {
    file_format: &FileFormat {
        id: 3_027_596,
        source_type: SourceType::Wikidata,
        name: "DGN",
        extensions: &["dgn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
