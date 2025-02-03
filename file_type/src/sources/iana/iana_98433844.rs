use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_98433844: FileFormat = FileFormat {
    id: 98_433_844,
    source_type: SourceType::Iana,
    name: "mbms-deregister+xml",
    extensions: &[],
    media_types: &["application/mbms-deregister+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
