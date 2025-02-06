use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3171097430: FileFormat = FileFormat {
    id: 3_171_097_430,
    source_type: SourceType::Iana,
    name: "missing-blocks+cbor-seq",
    extensions: &[],
    media_types: &["application/missing-blocks+cbor-seq"],
    signatures: &[],
    related_formats: &[],
};
