use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3164374345: FileFormat = FileFormat {
    id: 3_164_374_345,
    source_type: SourceType::Iana,
    name: "vnd.3gpp-prose-pc3ach+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp-prose-pc3ach+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
