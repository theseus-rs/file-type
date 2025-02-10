use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2071630442: FileType = FileType {
    file_format: &FileFormat {
        id: 2_071_630_442,
        source_type: SourceType::Iana,
        name: "vnd.hsl",
        extensions: &[],
        media_types: &["application/vnd.hsl"],
        signatures: &[],
        related_formats: &[],
    },
};
