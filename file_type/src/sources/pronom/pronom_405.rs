use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_405: FileType = FileType {
    file_format: &FileFormat {
        id: 405,
        source_type: SourceType::Pronom,
        name: "Microsoft Word for MS-DOS Document",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
