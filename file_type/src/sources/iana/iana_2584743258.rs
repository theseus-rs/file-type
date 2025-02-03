use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2584743258: FileFormat = FileFormat {
    id: 2_584_743_258,
    source_type: SourceType::Iana,
    name: "AML",
    extensions: &[],
    media_types: &["application/AML"],
    internal_signatures: &[],
    related_formats: &[],
};
