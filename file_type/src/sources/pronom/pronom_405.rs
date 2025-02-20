use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
