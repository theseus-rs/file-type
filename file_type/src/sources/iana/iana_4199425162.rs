use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4199425162: FileFormat = FileFormat {
    id: 4_199_425_162,
    source_type: SourceType::Iana,
    name: "vnd.ms-office.activeX+xml",
    extensions: &[],
    media_types: &["application/vnd.ms-office.activeX+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
