use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_561111317: FileFormat = FileFormat {
    id: 561_111_317,
    source_type: SourceType::Iana,
    name: "vnd.3gpp-prose-pc3ch+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp-prose-pc3ch+xml"],
    signatures: &[],
    related_formats: &[],
};
