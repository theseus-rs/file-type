use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3667895505: FileFormat = FileFormat {
    id: 3_667_895_505,
    source_type: SourceType::Iana,
    name: "tamp-status-query",
    extensions: &[],
    media_types: &["application/tamp-status-query"],
    signatures: &[],
    related_formats: &[],
};
