use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1632936163: FileFormat = FileFormat {
    id: 1_632_936_163,
    source_type: SourceType::Iana,
    name: "tamp-apex-update",
    extensions: &[],
    media_types: &["application/tamp-apex-update"],
    signatures: &[],
    related_formats: &[],
};
