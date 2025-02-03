use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1087953579: FileFormat = FileFormat {
    id: 1_087_953_579,
    source_type: SourceType::Iana,
    name: "vnd.3gpp-prose+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp-prose+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
