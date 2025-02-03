use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4077824776: FileFormat = FileFormat {
    id: 4_077_824_776,
    source_type: SourceType::Iana,
    name: "vnd.commonspace",
    extensions: &[],
    media_types: &["application/vnd.commonspace"],
    internal_signatures: &[],
    related_formats: &[],
};
