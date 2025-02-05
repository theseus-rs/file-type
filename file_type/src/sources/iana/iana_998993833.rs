use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_998993833: FileFormat = FileFormat {
    id: 998_993_833,
    source_type: SourceType::Iana,
    name: "vnd.a",
    extensions: &[],
    media_types: &["text/vnd.a"],
    signatures: &[],
    related_formats: &[],
};
