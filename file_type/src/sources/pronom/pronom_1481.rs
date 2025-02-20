use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1481: FileType = FileType {
    file_format: &FileFormat {
        id: 1_481,
        source_type: SourceType::Pronom,
        name: "Thumbs DB file",
        extensions: &["db"],
        media_types: &["application/vnd.microsoft.windows.thumbnail-cache"],
        signatures: &[],
        related_formats: &[],
    },
};
