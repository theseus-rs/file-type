use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1719636634: FileFormat = FileFormat {
    id: 1_719_636_634,
    source_type: SourceType::Iana,
    name: "alto-cdni+json",
    extensions: &[],
    media_types: &["application/alto-cdni+json"],
    signatures: &[],
    related_formats: &[],
};
