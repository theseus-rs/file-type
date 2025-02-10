use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_741089846: FileType = FileType {
    file_format: &FileFormat {
        id: 741_089_846,
        source_type: SourceType::Iana,
        name: "fdt+xml",
        extensions: &[],
        media_types: &["application/fdt+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
