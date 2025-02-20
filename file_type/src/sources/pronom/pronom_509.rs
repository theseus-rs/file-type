use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_509: FileType = FileType {
    file_format: &FileFormat {
        id: 509,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Document",
        extensions: &["bps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
