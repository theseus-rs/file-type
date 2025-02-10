use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000745: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_745,
        source_type: SourceType::Wikidata,
        name: "MultiGen Flight",
        extensions: &["flt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
