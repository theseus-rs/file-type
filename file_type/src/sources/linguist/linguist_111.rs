use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
