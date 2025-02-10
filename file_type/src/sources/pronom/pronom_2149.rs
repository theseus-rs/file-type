use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2149: FileType = FileType {
    file_format: &FileFormat {
        id: 2_149,
        source_type: SourceType::Pronom,
        name: "Avery DesignPro Document",
        extensions: &["zdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
