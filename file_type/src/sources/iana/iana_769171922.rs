use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_769171922: FileFormat = FileFormat {
    id: 769_171_922,
    source_type: SourceType::Iana,
    name: "vnd.ascii-art",
    extensions: &[],
    media_types: &["text/vnd.ascii-art"],
    internal_signatures: &[],
    related_formats: &[],
};
