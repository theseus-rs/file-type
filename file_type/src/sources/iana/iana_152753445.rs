use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_152753445: FileFormat = FileFormat {
    id: 152_753_445,
    source_type: SourceType::Iana,
    name: "SGML",
    extensions: &[],
    media_types: &["text/SGML"],
    internal_signatures: &[],
    related_formats: &[],
};
