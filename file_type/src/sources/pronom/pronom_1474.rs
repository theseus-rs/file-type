use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1474: FileType = FileType {
    file_format: &FileFormat {
        id: 1_474,
        source_type: SourceType::Pronom,
        name: "Serif PagePlus Publication",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
