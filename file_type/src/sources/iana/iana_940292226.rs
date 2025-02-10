use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_940292226: FileType = FileType {
    file_format: &FileFormat {
        id: 940_292_226,
        source_type: SourceType::Iana,
        name: "vnd.3gpp-v2x-local-service-information",
        extensions: &[],
        media_types: &["application/vnd.3gpp-v2x-local-service-information"],
        signatures: &[],
        related_formats: &[],
    },
};
