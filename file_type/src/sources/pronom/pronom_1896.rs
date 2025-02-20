use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1896: FileType = FileType {
    file_format: &FileFormat {
        id: 1_896,
        source_type: SourceType::Pronom,
        name: "Visual Basic (VB) File",
        extensions: &["vb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
