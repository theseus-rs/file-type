use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_42: FileType = FileType {
    file_format: &FileFormat {
        id: 42,
        source_type: SourceType::Linguist,
        name: "C#",
        extensions: &["cake", "cs", "cs.pp", "csx", "linq"],
        media_types: &["text/x-csharp"],
        signatures: &[],
        related_formats: &[],
    },
};
