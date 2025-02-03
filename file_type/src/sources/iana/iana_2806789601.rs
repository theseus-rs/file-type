use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2806789601: FileFormat = FileFormat {
    id: 2_806_789_601,
    source_type: SourceType::Iana,
    name: "vnd.poc.group-advertisement+xml",
    extensions: &[],
    media_types: &["application/vnd.poc.group-advertisement+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
