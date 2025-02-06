use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2179273523: FileFormat = FileFormat {
    id: 2_179_273_523,
    source_type: SourceType::Iana,
    name: "vnd.hal+xml",
    extensions: &[],
    media_types: &["application/vnd.hal+xml"],
    signatures: &[],
    related_formats: &[],
};
