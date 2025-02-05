use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3061890077: FileFormat = FileFormat {
    id: 3_061_890_077,
    source_type: SourceType::Iana,
    name: "vnd.xarin.cpj",
    extensions: &[],
    media_types: &["application/vnd.xarin.cpj"],
    signatures: &[],
    related_formats: &[],
};
