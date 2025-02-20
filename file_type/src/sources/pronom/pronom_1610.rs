use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1610: FileType = FileType {
    file_format: &FileFormat {
        id: 1_610,
        source_type: SourceType::Pronom,
        name: "StarOffice Draw",
        extensions: &["sdd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
