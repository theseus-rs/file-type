use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_545626333: FileType = FileType {
    file_format: &FileFormat {
        id: 545_626_333,
        source_type: SourceType::Linguist,
        name: "Beef",
        extensions: &["bf"],
        media_types: &["text/x-csharp"],
        signatures: &[],
        related_formats: &[],
    },
};
