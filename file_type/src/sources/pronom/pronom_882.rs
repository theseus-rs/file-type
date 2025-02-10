use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_882: FileType = FileType {
    file_format: &FileFormat {
        id: 882,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel for Macintosh",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
