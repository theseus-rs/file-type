use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_703786695: FileFormat = FileFormat {
    id: 703_786_695,
    source_type: SourceType::Iana,
    name: "MELP600",
    extensions: &[],
    media_types: &["audio/MELP600"],
    internal_signatures: &[],
    related_formats: &[],
};
