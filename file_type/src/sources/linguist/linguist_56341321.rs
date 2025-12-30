use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_56341321: FileType = FileType {
    file_format: &FileFormat {
        id: 56_341_321,
        source_type: SourceType::Linguist,
        name: "ArkTS",
        extensions: &["ets"],
        media_types: &["application/typescript"],
        signatures: &[],
        related_formats: &[],
    },
};
