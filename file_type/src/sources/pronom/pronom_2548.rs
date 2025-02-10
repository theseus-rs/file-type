use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2548: FileType = FileType {
    file_format: &FileFormat {
        id: 2_548,
        source_type: SourceType::Pronom,
        name: "Calc602 Macro File",
        extensions: &["mc6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
