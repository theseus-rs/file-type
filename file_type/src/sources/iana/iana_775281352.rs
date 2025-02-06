use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_775281352: FileFormat = FileFormat {
    id: 775_281_352,
    source_type: SourceType::Iana,
    name: "vnd.3gpp-prose-pc3a+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp-prose-pc3a+xml"],
    signatures: &[],
    related_formats: &[],
};
