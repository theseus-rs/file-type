use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_509235866: FileFormat = FileFormat {
    id: 509_235_866,
    source_type: SourceType::Iana,
    name: "vnd.ms-wpl",
    extensions: &[],
    media_types: &["application/vnd.ms-wpl"],
    signatures: &[],
    related_formats: &[],
};
