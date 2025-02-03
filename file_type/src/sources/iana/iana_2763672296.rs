use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2763672296: FileFormat = FileFormat {
    id: 2_763_672_296,
    source_type: SourceType::Iana,
    name: "call-completion",
    extensions: &[],
    media_types: &["application/call-completion"],
    internal_signatures: &[],
    related_formats: &[],
};
