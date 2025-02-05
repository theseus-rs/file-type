use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2927980230: FileFormat = FileFormat {
    id: 2_927_980_230,
    source_type: SourceType::Iana,
    name: "vnd.oma.group-usage-list+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.group-usage-list+xml"],
    signatures: &[],
    related_formats: &[],
};
