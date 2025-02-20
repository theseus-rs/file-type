use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_266: FileType = FileType {
    file_format: &FileFormat {
        id: 266,
        source_type: SourceType::Pronom,
        name: "Unisys (Sperry) System Data File",
        extensions: &["sdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
