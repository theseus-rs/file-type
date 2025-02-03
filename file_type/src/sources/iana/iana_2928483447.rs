use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2928483447: FileFormat = FileFormat {
    id: 2_928_483_447,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
