use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_191439868: FileFormat = FileFormat {
    id: 191_439_868,
    source_type: SourceType::Iana,
    name: "cql",
    extensions: &[],
    media_types: &["text/cql"],
    internal_signatures: &[],
    related_formats: &[],
};
