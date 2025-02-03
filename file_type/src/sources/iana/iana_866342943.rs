use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_866342943: FileFormat = FileFormat {
    id: 866_342_943,
    source_type: SourceType::Iana,
    name: "xcap-error+xml",
    extensions: &[],
    media_types: &["application/xcap-error+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
