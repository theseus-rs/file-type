use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131620885: FileType = FileType {
    file_format: &FileFormat {
        id: 131_620_885,
        source_type: SourceType::Wikidata,
        name: "MotionFX motion definition file",
        extensions: &["cfg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
