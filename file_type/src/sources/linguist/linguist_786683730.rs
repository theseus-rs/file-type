use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_786683730: FileType = FileType {
    file_format: &FileFormat {
        id: 786_683_730,
        source_type: SourceType::Linguist,
        name: "HXML",
        extensions: &["hxml"],
        media_types: &["text/x-hxml"],
        signatures: &[],
        related_formats: &[],
    },
};
