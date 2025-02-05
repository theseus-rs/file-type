use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1628556655: FileFormat = FileFormat {
    id: 1_628_556_655,
    source_type: SourceType::Iana,
    name: "vnd.hyperdrive+json",
    extensions: &[],
    media_types: &["application/vnd.hyperdrive+json"],
    signatures: &[],
    related_formats: &[],
};
