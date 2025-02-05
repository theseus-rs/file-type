use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4149901353: FileFormat = FileFormat {
    id: 4_149_901_353,
    source_type: SourceType::Iana,
    name: "vnd.omads-email+xml",
    extensions: &[],
    media_types: &["application/vnd.omads-email+xml"],
    signatures: &[],
    related_formats: &[],
};
