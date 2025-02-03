use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2093684979: FileFormat = FileFormat {
    id: 2_093_684_979,
    source_type: SourceType::Iana,
    name: "vnd.avistar+xml",
    extensions: &[],
    media_types: &["application/vnd.avistar+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
