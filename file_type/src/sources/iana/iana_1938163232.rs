use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1938163232: FileFormat = FileFormat {
    id: 1_938_163_232,
    source_type: SourceType::Iana,
    name: "vnd.zul",
    extensions: &[],
    media_types: &["application/vnd.zul"],
    internal_signatures: &[],
    related_formats: &[],
};
