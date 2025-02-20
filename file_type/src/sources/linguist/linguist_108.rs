use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_108: FileType = FileType {
    file_format: &FileFormat {
        id: 108,
        source_type: SourceType::Linguist,
        name: "Factor",
        extensions: &["factor"],
        media_types: &["text/x-factor"],
        signatures: &[],
        related_formats: &[],
    },
};
