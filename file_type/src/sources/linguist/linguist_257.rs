use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_257: FileType = FileType {
    file_format: &FileFormat {
        id: 257,
        source_type: SourceType::Linguist,
        name: "Objective-C",
        extensions: &["h", "m"],
        media_types: &["text/x-objectivec"],
        signatures: &[],
        related_formats: &[],
    },
};
