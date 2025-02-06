use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2596127697: FileFormat = FileFormat {
    id: 2_596_127_697,
    source_type: SourceType::Iana,
    name: "vnd.rapid",
    extensions: &[],
    media_types: &["application/vnd.rapid"],
    signatures: &[],
    related_formats: &[],
};
