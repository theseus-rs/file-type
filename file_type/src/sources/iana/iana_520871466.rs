use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_520871466: FileFormat = FileFormat {
    id: 520_871_466,
    source_type: SourceType::Iana,
    name: "pkix-cert",
    extensions: &[],
    media_types: &["application/pkix-cert"],
    signatures: &[],
    related_formats: &[],
};
