use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_377204539: FileType = FileType {
    file_format: &FileFormat {
        id: 377_204_539,
        source_type: SourceType::Linguist,
        name: "Linear Programming",
        extensions: &["lp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
