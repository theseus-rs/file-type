use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3102304575: FileFormat = FileFormat {
    id: 3_102_304_575,
    source_type: SourceType::Iana,
    name: "EmergencyCallData.ProviderInfo+xml",
    extensions: &[],
    media_types: &["application/EmergencyCallData.ProviderInfo+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
