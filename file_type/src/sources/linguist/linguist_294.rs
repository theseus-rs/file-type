use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_294: FileType = FileType {
    file_format: &FileFormat {
        id: 294,
        source_type: SourceType::Linguist,
        name: "Processing",
        extensions: &["pde"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
