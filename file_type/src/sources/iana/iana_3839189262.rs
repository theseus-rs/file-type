use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3839189262: FileFormat = FileFormat {
    id: 3_839_189_262,
    source_type: SourceType::Iana,
    name: "vnd.onepagertat",
    extensions: &[],
    media_types: &["application/vnd.onepagertat"],
    internal_signatures: &[],
    related_formats: &[],
};
