use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3251275829: FileFormat = FileFormat {
    id: 3_251_275_829,
    source_type: SourceType::Iana,
    name: "vnd.FloGraphIt",
    extensions: &[],
    media_types: &["application/vnd.FloGraphIt"],
    internal_signatures: &[],
    related_formats: &[],
};
