use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1048917476: FileFormat = FileFormat {
    id: 1_048_917_476,
    source_type: SourceType::Iana,
    name: "xcap-diff+xml",
    extensions: &[],
    media_types: &["application/xcap-diff+xml"],
    signatures: &[],
    related_formats: &[],
};
