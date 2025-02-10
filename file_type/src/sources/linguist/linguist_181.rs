use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_181: FileType = FileType {
    file_format: &FileFormat {
        id: 181,
        source_type: SourceType::Linguist,
        name: "Java",
        extensions: &["jav", "java", "jsh"],
        media_types: &["text/x-java"],
        signatures: &[],
        related_formats: &[],
    },
};
