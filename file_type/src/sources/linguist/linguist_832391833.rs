use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_832391833: FileType = FileType {
    file_format: &FileFormat {
        id: 832_391_833,
        source_type: SourceType::Linguist,
        name: "Portugol",
        extensions: &["por"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
