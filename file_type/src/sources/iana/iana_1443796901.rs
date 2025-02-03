use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1443796901: FileFormat = FileFormat {
    id: 1_443_796_901,
    source_type: SourceType::Iana,
    name: "vnd.wfa.wsc",
    extensions: &[],
    media_types: &["application/vnd.wfa.wsc"],
    internal_signatures: &[],
    related_formats: &[],
};
