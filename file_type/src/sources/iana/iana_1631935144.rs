use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1631935144: FileFormat = FileFormat {
    id: 1_631_935_144,
    source_type: SourceType::Iana,
    name: "vnd.freelog.comic",
    extensions: &[],
    media_types: &["application/vnd.freelog.comic"],
    internal_signatures: &[],
    related_formats: &[],
};
