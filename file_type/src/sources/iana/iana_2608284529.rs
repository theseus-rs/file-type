use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2608284529: FileFormat = FileFormat {
    id: 2_608_284_529,
    source_type: SourceType::Iana,
    name: "yang-patch+json",
    extensions: &[],
    media_types: &["application/yang-patch+json"],
    signatures: &[],
    related_formats: &[],
};
