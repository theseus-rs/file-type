use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
