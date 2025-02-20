use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206073: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_073,
        source_type: SourceType::Wikidata,
        name: "Fuckpaint PI4",
        extensions: &["PI4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
