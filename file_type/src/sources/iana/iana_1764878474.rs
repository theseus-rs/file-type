use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1764878474: FileFormat = FileFormat {
    id: 1_764_878_474,
    source_type: SourceType::Iana,
    name: "vnd.dwf",
    extensions: &[],
    media_types: &["model/vnd.dwf"],
    internal_signatures: &[],
    related_formats: &[],
};
