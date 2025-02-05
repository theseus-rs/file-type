use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3813641127: FileFormat = FileFormat {
    id: 3_813_641_127,
    source_type: SourceType::Iana,
    name: "vnd.3gpp-prose-pc8+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp-prose-pc8+xml"],
    signatures: &[],
    related_formats: &[],
};
