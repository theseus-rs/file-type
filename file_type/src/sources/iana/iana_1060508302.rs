use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1060508302: FileFormat = FileFormat {
    id: 1_060_508_302,
    source_type: SourceType::Iana,
    name: "vp+jwt",
    extensions: &[],
    media_types: &["application/vp+jwt"],
    signatures: &[],
    related_formats: &[],
};
