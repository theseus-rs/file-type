use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_336: FileType = FileType {
    file_format: &FileFormat {
        id: 336,
        source_type: SourceType::Linguist,
        name: "STON",
        extensions: &["ston"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
