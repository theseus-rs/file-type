use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2812998300: FileFormat = FileFormat {
    id: 2_812_998_300,
    source_type: SourceType::Iana,
    name: "x-x509-ca-cert",
    extensions: &[],
    media_types: &["application/x-x509-ca-cert"],
    signatures: &[],
    related_formats: &[],
};
