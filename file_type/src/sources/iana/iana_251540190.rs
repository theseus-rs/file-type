use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_251540190: FileFormat = FileFormat {
    id: 251_540_190,
    source_type: SourceType::Iana,
    name: "vnd.intercon.formnet",
    extensions: &[],
    media_types: &["application/vnd.intercon.formnet"],
    signatures: &[],
    related_formats: &[],
};
