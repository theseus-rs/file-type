use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3815365048: FileFormat = FileFormat {
    id: 3_815_365_048,
    source_type: SourceType::Iana,
    name: "EmergencyCallData.ServiceInfo+xml",
    extensions: &[],
    media_types: &["application/EmergencyCallData.ServiceInfo+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
