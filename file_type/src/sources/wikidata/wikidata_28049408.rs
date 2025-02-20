use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28049408: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_408,
        source_type: SourceType::Wikidata,
        name: "DEGAS image, low resolution",
        extensions: &["PI1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
