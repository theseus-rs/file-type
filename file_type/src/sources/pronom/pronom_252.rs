use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_252: FileType = FileType {
    file_format: &FileFormat {
        id: 252,
        source_type: SourceType::Pronom,
        name: "Microsoft Visual Modeller Petal file (ASCII)",
        extensions: &["ptl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
