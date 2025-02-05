use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2509110589: FileFormat = FileFormat {
    id: 2_509_110_589,
    source_type: SourceType::Iana,
    name: "jrd+json",
    extensions: &[],
    media_types: &["application/jrd+json"],
    signatures: &[],
    related_formats: &[],
};
