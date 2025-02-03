use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_116508161: FileFormat = FileFormat {
    id: 116_508_161,
    source_type: SourceType::Iana,
    name: "mosskey-request",
    extensions: &[],
    media_types: &["application/mosskey-request"],
    internal_signatures: &[],
    related_formats: &[],
};
