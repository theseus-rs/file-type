use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_337: FileType = FileType {
    file_format: &FileFormat {
        id: 337,
        source_type: SourceType::Linguist,
        name: "SVG",
        extensions: &["svg"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
