use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_396279391: FileFormat = FileFormat {
    id: 396_279_391,
    source_type: SourceType::Iana,
    name: "vnd.collection.next+json",
    extensions: &[],
    media_types: &["application/vnd.collection.next+json"],
    signatures: &[],
    related_formats: &[],
};
