use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_175: FileType = FileType {
    file_format: &FileFormat {
        id: 175,
        source_type: SourceType::Pronom,
        name: "Microsoft Works for Windows",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
