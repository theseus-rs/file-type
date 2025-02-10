use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1897: FileType = FileType {
    file_format: &FileFormat {
        id: 1_897,
        source_type: SourceType::Pronom,
        name: "VBScript (VBS) File",
        extensions: &["vbs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
