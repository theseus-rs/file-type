use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_271: FileType = FileType {
    file_format: &FileFormat {
        id: 271,
        source_type: SourceType::Pronom,
        name: "DataFlex Query Tag Name",
        extensions: &["tag"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
