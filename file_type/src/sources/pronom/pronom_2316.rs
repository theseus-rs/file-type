use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2316: FileType = FileType {
    file_format: &FileFormat {
        id: 2_316,
        source_type: SourceType::Pronom,
        name: "NTI JewelCase Maker",
        extensions: &["jwc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
