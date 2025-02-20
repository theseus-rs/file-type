use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_650321585: FileType = FileType {
    file_format: &FileFormat {
        id: 650_321_585,
        source_type: SourceType::Iana,
        name: "vnd.cncf.helm.config.v1+json",
        extensions: &[],
        media_types: &["application/vnd.cncf.helm.config.v1+json"],
        signatures: &[],
        related_formats: &[],
    },
};
