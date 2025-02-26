use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_327071: FileType = FileType {
    file_format: &FileFormat {
        id: 327_071,
        source_type: SourceType::Linguist,
        name: "ISPC",
        extensions: &["ispc"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
