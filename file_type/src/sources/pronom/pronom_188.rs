use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_188: FileType = FileType {
    file_format: &FileFormat {
        id: 188,
        source_type: SourceType::Pronom,
        name: "Microsoft Word for Macintosh Document",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
