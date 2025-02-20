use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_258: FileType = FileType {
    file_format: &FileFormat {
        id: 258,
        source_type: SourceType::Linguist,
        name: "Objective-C++",
        extensions: &["mm"],
        media_types: &["text/x-objectivec"],
        signatures: &[],
        related_formats: &[],
    },
};
