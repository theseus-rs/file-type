use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2645702580: FileFormat = FileFormat {
    id: 2_645_702_580,
    source_type: SourceType::Iana,
    name: "EmergencyCallData.DeviceInfo+xml",
    extensions: &[],
    media_types: &["application/EmergencyCallData.DeviceInfo+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
