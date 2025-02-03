use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4121617177: FileFormat = FileFormat {
    id: 4_121_617_177,
    source_type: SourceType::Iana,
    name: "im-iscomposing+xml",
    extensions: &[],
    media_types: &["application/im-iscomposing+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
