use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_284: FileType = FileType {
    file_format: &FileFormat {
        id: 284,
        source_type: SourceType::Pronom,
        name: "Microsoft Word for Windows Macro",
        extensions: &["wpm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
