use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1315657: FileType = FileType {
    file_format: &FileFormat {
        id: 1_315_657,
        source_type: SourceType::Wikidata,
        name: "Textile",
        extensions: &["textile"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
