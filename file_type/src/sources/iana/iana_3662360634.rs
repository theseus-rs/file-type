use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3662360634: FileFormat = FileFormat {
    id: 3_662_360_634,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
