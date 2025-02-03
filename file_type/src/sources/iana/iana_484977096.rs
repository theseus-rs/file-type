use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_484977096: FileFormat = FileFormat {
    id: 484_977_096,
    source_type: SourceType::Iana,
    name: "dns",
    extensions: &[],
    media_types: &["text/dns"],
    internal_signatures: &[],
    related_formats: &[],
};
