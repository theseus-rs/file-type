use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_883: FileType = FileType {
    file_format: &FileFormat {
        id: 883,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel for Macintosh",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
