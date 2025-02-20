use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
