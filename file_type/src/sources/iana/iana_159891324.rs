use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_159891324: FileFormat = FileFormat {
    id: 159_891_324,
    source_type: SourceType::Iana,
    name: "msc-ivr+xml",
    extensions: &[],
    media_types: &["application/msc-ivr+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
