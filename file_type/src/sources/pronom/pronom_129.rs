use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_129: FileType = FileType {
    file_format: &FileFormat {
        id: 129,
        source_type: SourceType::Pronom,
        name: "Microsoft Powerpoint Design Template",
        extensions: &["pot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
