use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3539317545: FileFormat = FileFormat {
    id: 3_539_317_545,
    source_type: SourceType::Iana,
    name: "atsc-rdt+json",
    extensions: &[],
    media_types: &["application/atsc-rdt+json"],
    internal_signatures: &[],
    related_formats: &[],
};
