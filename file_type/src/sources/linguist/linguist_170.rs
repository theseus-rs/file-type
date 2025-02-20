use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_170: FileType = FileType {
    file_format: &FileFormat {
        id: 170,
        source_type: SourceType::Linguist,
        name: "Isabelle",
        extensions: &["thy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
