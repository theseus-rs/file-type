use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_849097741: FileFormat = FileFormat {
    id: 849_097_741,
    source_type: SourceType::Iana,
    name: "vnd.omads-file+xml",
    extensions: &[],
    media_types: &["application/vnd.omads-file+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
