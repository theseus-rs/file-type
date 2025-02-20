use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_57: FileType = FileType {
    file_format: &FileFormat {
        id: 57,
        source_type: SourceType::Linguist,
        name: "ChucK",
        extensions: &["ck"],
        media_types: &["text/x-java"],
        signatures: &[],
        related_formats: &[],
    },
};
