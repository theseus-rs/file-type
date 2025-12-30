use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_242815331: FileType = FileType {
    file_format: &FileFormat {
        id: 242_815_331,
        source_type: SourceType::Iana,
        name: "3gpp-mbs-object-manifest+json",
        extensions: &[],
        media_types: &["application/3gpp-mbs-object-manifest+json"],
        signatures: &[],
        related_formats: &[],
    },
};
