use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_86020751: FileType = FileType {
    file_format: &FileFormat {
        id: 86_020_751,
        source_type: SourceType::Iana,
        name: "PDX",
        extensions: &[],
        media_types: &["application/PDX"],
        signatures: &[],
        related_formats: &[],
    },
};
