use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_235277043: FileType = FileType {
    file_format: &FileFormat {
        id: 235_277_043,
        source_type: SourceType::Linguist,
        name: "Jac",
        extensions: &["jac"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
