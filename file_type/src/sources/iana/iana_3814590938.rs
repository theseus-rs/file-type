use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3814590938: FileFormat = FileFormat {
    id: 3_814_590_938,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.seal-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.seal-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
