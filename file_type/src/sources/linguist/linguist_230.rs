use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_230: FileType = FileType {
    file_format: &FileFormat {
        id: 230,
        source_type: SourceType::Linguist,
        name: "Metal",
        extensions: &["metal"],
        media_types: &["text/x-c++src"],
        signatures: &[],
        related_formats: &[],
    },
};
