use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_786683730: FileType = FileType {
    file_format: &FileFormat {
        id: 786_683_730,
        source_type: SourceType::Linguist,
        name: "HXML",
        extensions: &["hxml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
