use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_351: FileType = FileType {
    file_format: &FileFormat {
        id: 351,
        source_type: SourceType::Linguist,
        name: "Smali",
        extensions: &["smali"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
