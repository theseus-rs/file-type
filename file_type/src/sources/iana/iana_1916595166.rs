use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1916595166: FileFormat = FileFormat {
    id: 1_916_595_166,
    source_type: SourceType::Iana,
    name: "riscos",
    extensions: &[],
    media_types: &["application/riscos"],
    internal_signatures: &[],
    related_formats: &[],
};
