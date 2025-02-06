use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_763411276: FileFormat = FileFormat {
    id: 763_411_276,
    source_type: SourceType::Iana,
    name: "vnd.ms-PrintDeviceCapabilities+xml",
    extensions: &[],
    media_types: &["application/vnd.ms-PrintDeviceCapabilities+xml"],
    signatures: &[],
    related_formats: &[],
};
