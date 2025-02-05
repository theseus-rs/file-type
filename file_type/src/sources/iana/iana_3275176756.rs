use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3275176756: FileFormat = FileFormat {
    id: 3_275_176_756,
    source_type: SourceType::Iana,
    name: "vnd.nokia.radio-presets",
    extensions: &[],
    media_types: &["application/vnd.nokia.radio-presets"],
    signatures: &[],
    related_formats: &[],
};
