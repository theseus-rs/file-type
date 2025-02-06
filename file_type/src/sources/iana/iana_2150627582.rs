use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2150627582: FileFormat = FileFormat {
    id: 2_150_627_582,
    source_type: SourceType::Iana,
    name: "resource-lists+xml",
    extensions: &[],
    media_types: &["application/resource-lists+xml"],
    signatures: &[],
    related_formats: &[],
};
