use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1805342700: FileType = FileType {
    file_format: &FileFormat {
        id: 1_805_342_700,
        source_type: SourceType::Iana,
        name: "asyncapi+json",
        extensions: &[],
        media_types: &["application/asyncapi+json"],
        signatures: &[],
        related_formats: &[],
    },
};
