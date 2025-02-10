use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_877: FileType = FileType {
    file_format: &FileFormat {
        id: 877,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel for Macintosh",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
