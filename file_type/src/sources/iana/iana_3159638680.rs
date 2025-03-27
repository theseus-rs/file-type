use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3159638680: FileType = FileType {
    file_format: &FileFormat {
        id: 3_159_638_680,
        source_type: SourceType::Iana,
        name: "sdf+json",
        extensions: &[],
        media_types: &["application/sdf+json"],
        signatures: &[],
        related_formats: &[],
    },
};
