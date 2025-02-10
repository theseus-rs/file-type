use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2692: FileType = FileType {
    file_format: &FileFormat {
        id: 2_692,
        source_type: SourceType::Pronom,
        name: "WACZ",
        extensions: &["wacz"],
        media_types: &["application/x-wacz"],
        signatures: &[],
        related_formats: &[],
    },
};
