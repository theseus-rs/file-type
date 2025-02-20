use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120731559: FileType = FileType {
    file_format: &FileFormat {
        id: 120_731_559,
        source_type: SourceType::Wikidata,
        name: "MotionMaker Template",
        extensions: &["fmt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
