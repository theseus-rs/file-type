use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2681066647: FileType = FileType {
    file_format: &FileFormat {
        id: 2_681_066_647,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-data-delivery-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-data-delivery-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
