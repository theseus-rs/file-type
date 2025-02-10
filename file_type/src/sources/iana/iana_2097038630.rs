use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2097038630: FileType = FileType {
    file_format: &FileFormat {
        id: 2_097_038_630,
        source_type: SourceType::Iana,
        name: "markdown",
        extensions: &[],
        media_types: &["text/markdown"],
        signatures: &[],
        related_formats: &[],
    },
};
