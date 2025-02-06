use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_940292226: FileFormat = FileFormat {
    id: 940_292_226,
    source_type: SourceType::Iana,
    name: "vnd.3gpp-v2x-local-service-information",
    extensions: &[],
    media_types: &["application/vnd.3gpp-v2x-local-service-information"],
    signatures: &[],
    related_formats: &[],
};
