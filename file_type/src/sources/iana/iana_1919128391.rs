use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1919128391: FileFormat = FileFormat {
    id: 1_919_128_391,
    source_type: SourceType::Iana,
    name: "vnd.microsoft.portable-executable",
    extensions: &[],
    media_types: &["application/vnd.microsoft.portable-executable"],
    signatures: &[],
    related_formats: &[],
};
