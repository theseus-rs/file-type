use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_479039817: FileType = FileType {
    file_format: &FileFormat {
        id: 479_039_817,
        source_type: SourceType::Linguist,
        name: "HTML+Razor",
        extensions: &["cshtml", "razor"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
