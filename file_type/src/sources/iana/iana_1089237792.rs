use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1089237792: FileType = FileType {
    file_format: &FileFormat {
        id: 1_089_237_792,
        source_type: SourceType::Iana,
        name: "postscript",
        extensions: &[],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
