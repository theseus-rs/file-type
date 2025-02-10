use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1959: FileType = FileType {
    file_format: &FileFormat {
        id: 1_959,
        source_type: SourceType::Pronom,
        name: "Markdown",
        extensions: &["md", "markdown"],
        media_types: &["text/markdown"],
        signatures: &[],
        related_formats: &[],
    },
};
