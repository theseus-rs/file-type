use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_875961514: FileFormat = FileFormat {
    id: 875_961_514,
    source_type: SourceType::Iana,
    name: "application/jwk-set+jwt",
    extensions: &[],
    media_types: &["application/jwk-set+jwt"],
    signatures: &[],
    related_formats: &[],
};
