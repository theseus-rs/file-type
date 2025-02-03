use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_673108048: FileFormat = FileFormat {
    id: 673_108_048,
    source_type: SourceType::Iana,
    name: "cdmi-domain",
    extensions: &[],
    media_types: &["application/cdmi-domain"],
    internal_signatures: &[],
    related_formats: &[],
};
