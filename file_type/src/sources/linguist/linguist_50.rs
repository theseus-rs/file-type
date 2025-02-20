use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_50: FileType = FileType {
    file_format: &FileFormat {
        id: 50,
        source_type: SourceType::Linguist,
        name: "CSS",
        extensions: &["css"],
        media_types: &["text/css"],
        signatures: &[],
        related_formats: &[],
    },
};
