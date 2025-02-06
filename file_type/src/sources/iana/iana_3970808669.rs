use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3970808669: FileFormat = FileFormat {
    id: 3_970_808_669,
    source_type: SourceType::Iana,
    name: "pkix-crl",
    extensions: &[],
    media_types: &["application/pkix-crl"],
    signatures: &[],
    related_formats: &[],
};
