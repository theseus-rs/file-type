use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2727265353: FileFormat = FileFormat {
    id: 2_727_265_353,
    source_type: SourceType::Iana,
    name: "alto-costmap+json",
    extensions: &[],
    media_types: &["application/alto-costmap+json"],
    internal_signatures: &[],
    related_formats: &[],
};
