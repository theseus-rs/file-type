use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
