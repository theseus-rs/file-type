use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_377: FileType = FileType {
    file_format: &FileFormat {
        id: 377,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Drawing",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
