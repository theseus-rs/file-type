use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3679311052: FileFormat = FileFormat {
    id: 3_679_311_052,
    source_type: SourceType::Iana,
    name: "vnd.fly",
    extensions: &[],
    media_types: &["text/vnd.fly"],
    internal_signatures: &[],
    related_formats: &[],
};
