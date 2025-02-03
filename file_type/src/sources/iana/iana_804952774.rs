use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_804952774: FileFormat = FileFormat {
    id: 804_952_774,
    source_type: SourceType::Iana,
    name: "sipc",
    extensions: &[],
    media_types: &["application/sipc"],
    internal_signatures: &[],
    related_formats: &[],
};
