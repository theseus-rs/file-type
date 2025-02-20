use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
