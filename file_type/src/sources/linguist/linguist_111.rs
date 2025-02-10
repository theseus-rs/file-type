use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_111: FileType = FileType {
    file_format: &FileFormat {
        id: 111,
        source_type: SourceType::Linguist,
        name: "Filebench WML",
        extensions: &["f"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
