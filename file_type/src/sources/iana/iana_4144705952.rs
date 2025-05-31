use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4144705952: FileType = FileType {
    file_format: &FileFormat {
        id: 4_144_705_952,
        source_type: SourceType::Iana,
        name: "sd-jwt",
        extensions: &[],
        media_types: &["application/sd-jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
