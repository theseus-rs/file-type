use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_129: FileType = FileType {
    file_format: &FileFormat {
        id: 129,
        source_type: SourceType::Linguist,
        name: "Gettext Catalog",
        extensions: &["po", "pot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
