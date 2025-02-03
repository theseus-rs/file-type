use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2814014795: FileFormat = FileFormat {
    id: 2_814_014_795,
    source_type: SourceType::Iana,
    name: "jwk-set+json",
    extensions: &[],
    media_types: &["application/jwk-set+json"],
    internal_signatures: &[],
    related_formats: &[],
};
