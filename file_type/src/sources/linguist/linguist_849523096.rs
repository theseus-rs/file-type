use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_849523096: FileType = FileType {
    file_format: &FileFormat {
        id: 849_523_096,
        source_type: SourceType::Linguist,
        name: "Microsoft Visual Studio Solution",
        extensions: &["sln"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
