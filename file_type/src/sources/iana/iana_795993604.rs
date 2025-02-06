use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_795993604: FileFormat = FileFormat {
    id: 795_993_604,
    source_type: SourceType::Iana,
    name: "vnd.latex-z",
    extensions: &[],
    media_types: &["text/vnd.latex-z"],
    signatures: &[],
    related_formats: &[],
};
