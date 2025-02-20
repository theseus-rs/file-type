use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2342: FileType = FileType {
    file_format: &FileFormat {
        id: 2_342,
        source_type: SourceType::Pronom,
        name: "Serif PhotoPlus Image",
        extensions: &["spp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
