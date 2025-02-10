use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_195: FileType = FileType {
    file_format: &FileFormat {
        id: 195,
        source_type: SourceType::Linguist,
        name: "Lasso",
        extensions: &["las", "lasso", "lasso8", "lasso9"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
