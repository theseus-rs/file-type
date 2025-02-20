use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_259: FileType = FileType {
    file_format: &FileFormat {
        id: 259,
        source_type: SourceType::Linguist,
        name: "Objective-J",
        extensions: &["j", "sj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
