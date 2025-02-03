use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_268959593: FileFormat = FileFormat {
    id: 268_959_593,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcdata-regroup+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcdata-regroup+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
