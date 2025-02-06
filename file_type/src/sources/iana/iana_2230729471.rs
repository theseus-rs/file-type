use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2230729471: FileFormat = FileFormat {
    id: 2_230_729_471,
    source_type: SourceType::Iana,
    name: "vnd.grafeq",
    extensions: &[],
    media_types: &["application/vnd.grafeq"],
    signatures: &[],
    related_formats: &[],
};
