use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3199675479: FileFormat = FileFormat {
    id: 3_199_675_479,
    source_type: SourceType::Iana,
    name: "DIT",
    extensions: &[],
    media_types: &["application/DIT"],
    signatures: &[],
    related_formats: &[],
};
