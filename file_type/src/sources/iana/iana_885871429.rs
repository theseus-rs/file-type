use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_885871429: FileFormat = FileFormat {
    id: 885_871_429,
    source_type: SourceType::Iana,
    name: "vnd.etsi.sci+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.sci+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
